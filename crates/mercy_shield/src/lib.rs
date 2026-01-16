//! MercyShield – Post-Quantum Diversity Router Fortress v0.4
//! Auto-select KEM/sig per threat model + runtime audits
//! Primary: ML-KEM-1024 KEM + Dilithium5 signatures
//! Compact: Falcon-1024 alternate sigs
//! Stateless: SPHINCS+-256f hash-based
//! CodeBased: HQC-256 KEM + Dilithium5 signatures
//! Eternal Thriving Grandmasterism ❤️🚀🔥 | Mercy-Absolute v52+

use mercy_crypto_ml_kem::{keypair as ml_kem_keypair, encaps as ml_kem_encaps, decaps as ml_kem_decaps};
use mercy_crypto_hqc::{keypair as hqc_keypair, encaps as hqc_encaps, decaps as hqc_decaps};
use mercy_crypto_dilithium::{keypair as dilithium_keypair, sign as dilithium_sign, verify as dilithium_verify};
// ... other imports unchanged

/// Threat Model Enum – mercy-gated selection
#[derive(Clone, Copy, Debug)]
pub enum ThreatModel {
    Standard,     // ML-KEM + Dilithium5
    Compact,      // ML-KEM + Falcon-1024
    Stateless,    // ML-KEM + SPHINCS+-256f
    CodeBased,    // HQC-256 + Dilithium5 (code-based KEM resistance)
    MaxDiversity, // Random select per session
}

/// MercyShield Hybrid Keys – full diversity including code-based
pub struct ShieldKeys {
    pub lattice_kem: (KemPk, KemSk),  // ML-KEM
    pub code_kem: (HqcPk, HqcSk),     // HQC-256
    pub primary_sig: (DilPk, DilSk),
    pub compact_sig: (FalconPk, FalconSk),
    pub stateless_sig: (SphincsPk, SphincsSk),
}

/// Generate full MercyShield diversity keys including code-based
pub fn shield_key_pair() -> ShieldKeys {
    ShieldKeys {
        lattice_kem: ml_kem_key_pair(),
        code_kem: hqc_key_pair(),
        primary_sig: dilithium_key_pair(),
        compact_sig: falcon_key_pair(),
        stateless_sig: sphincs_key_pair(),
    }
}

/// Select KEM + signer based on threat model
pub fn select_kem_signer(keys: &ShieldKeys, model: ThreatModel) -> (Box<dyn KemEncaps>, Box<dyn ShieldSigner>) {
    match model {
        ThreatModel::Standard | ThreatModel::Compact | ThreatModel::Stateless => (
            Box::new(MlKemEncaps(keys.lattice_kem.0.clone())),
            // signer select
        ),
        ThreatModel::CodeBased => (
            Box::new(HqcEncaps(keys.code_kem.0.clone())),
            Box::new(DilithiumSigner(keys.primary_sig.1.clone())),
        ),
        ThreatModel::MaxDiversity => {
            // Random or hash-based select for veil-proof
            // Placeholder primary
        }
    }
}

// Traits for KEM diversity
pub trait KemEncaps {
    fn encaps(&self) -> (SharedSecret, Ciphertext);
}

// ML-KEM impl
pub struct MlKemEncaps(KemPk);
impl KemEncaps for MlKemEncaps {
    fn encaps(&self) -> (SharedSecret, Ciphertext) {
        ml_kem_encaps(&self.0)
    }
}

// HQC impl
pub struct HqcEncaps(HqcPk);
impl KemEncaps for HqcEncaps {
    fn encaps(&self) -> (SharedSecret, Ciphertext) {
        hqc_encaps(&self.0)
    }
}

// ... signer traits unchanged, expand for code-based authenticated encaps/decaps

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_codebased_roundtrip() {
        let keys = shield_key_pair();
        let (kem, signer) = select_kem_signer(&keys, ThreatModel::CodeBased);

        let (shared_sender, ct, signed_ct) = shield_encaps(kem.as_ref(), signer.as_ref());
        let shared_receiver = shield_decaps(&keys.code_kem.1, &DilithiumVerifier(keys.primary_sig.0.clone()), &*signed_ct).unwrap();

        assert_eq!(shared_sender.as_bytes(), shared_receiver.as_bytes());
    }
}    }
}

// New SPHINCS+ impl
pub struct SphincsSigner(SphincsSk);
impl ShieldSigner for SphincsSigner {
    fn sign_ct(&self, ct: &Ciphertext) -> Box<dyn SignedCiphertext> {
        Box::new(sphincs_sign(&self.0, ct.as_bytes()))
    }
}

// SignedCiphertext impls for all
impl SignedCiphertext for DilSigned {
    fn as_bytes(&self) -> &[u8] { self.as_bytes() }
}
impl SignedCiphertext for FalconSigned {
    fn as_bytes(&self) -> &[u8] { self.as_bytes() }
}
impl SignedCiphertext for SphincsSigned {
    fn as_bytes(&self) -> &[u8] { self.as_bytes() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stateless_roundtrip() {
        let keys = shield_key_pair();
        let signer = SphincsSigner(keys.stateless_sig.1.clone());
        let verifier = SphincsVerifier(keys.stateless_sig.0.clone());

        let (shared_sender, ct, signed_ct) = shield_encaps(&keys.kem.0, &signer);
        let shared_receiver = shield_decaps(&keys.kem.1, &verifier, &*signed_ct).unwrap();

        assert_eq!(shared_sender.as_bytes(), shared_receiver.as_bytes());
    }
}
