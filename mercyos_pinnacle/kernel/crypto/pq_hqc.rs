//! Post-Quantum Encryption — HQC-256 (Code-Based KEM)
//! Diversity-resilient session key encapsulation for eternal council propagation
//! Forged January 2026 — MercyOS-Pinnacle Ultramasterpiece
//! MIT License — Open Beacon Eternal
//!
//! Security Parameters (HQC-256 Level 5 intended):
//! - n = 57_637
//! - w = 114
//! - Public Key:  7_249 bytes
//! - Private Key: 7_285 bytes
//! - Ciphertext:  14_498 bytes
//! - Shared Secret: 64 bytes (post-processed)

use pqcrypto_hqc::hqc256::{
    keypair, encapsulate, decapsulate,
    PublicKey, SecretKey, Ciphertext, SharedSecret,
};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum PQHQCError {
    #[error("Decapsulation failed")]
    DecapsFailed,
}

pub struct PQHQCModule {
    sk: SecretKey,
    pk: PublicKey,
}

impl PQHQCModule {
    /// Generate long-term HQC-256 council identity keypair (run once, persist SK mercy-guarded)
    pub fn new() -> Self {
        let (pk, sk) = keypair();
        Self { sk, pk }
    }

    /// Initiate session → ciphertext + shared secret (ephemeral)
    pub fn initiate(&self) -> (Ciphertext, SharedSecret) {
        encapsulate(&self.pk)
    }

    /// Accept session → recover shared secret
    pub fn accept(&self, ct: &Ciphertext) -> Result<SharedSecret, PQHQCError> {
        decapsulate(ct, &self.sk).ok_or(PQHQCError::DecapsFailed)
    }

    /// Get public key for open propagation
    pub fn public_key(&self) -> PublicKey {
        self.pk.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roundtrip_immaculate() {
        let module = PQHQCModule::new();

        let (ct, ss_sender) = module.initiate();

        let ss_receiver = module.accept(&ct).unwrap();

        assert_eq!(ss_sender.as_bytes(), ss_receiver.as_bytes());
    }
}
