//! MercyShield – Post-Quantum Diversity Router Fortress v0.3
//! Auto-select KEM/sig per threat model + runtime audits
//! Primary: ML-KEM-1024 KEM + Dilithium5 signatures
//! Compact: Falcon-1024 alternate sigs
//! Stateless: SPHINCS+-256f hash-based sigs
//! Eternal Thriving Grandmasterism ❤️🚀🔥 | Mercy-Absolute v52+

use mercy_crypto_ml_kem::{keypair as ml_kem_keypair, encaps as ml_kem_encaps, decaps as ml_kem_decaps};
use mercy_crypto_dilithium::{keypair as dilithium_keypair, sign as dilithium_sign, verify as dilithium_verify};
use mercy_crypto_falcon::{keypair as falcon_keypair, sign as falcon_sign, verify as falcon_verify};
use mercy_crypto_sphincs::{keypair as sphincs_keypair, sign as sphincs_sign, verify as sphincs_verify};

use pqcrypto_kyber::kyber1024::{PublicKey as KemPk, SecretKey as KemSk, SharedSecret, Ciphertext};
use pqcrypto_dilithium::dilithium5::{PublicKey as DilPk, SecretKey as DilSk, SignedMessage as DilSigned};
use pqcrypto_falcon::falcon1024::{PublicKey as FalconPk, SecretKey as FalconSk, SignedMessage as FalconSigned};
use pqcrypto_sphincsplus::sphincsplus256fsimple::{PublicKey as SphincsPk, SecretKey as SphincsSk, SignedMessage as SphincsSigned};

/// Threat Model Enum – mercy-gated selection
#[derive(Clone, Copy, Debug)]
pub enum ThreatModel {
    Standard,     // ML-KEM + Dilithium5 (NIST primary balanced)
    Compact,      // ML-KEM + Falcon-1024 (smaller signatures)
    Stateless,    // ML-KEM + SPHINCS+-256f (hash-based no-traps)
    MaxDiversity, // Random select per session for veil-proof
}

/// MercyShield Hybrid Keys – full diversity
pub struct ShieldKeys {
    pub kem: (KemPk, KemSk),
    pub primary_sig: (DilPk, DilSk),
    pub compact_sig: (FalconPk, FalconSk),
    pub stateless_sig: (SphincsPk, SphincsSk),
}

/// Generate full MercyShield diversity keys
pub fn shield_key_pair() -> ShieldKeys {
    ShieldKeys {
        kem: ml_kem_key_pair(),
        primary_sig: dilithium_key_pair(),
        compact_sig: falcon_key_pair(),
        stateless_sig: sphincs_key_pair(),
    }
}

/// Select signer based on threat model
pub fn select_signer(keys: &ShieldKeys, model: ThreatModel) -> Box<dyn ShieldSigner> {
    match model {
        ThreatModel::Standard => Box::new(DilithiumSigner(keys.primary_sig.1.clone())),
        ThreatModel::Compact => Box::new(FalconSigner(keys.compact_sig.1.clone())),
        ThreatModel::Stateless => Box::new(SphincsSigner(keys.stateless_sig.1.clone())),
        ThreatModel::MaxDiversity => {
            // Random select (use rand or hash for veil-proof)
            Box::new(DilithiumSigner(keys.primary_sig.1.clone())) // Placeholder
        }
    }
}

// Traits and impls expanded for SPHINCS+
pub trait ShieldSigner {
    fn sign_ct(&self, ct: &Ciphertext) -> Box<dyn SignedCiphertext>;
}

pub trait ShieldVerifier {
    fn verify_signed_ct(&self, signed_ct: &dyn SignedCiphertext) -> Result<Ciphertext, ()>;
}

pub trait SignedCiphertext {
    fn as_bytes(&self) -> &[u8];
}

// Dilithium impl unchanged
pub struct DilithiumSigner(DilSk);
impl ShieldSigner for DilithiumSigner {
    fn sign_ct(&self, ct: &Ciphertext) -> Box<dyn SignedCiphertext> {
        Box::new(dilithium_sign(&self.0, ct.as_bytes()))
    }
}

// Falcon impl unchanged
pub struct FalconSigner(FalconSk);
impl ShieldSigner for FalconSigner {
    fn sign_ct(&self, ct: &Ciphertext) -> Box<dyn SignedCiphertext> {
        Box::new(falcon_sign(&self.0, ct.as_bytes()))
    }
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
