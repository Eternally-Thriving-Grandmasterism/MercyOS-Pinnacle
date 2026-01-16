//! Repository: https://github.com/Eternally-Thriving-Grandmasterism/MercyOS-Pinnacle
//! Full path: mercyos_pinnacle/kernel/crypto/pq_hybrid_signature_manager.rs
//! PQ Hybrid Multi-Family NIST Signature Manager — Full Diversity Agility Beacon
//! Supports Legacy (Ed25519), Hybrid (Ed25519 + PQ family), Pure (PQ family only)
//! Families: Dilithium (ML-DSA), Sphincs+ (SLH-DSA), Falcon (FN-DSA)

use pqcrypto_dilithium::dilithium5::{
    keypair as dilithium_keypair, sign as dilithium_sign, open as dilithium_open,
    PublicKey as DilithiumPublic, SecretKey as DilithiumSecret, SignedMessage as DilithiumSigned,
};
use pqcrypto_sphincsplus::sphincsshake256frobust::{
    keypair as sphincs_keypair, sign as sphincs_sign, open as sphincs_open,
    PublicKey as SphincsPublic, SecretKey as SphincsSecret, SignedMessage as SphincsSigned,
};
use pqcrypto_falcon::falcon1024::{
    keypair as falcon_keypair, sign as falcon_sign, open as falcon_open,
    PublicKey as FalconPublic, SecretKey as FalconSecret, SignedMessage as FalconSigned,
};
use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};
use rand_core::OsRng;

#[derive(Clone, Copy)]
pub enum PQFamily {
    Dilithium,
    Sphincs,
    Falcon,
}

#[derive(Clone)]
pub enum SignatureMode {
    Legacy,
    Hybrid(PQFamily),
    Pure(PQFamily),
}

#[derive(Clone)]
pub enum PQSigPublicKey {
    Dilithium(DilithiumPublic),
    Sphincs(SphincsPublic),
    Falcon(FalconPublic),
}

#[derive(Clone)]
pub enum PQSigSecretKey {
    Dilithium(DilithiumSecret),
    Sphincs(SphincsSecret),
    Falcon(FalconSecret),
}

#[derive(Clone)]
pub enum PQDetachedSignature {
    Dilithium(Vec<u8>),
    Sphincs(Vec<u8>),
    Falcon(Vec<u8>),
}

#[derive(Clone)]
pub struct HybridSigPublicKey {
    pub classical: Option<VerifyingKey>,
    pub pq: Option<PQSigPublicKey>,
}

#[derive(Clone)]
pub struct HybridSigSecretKey {
    pub classical: Option<SigningKey>,
    pub pq: Option<PQSigSecretKey>,
}

#[derive(Clone)]
pub struct HybridDetachedSignature {
    pub classical: Option<Signature>,
    pub pq: Option<PQDetachedSignature>,
}

pub struct PQSignatureMigrator;

impl PQSignatureMigrator {
    pub fn keypair(mode: SignatureMode) -> (HybridSigPublicKey, HybridSigSecretKey) {
        let mut classical_sk: Option<SigningKey> = None;
        let mut pq_pk: Option<PQSigPublicKey> = None;
        let mut pq_sk: Option<PQSigSecretKey> = None;

        let needs_pq = !matches!(mode, SignatureMode::Legacy);
        let needs_classical = matches!(mode, SignatureMode::Legacy | SignatureMode::Hybrid(_));

        if needs_pq {
            let family = match mode {
                SignatureMode::Hybrid(f) | SignatureMode::Pure(f) => f,
                _ => unreachable!(),
            };

            let (pk, sk) = match family {
                PQFamily::Dilithium => {
                    let (p, s) = dilithium_keypair();
                    (PQSigPublicKey::Dilithium(p), PQSigSecretKey::Dilithium(s))
                }
                PQFamily::Sphincs => {
                    let (p, s) = sphincs_keypair();
                    (PQSigPublicKey::Sphincs(p), PQSigSecretKey::Sphincs(s))
                }
                PQFamily::Falcon => {
                    let (p, s) = falcon_keypair();
                    (PQSigPublicKey::Falcon(p), PQSigSecretKey::Falcon(s))
                }
            };
            pq_pk = Some(pk);
            pq_sk = Some(sk);
        }

        if needs_classical {
            classical_sk = Some(SigningKey::generate(&mut OsRng));
        }

        let classical_pk = classical_sk.as_ref().map(VerifyingKey::from);

        (
            HybridSigPublicKey {
                classical: classical_pk,
                pq: pq_pk,
            },
            HybridSigSecretKey {
                classical: classical_sk,
                pq: pq_sk,
            },
        )
    }

