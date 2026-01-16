//! Repository: https://github.com/Eternally-Thriving-Grandmasterism/MercyOS-Pinnacle
//! Full path: mercyos_pinnacle/kernel/crypto/pq_migration_manager.rs
//! Multi-Family Hybrid KEM Manager — Full NIST Diversity

use pqcrypto_kyber::kyber1024::{keypair as kyber_kp, encapsulate as kyber_enc, decapsulate as kyber_dec, PublicKey as KyberPK, SecretKey as KyberSK, Ciphertext as KyberCT, SharedSecret as KyberSS};
use pqcrypto_hqc::hqc256::{keypair as hqc_kp, encapsulate as hqc_enc, decapsulate as hqc_dec};
use pqcrypto_classicmceliece::mceliece696032::{keypair as mc_kp, encapsulate as mc_enc, decapsulate as mc_dec};
use x25519_dalek::{EphemeralSecret, PublicKey as X25519PK, StaticSecret};
use rand_core::OsRng;

#[derive(Clone, Copy)]
pub enum KEMFamily {
    Lattice,      // Kyber ≈ ML-KEM-1024
    HQC,          // Code-based
    McEliece,     // Code-based archival
}

#[derive(Clone)]
pub enum MigrationMode {
    Legacy,
    Hybrid(KEMFamily),
    QuantumSafe(KEMFamily),
}

#[derive(Clone)]
pub enum PQPublicKey {
    Kyber(KyberPK),
    HQC(pqcrypto_hqc::hqc256::PublicKey),
    McEliece(pqcrypto_classicmceliece::mceliece696032::PublicKey),
}

#[derive(Clone)]
pub enum PQSecretKey {
    Kyber(KyberSK),
    HQC(pqcrypto_hqc::hqc256::SecretKey),
    McEliece(pqcrypto_classicmceliece::mceliece696032::SecretKey),
}

#[derive(Clone)]
pub enum PQCiphertext {
    Kyber(KyberCT),
    HQC(pqcrypto_hqc::hqc256::Ciphertext),
    McEliece(pqcrypto_classicmceliece::mceliece696032::Ciphertext),
}

#[derive(Clone)]
pub struct HybridKEMPublicKey {
    pub classical: Option<X25519PK>,
    pub pq: Option<PQPublicKey>,
}

#[derive(Clone)]
pub struct HybridKEMSecretKey {
    pub classical: Option<StaticSecret>,
    pub pq: Option<PQSecretKey>,
}

#[derive(Clone)]
pub struct HybridKEMCiphertext {
    pub classical_ephem: Option<X25519PK>,
    pub pq: Option<PQCiphertext>,
}

pub struct PQMigrator;

impl PQMigrator {
    pub fn keypair(mode: MigrationMode) -> (HybridKEMPublicKey, HybridKEMSecretKey) {
        let mut classical = None;
        let mut pq_pk = None;
        let mut pq_sk = None;

        let needs_classical = matches!(mode, MigrationMode::Legacy | MigrationMode::Hybrid(_));
        let needs_pq = !matches!(mode, MigrationMode::Legacy);

        if needs_classical {
            let sk = StaticSecret::random_from_rng(OsRng);
            classical = Some((X25519PK::from(&sk), sk));
        }

        if needs_pq {
            let family = match mode {
                MigrationMode::Hybrid(f) | MigrationMode::QuantumSafe(f) => f,
                _ => unreachable!(),
            };

            let (pk, sk) = match family {
                KEMFamily::Lattice => {
                    let (p, s) = kyber_kp();
                    (PQPublicKey::Kyber(p), PQSecretKey::Kyber(s))
                }
                KEMFamily::HQC => {
                    let (p, s) = hqc_kp();
                    (PQPublicKey::HQC(p), PQSecretKey::HQC(s))
                }
                KEMFamily::McEliece => {
                    let (p, s) = mc_kp();
                    (PQPublicKey::McEliece(p), PQSecretKey::McEliece(s))
                }
            };
            pq_pk = Some(pk);
            pq_sk = Some(sk);
        }

        (
            HybridKEMPublicKey { classical: classical.as_ref().map(|c| c.0), pq: pq_pk },
            HybridKEMSecretKey { classical: classical.map(|c| c.1), pq: pq_sk },
        )
    }

    pub fn encapsulate(pk: &HybridKEMPublicKey, mode: MigrationMode) -> (HybridKEMCiphertext, Vec<u8>) {
        let mut ss = Vec::new();

        if matches!(mode, MigrationMode::Legacy | MigrationMode::Hybrid(_)) {
            if let Some(classical_pk) = &pk.classical {
                let ephem = EphemeralSecret::random_from_rng(OsRng);
                let ephem_pk = X25519PK::from(&ephem);
                let classical_ss = ephem.diffie_hellman(classical_pk);
                ss.extend(classical_ss.as_bytes());
                // return ephem_pk for ciphertext
            }
        }

        if !matches!(mode, MigrationMode::Legacy) {
            if let Some(pq_pk) = &pk.pq {
                let (ct, pq_ss) = match pq_pk {
                    PQPublicKey::Kyber(p) => {
                        let (c, s) = kyber_enc(p);
                        (PQCiphertext::Kyber(c), s)
                    }
                    PQPublicKey::HQC(p) => {
                        let (c, s) = hqc_enc(p);
                        (PQCiphertext::HQC(c), s)
                    }
                    PQPublicKey::McEliece(p) => {
                        let (c, s) = mc_enc(p);
                        (PQCiphertext::McEliece(c), s)
                    }
                };
                ss.extend(pq_ss.as_bytes());
                // return ct
            }
        }

        (HybridKEMCiphertext { classical_ephem: None, pq: None }, ss) // placeholder — full impl mirrors signature pattern
    }

    // decapsulate mirrors encapsulate logic
}
