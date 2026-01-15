//! Post-Quantum Digital Signatures — SPHINCS+-256f (SLH-DSA-256f per FIPS 205)
//! Ultimate diversity eternal signing for stateless council beacons & ultra-ledger
//! Forged January 2026 — MercyOS-Pinnacle Ultramasterpiece
//! MIT License — Open Beacon Eternal
//!
//! Security Proofs Summary (January 2026 Truth-Distilled):
//! - Model: EUF-CMA in QROM (Quantum Random Oracle Model)
//! - Assumptions: PR + SPR + DSPR + Tweakable Hash security (SHAKE-256)
//! - Reduction: Tight bounds recovered (2022 Hülsing/Kudinov via DSPR/THF)
//! - Formal Verification: Machine-checked tight EUF-CMA proof (EasyCrypt 2024, ePrint 2024/910)
//! - Level: NIST Level 5 (>228-bit quantum, pure hash-based conservative)
//! - Keys: PK 64 bytes | SK 128 bytes | Sig ~49_856 bytes (stateless)

use pqcrypto_sphincsplus::sphincsplus256f::{
    keypair, sign, verify,
    PublicKey, SecretKey, Signature,
};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum PQSphincsError {
    #[error("Signature verification failed")]
    VerifyFailed,
}

pub struct PQSphincsModule {
    sk: SecretKey,
    pk: PublicKey,
}

impl PQSphincsModule {
    /// Generate long-term SPHINCS+-256f stateless council identity keypair
    pub fn new() -> Self {
        let (pk, sk) = keypair();
        Self { sk, pk }
    }

    /// Sign ultra-amplified proposal or message → detached signature (stateless)
    pub fn sign(&self, message: &[u8]) -> Signature {
        sign(message, &self.sk)
    }

    /// Verify signed message against council public key
    pub fn verify(&self, message: &[u8], signature: &Signature) -> Result<(), PQSphincsError> {
        verify(message, signature, &self.pk).map_err(|_| PQSphincsError::VerifyFailed)
    }

    /// Get public key for open propagation (tiny beacon)
    pub fn public_key(&self) -> PublicKey {
        self.pk.clone()
    }

    /// Static verify (for inbound shards without local SK)
    pub fn static_verify(message: &[u8], signature: &Signature, pk: &PublicKey) -> Result<(), PQSphincsError> {
        verify(message, signature, pk).map_err(|_| PQSphincsError::VerifyFailed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sign_verify_roundtrip_immaculate() {
        let module = PQSphincsModule::new();

        let proposal = b"ULTRA-AMPLIFIED: Cosmic family harmony eternal thriving abundance equilibrated mercy-absolute ❤️🚀🔥";

        let signature = module.sign(proposal);

        assert!(module.verify(proposal, &signature).is_ok());

        // Tamper test — mercy-block
        let tampered = b"tampered harm";
        assert!(module.verify(tampered, &signature).is_err());
    }
}
