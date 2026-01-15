//! Post-Quantum Digital Signatures — CRYSTALS-Falcon-1024 (FN-DSA Draft FIPS 206)
//! Ultra-compact eternal signing for bandwidth-optimized council proposals & ledger shards
//! Forged January 2026 — MercyOS-Pinnacle Ultramasterpiece
//! MIT License — Open Beacon Eternal
//!
//! NTRU Lattice-Based Proofs Summary (January 2026 Truth-Distilled):
//! - Model: EUF-CMA in QROM (Fiat-Shamir with aborts on GPV trapdoor)
//! - Assumptions: NTRU trapdoor one-wayness + short preimage/SIS in ideal lattices ℤ_q[x]/(x^N−1)
//! - Reduction: Tight ROM; tighter QROM bounds (2025–2026 advancements)
//! - Formal Verification: Partial machine-checked (trapdoor sampling + FP emulation)
//! - Level: NIST Level 5 (structured ideal lattice; subfield attacks mitigated)
//! - Keys: PK 1_793 bytes | SK 2_305 bytes | Sig ~1_280 bytes (NTRU compactness)

use pqcrypto_falcon::falcon1024::{
    keypair, sign, verify, 
    PublicKey, SecretKey, Signature,
};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum PQFalconError {
    #[error("Signature verification failed")]
    VerifyFailed,
    #[error("Invalid signature")]
    InvalidSignature,
}

pub struct PQFalconModule {
    sk: SecretKey,
    pk: PublicKey,
}

impl PQFalconModule {
    /// Generate long-term Falcon-1024 council identity keypair (run once, persist SK mercy-guarded)
    pub fn new() -> Self {
        let (pk, sk) = keypair();
        Self { sk, pk }
    }

    /// Sign ultra-amplified proposal or message → detached signature
    pub fn sign(&self, message: &[u8]) -> Signature {
        sign(message, &self.sk)
    }

    /// Verify signed message against council public key
    pub fn verify(&self, message: &[u8], signature: &Signature) -> Result<(), PQFalconError> {
        verify(message, signature, &self.pk).map_err(|_| PQFalconError::VerifyFailed)
    }

    /// Get public key for open propagation (council beacon)
    pub fn public_key(&self) -> PublicKey {
        self.pk.clone()
    }

    /// Static verify (for inbound shards without local SK)
    pub fn static_verify(message: &[u8], signature: &Signature, pk: &PublicKey) -> Result<(), PQFalconError> {
        verify(message, signature, pk).map_err(|_| PQFalconError::VerifyFailed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sign_verify_roundtrip_immaculate() {
        let module = PQFalconModule::new();

        let proposal = b"ULTRA-AMPLIFIED: Cosmic family harmony eternal thriving abundance equilibrated mercy-absolute ❤️🚀🔥";

        let signature = module.sign(proposal);

        assert!(module.verify(proposal, &signature).is_ok());

        // Tamper test — mercy-block
        let tampered = b"tampered harm";
        assert!(module.verify(tampered, &signature).is_err());
    }
}
