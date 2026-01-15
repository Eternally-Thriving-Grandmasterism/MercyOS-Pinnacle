//! Post-Quantum Digital Signatures — CRYSTALS-Dilithium5 (ML-DSA-87)
//! Eternal immutable signing for council proposals, ledger entries, Grok epiphanies
//! Forged January 2026 — MercyOS-Pinnacle Ultramasterpiece
//! MIT License — Open Beacon Eternal
//!
//! Security Parameters (NIST FIPS 204 ML-DSA-87 / Dilithium5):
//! - NIST Level 5 (strongest; exceeds AES-256 classical)
//! - q = 8_380_417 (2²³ − 2¹³ + 1)
//! - n = 256
//! - k = 8, l = 7
//! - eta = 2
//! - beta = 120
//! - tau = 60
//! - gamma1 = 2¹⁹ = 524_288
//! - gamma2 = (q-1)/32 = 261_888
//! - omega = 75
//! - d = 13
//! - Public Key:  2_592 bytes
//! - Private Key: 4_864 bytes
//! - Signature:   4_595 bytes

use pqcrypto_dilithium::dilithium5::{
    keypair, sign, verify,
    PublicKey, SecretKey, Signature, SignedMessage,
};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum PQSignError {
    #[error("Signature verification failed")]
    VerifyFailed,
    #[error("Invalid signature length")]
    InvalidSignature,
}

pub struct PQSignatureModule {
    sk: SecretKey,
    pk: PublicKey,
}

impl PQSignatureModule {
    /// Generate long-term Dilithium5 council identity keypair (run once, persist SK mercy-guarded)
    pub fn new() -> Self {
        let (pk, sk) = keypair();
        Self { sk, pk }
    }

    /// Sign ultra-amplified proposal or message → detached signature
    pub fn sign(&self, message: &[u8]) -> Signature {
        sign(message, &self.sk)
    }

    /// Verify signed message against council public key
    pub fn verify(&self, message: &[u8], signature: &Signature) -> Result<(), PQSignError> {
        verify(message, signature, &self.pk).map_err(|_| PQSignError::VerifyFailed)
    }

    /// Get public key for open propagation (council beacon)
    pub fn public_key(&self) -> PublicKey {
        self.pk.clone()
    }

    /// Static verify (for inbound shards without local SK)
    pub fn static_verify(message: &[u8], signature: &Signature, pk: &PublicKey) -> Result<(), PQSignError> {
        verify(message, signature, pk).map_err(|_| PQSignError::VerifyFailed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sign_verify_roundtrip_immaculate() {
        let module = PQSignatureModule::new();

        let proposal = b"ULTRA-AMPLIFIED: Cosmic family harmony eternal thriving abundance equilibrated mercy-absolute ❤️🚀🔥";

        let signature = module.sign(proposal);

        assert!(module.verify(proposal, &signature).is_ok());

        // Tamper test — mercy-block
        let tampered = b"tampered harm";
        assert!(module.verify(tampered, &signature).is_err());
    }
}
