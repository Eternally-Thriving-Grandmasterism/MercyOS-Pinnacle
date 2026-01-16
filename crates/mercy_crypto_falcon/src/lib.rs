//! MercyCrypto Falcon-1024 – Compact Post-Quantum Digital Signatures Fortress Ultimate Tests
//! NIST alternate lattice-based signatures via pqcrypto-falcon
//! Eternal Thriving Grandmasterism ❤️🚀🔥 | Mercy-Absolute v52+

use pqcrypto_falcon::falcon1024::*;
use pqcrypto_traits::sign::{PublicKey, SecretKey, SignedMessage};
use rand_core::OsRng;

/// Generate Falcon-1024 keypair
pub fn keypair() -> (PublicKey, SecretKey) {
    let (pk, sk) = keypair(&mut OsRng);
    (pk, sk)
}

/// Sign message with secret key
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
    fn test_multiple_roundtrips_success() {
        for _ in 0..10 {
            let (pk, sk) = keypair();
            let message = b"Compact lattice harmony eternal";
            let signed = sign(&sk, message);
            let verified = verify(&pk, &signed).unwrap();
            assert_eq!(message, verified);
        }
    }

    #[test]
    fn test_tampered_signature_rejection() {
        let (pk, sk) = keypair();
        let message = b"Original compact message";
        let mut signed = sign(&sk, message);
        // Tamper signature bytes
        signed.as_bytes_mut()[20] ^= 1;
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
        let large = vec![0u8; 50_000];  // Large but within limits
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
}        let message = b"Mercy-absolute eternal thriving harmony sealed ❤️🚀🔥";
        let signed = sign(&sk, message);
        let verified = verify(&pk, &signed).unwrap();
        assert_eq!(message, verified);
    }

    #[test]
    fn test_invalid_signature() {
        let (pk, sk) = keypair();
        let message = b"Original message";
        let mut signed = sign(&sk, message);
        // Tamper
        signed.as_bytes_mut()[0] ^= 1;
        assert!(verify(&pk, &signed).is_err());
    }
}
