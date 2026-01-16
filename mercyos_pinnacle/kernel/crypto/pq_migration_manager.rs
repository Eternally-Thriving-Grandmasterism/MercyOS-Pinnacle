//! PQ Migration Manager — Hybrid Multi-Mode Agility Kernel Beacon
//! Aligned with NIST migration guidelines: inventory → agility → hybrid transition → pure PQ.
//! Uses pqcrypto-traits for unified KEM interface.
//! Multi-family ready: current lattice (Kyber ≈ ML-KEM), extensible to hash/code-based.

use pqcrypto_kyber::kyber1024::*;
use pqcrypto_traits::kem::{
    Ciphertext as KemCiphertext, PublicKey as KemPublicKey, SecretKey as KemSecretKey,
    SharedSecret as KemSharedSecret,
};
use rand_core::{OsRng, RngCore};
use x25519_dalek::{EphemeralSecret, PublicKey as X25519Public, StaticSecret};

pub enum MigrationMode {
    Legacy,       // Classical only (X25519) — for backward compatibility
    Hybrid,       // X25519 + Kyber1024 (recommended NIST transition)
    QuantumSafe,  // Pure PQ (Kyber1024 ≈ ML-KEM-1024)
}

pub struct HybridPublicKey {
    pub classical: Option<X25519Public>,
    pub pq: KemPublicKey,
}

pub struct HybridSecretKey {
    pub classical: Option<StaticSecret>,
    pub pq: KemSecretKey,
}

pub struct HybridCiphertext {
    pub classical_ephem: Option<X25519Public>,
    pub pq_ct: Ciphertext,
}

pub struct PQMigrator;

impl PQMigrator {
    pub fn keypair(mode: MigrationMode) -> (HybridPublicKey, HybridSecretKey) {
        let (pq_pk, pq_sk) = keypair();

        match mode {
            MigrationMode::Legacy | MigrationMode::Hybrid => {
                let classical_sk = StaticSecret::random_from_rng(OsRng);
                let classical_pk = X25519Public::from(&classical_sk);
                (
                    HybridPublicKey {
                        classical: Some(classical_pk),
                        pq: pq_pk,
                    },
                    HybridSecretKey {
                        classical: Some(classical_sk),
                        pq: pq_sk,
                    },
                )
            }
            MigrationMode::QuantumSafe => (
                HybridPublicKey {
                    classical: None,
                    pq: pq_pk,
                },
                HybridSecretKey {
                    classical: None,
                    pq: pq_sk,
                },
            ),
        }
    }

    pub fn encapsulate(
        pk: &HybridPublicKey,
        mode: MigrationMode,
    ) -> Result<(HybridCiphertext, Vec<u8>), &'static str> {
        let mut shared = Vec::with_capacity(64);

        match mode {
            MigrationMode::Legacy | MigrationMode::Hybrid => {
                if let Some(classical_pk) = &pk.classical {
                    let ephem = EphemeralSecret::random_from_rng(OsRng);
                    let ephem_pk = X25519Public::from(&ephem);
                    let classical_ss = ephem.diffie_hellman(classical_pk);
                    shared.extend_from_slice(classical_ss.as_bytes());
                    let (pq_ct, pq_ss) = encapsulate(&pk.pq);
                    Ok((
                        HybridCiphertext {
                            classical_ephem: Some(ephem_pk),
                            pq_ct,
                        },
                        [shared.as_slice(), pq_ss.as_bytes()].concat(),
                    ))
                } else {
                    Err("Classical key missing for selected mode")
                }
            }
            MigrationMode::QuantumSafe => {
                let (pq_ct, pq_ss) = encapsulate(&pk.pq);
                Ok((
                    HybridCiphertext {
                        classical_ephem: None,
                        pq_ct,
                    },
                    pq_ss.as_bytes().to_vec(),
                ))
            }
        }
    }

    pub fn decapsulate(
        ct: &HybridCiphertext,
        sk: &HybridSecretKey,
        mode: MigrationMode,
    ) -> Result<Vec<u8>, &'static str> {
        let mut shared = Vec::with_capacity(64);

        match mode {
            MigrationMode::Legacy | MigrationMode::Hybrid => {
                if let (Some(ephem_pk), Some(classical_sk)) = (&ct.classical_ephem, &sk.classical) {
                    let classical_ss = classical_sk.diffie_hellman(ephem_pk);
                    shared.extend_from_slice(classical_ss.as_bytes());
                }
                let pq_ss = decapsulate(&ct.pq_ct, &sk.pq);
                shared.extend_from_slice(pq_ss.as_bytes());
                Ok(shared)
            }
            MigrationMode::QuantumSafe => Ok(decapsulate(&ct.pq_ct, &sk.pq).as_bytes().to_vec()),
        }
    }
}