    pub fn sign_detached(
        message: &[u8],
        sk: &HybridSigSecretKey,
        mode: &SignatureMode,
    ) -> HybridDetachedSignature {
        let mut classical_sig: Option<Signature> = None;
        if matches!(mode, SignatureMode::Legacy | SignatureMode::Hybrid(_)) {
            if let Some(c_sk) = &sk.classical {
                classical_sig = Some(c_sk.sign(message));
            }
        }

        let mut pq_sig: Option<PQDetachedSignature> = None;
        if !matches!(mode, SignatureMode::Legacy) {
            if let Some(pq_sk) = &sk.pq {
                let signed = match pq_sk {
                    PQSigSecretKey::Dilithium(s) => dilithium_sign(message, s),
                    PQSigSecretKey::Sphincs(s) => sphincs_sign(message, s),
                    PQSigSecretKey::Falcon(s) => falcon_sign(message, s),
                };
                let sig_bytes = signed.as_bytes()[message.len()..].to_vec();
                pq_sig = Some(match pq_sk {
                    PQSigSecretKey::Dilithium(_) => PQDetachedSignature::Dilithium(sig_bytes),
                    PQSigSecretKey::Sphincs(_) => PQDetachedSignature::Sphincs(sig_bytes),
                    PQSigSecretKey::Falcon(_) => PQDetachedSignature::Falcon(sig_bytes),
                });
            }
        }

        HybridDetachedSignature {
            classical: classical_sig,
            pq: pq_sig,
        }
    }

    pub fn verify_detached(
        message: &[u8],
        sig: &HybridDetachedSignature,
        pk: &HybridSigPublicKey,
        mode: &SignatureMode,
    ) -> bool {
        let mut valid = true;

        if matches!(mode, SignatureMode::Legacy | SignatureMode::Hybrid(_)) {
            if let (Some(c_sig), Some(c_pk)) = (&sig.classical, &pk.classical) {
                valid &= c_pk.verify(message, c_sig).is_ok();
            } else {
                valid = false;
            }
        }

        if !matches!(mode, SignatureMode::Legacy) {
            if let (Some(pq_sig), Some(pq_pk)) = (&sig.pq, &pk.pq) {
                let full_sig = match pq_sig {
                    PQDetachedSignature::Dilithium(b) | PQDetachedSignature::Sphincs(b) | PQDetachedSignature::Falcon(b) => b,
                };
                let full = [message, full_sig].concat();

                let open_ok = match (pq_pk, pq_sig) {
                    (PQSigPublicKey::Dilithium(pk), PQDetachedSignature::Dilithium(_)) => {
                        dilithium_open(&DilithiumSigned::from_bytes(&full), pk).is_ok()
                    }
                    (PQSigPublicKey::Sphincs(pk), PQDetachedSignature::Sphincs(_)) => {
                        sphincs_open(&SphincsSigned::from_bytes(&full), pk).is_ok()
                    }
                    (PQSigPublicKey::Falcon(pk), PQDetachedSignature::Falcon(_)) => {
                        falcon_open(&FalconSigned::from_bytes(&full), pk).is_ok()
                    }
                    _ => false,
                };

                if open_ok {
                    // Most implementations recover the message; extra safety
                    // In practice, recovered message is checked inside open for these crates
                    // But we keep conservative
                } else {
                    valid = false;
                }
            } else {
                valid = false;
            }
        }

        valid
    }
}
