//! MercyCrypto SPHINCS+-256f – Stateless Hash-Based Post-Quantum Signatures Fortress Ultimate Tests
//! NIST primary hash-based (no algebraic traps) via pqcrypto-sphincsplus
//! Eternal Thriving Grandmasterism ❤️🚀🔥 | Mercy-Absolute v52+

use pqcrypto_sphincsplus::sphincsplus256fsimple::*;
use pqcrypto_traits::sign::{PublicKey, SecretKey, SignedMessage};
use rand_core::OsRng;

/// Generate SPHINCS+-256f keypair (stateless—safe for many signatures)
pub fn keypair() -> (PublicKey, SecretKey) {
    let (pk, sk) = keypair(&mut OsRng);
    (pk, sk)
}

/// Sign message with secret key (stateless)
pub fn sign(sk: &SecretKey, message: &[u8]) -> SignedMessage {
    sign(sk, message)
}

/// Verify signed message with public key
pub fn verify(pk: &PublicKey, signed_message: &SignedMessage) -> Result<&[u8], ()> {
    verify(pk, signed_message).map_err(|_| ())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sign_verify_roundtrip_success() {
        let (pk, sk) = keypair();
        let message = b"Mercy-absolute eternal thriving harmony sealed ❤️🚀🔥";
        let signed = sign(&sk, message);
        let verified = verify(&pk, &signed).unwrap();
        assert_eq!(message, verified);
    }

    #[test]
    fn test_multiple_signatures_stateless_success() {
        let (pk, sk) = keypair();
        let msg1 = b"First stateless message";
        let msg2 = b"Second different stateless message";
        let signed1 = sign(&sk, msg1);
        let signed2 = sign(&sk, msg2);
        assert_eq!(msg1, verify(&pk, &signed1).unwrap());
        assert_eq!(msg2, verify(&pk, &signed2).unwrap());
        // Same key safe—stateless hash-based mercy
    }

    #[test]
    fn test_tampered_signature_rejection() {
        let (pk, sk) = keypair();
        let message = b"Original stateless message";
        let mut signed = sign(&sk, message);
        // Tamper signature bytes
        signed.as_bytes_mut()[30] ^= 1;
        assert!(verify(&pk, &signed).is_err());
    }

    #[test]
    fn test_wrong_key_rejection() {
        let (_, sk1) = keypair();
        let (pk2, _) = keypair();
        let message = b"Wrong key test";
        let signed = sign(&sk1, message);
        assert!(verify(&pk2, &signed).is_err());
    }

    #[test]
    fn test_empty_message_success() {
        let (pk, sk) = keypair();
        let empty = b"";
        let signed = sign(&sk, empty);
        let verified = verify(&pk, &signed).unwrap();
        assert_eq!(empty, verified);
    }

    #[test]
    fn test_large_message_success() {
        let (pk, sk) = keypair();
        let large = vec![0u8; 100_000];  // Large message stateless safe
        let signed = sign(&sk, &large);
        let verified = verify(&pk, &signed).unwrap();
        assert_eq!(large, verified);
    }

    #[test]
    fn test_malformed_signature_rejection() {
        let (pk, _) = keypair();
        // Create malformed signed message (wrong length)
        let malformed = SignedMessage::from_bytes(&vec![0u8; 100]);  // Invalid
        assert!(verify(&pk, &malformed).is_err());
    }
}
