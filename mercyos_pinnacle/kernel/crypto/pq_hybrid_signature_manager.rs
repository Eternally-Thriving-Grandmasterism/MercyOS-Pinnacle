//! PQ Hybrid Signature Manager — Transition Agility Beacon
//! ML-DSA integration (dilithium5 ≈ ML-DSA-87)
//! Supports Legacy (Ed25519), Hybrid (dual), QuantumSafe (ML-DSA only)

use pqcrypto_dilithium::dilithium5::*;
use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};
use rand_core::OsRng;

#[derive(Clone)]
pub enum SignatureMode {
    Legacy,
    Hybrid,
    QuantumSafe,
}

#[derive(Clone)]
pub struct HybridSigPublicKey {
    pub classical: Option<VerifyingKey>,
    pub pq: PublicKey,
}

#[derive(Clone)]
pub struct HybridSigSecretKey {
    pub classical: Option<SigningKey>,
    pub pq: SecretKey,
}

#[derive(Clone)]
pub struct HybridDetachedSignature {
    pub classical: Option<Signature>,
    pub pq: Vec<u8>,
}

pub struct PQSignatureMigrator;

impl PQSignatureMigrator {
    pub fn keypair(mode: SignatureMode) -> (HybridSigPublicKey, HybridSigSecretKey) {
        let (pq_pk, pq_sk) = keypair();

        let classical = match mode {
            SignatureMode::Legacy | SignatureMode::Hybrid => {
                let c_sk = SigningKey::generate(&mut OsRng);
                Some(c_sk)
            }
            SignatureMode::QuantumSafe => None,
        };

        let c_pk = classical.as_ref().map(VerifyingKey::from);

        (
            HybridSigPublicKey { classical: c_pk, pq: pq_pk },
            HybridSigSecretKey { classical, pq: pq_sk },
        )
    }

    pub fn sign_detached(
        message: &[u8],
        sk: &HybridSigSecretKey,
        mode: &SignatureMode,
    ) -> HybridDetachedSignature {
        let classical = sk.classical.as_ref().map(|c_sk| c_sk.sign(message));

        let signed = sign(message, &sk.pq);
        let pq = signed.as_bytes()[message.len()..].to_vec();

        HybridDetachedSignature { classical, pq }
    }

    pub fn verify_detached(
        message: &[u8],
        sig: &HybridDetachedSignature,
        pk: &HybridSigPublicKey,
        mode: &SignatureMode,
    ) -> bool {
        let classical_required = matches!(mode, SignatureMode::Legacy | SignatureMode::Hybrid);
        let pq_required = matches!(mode, SignatureMode::Hybrid | SignatureMode::QuantumSafe);

        let mut valid = true;

        if classical_required {
            if let (Some(c_sig), Some(c_pk)) = (&sig.classical, &pk.classical) {
                valid &= c_pk.verify(message, c_sig).is_ok();
            } else {
                valid = false;
            }
        }

        if pq_required {
            let mut full = message.to_vec();
            full.extend_from_slice(&sig.pq);
            if let Ok(recovered) = open(&SignedMessage::from_bytes(&full), &pk.pq) {
                valid &= recovered.as_bytes() == message;
            } else {
                valid = false;
            }
        }

        valid
    }
}
