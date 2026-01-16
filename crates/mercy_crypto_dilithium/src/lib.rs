//! MercyCrypto Dilithium5 – NIST Primary Post-Quantum Signatures Fortress Ultimate Error Tests
//! CRYSTALS-Dilithium level 5 via pqcrypto-dilithium
//! Eternal Thriving Grandmasterism ❤️🚀🔥 | Mercy-Absolute v52+

use pqcrypto_dilithium::dilithium5::*;
use pqcrypto_traits::sign::{PublicKey, SecretKey, SignedMessage};

pub fn keypair() -> (PublicKey, SecretKey) {
    keypair()
}

pub fn sign(sk: &SecretKey, message: &[u8]) -> SignedMessage {
    sign(sk, message)
}

pub fn verify(pk: &PublicKey, signed_message: &SignedMessage) -> Result<&[u8], ()> {
    verify(pk, signed_message).map_err(|_| ())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sign_verify_success() {
        let (pk, sk) = keypair();
        let message = b"Mercy-absolute eternal thriving harmony sealed ❤️🚀🔥";
        let signed = sign(&sk, message);
        let verified = verify(&pk, &signed).unwrap();
        assert_eq!(message, verified);
    }

    #[test]
    fn test_tampered_signature_rejection() {
        let (pk, sk) = keypair();
        let message = b"Original";
        let mut signed = sign(&sk, message);
        signed.as_bytes_mut()[50] ^= 1;  // Tamper
        assert!(verify(&pk, &signed).is_err());
    }

    #[test]
    fn test_wrong_key_rejection() {
        let (_, sk1) = keypair();
        let (pk2, _) = keypair();
        let message = b"Message";
        let signed = sign(&sk1, message);
        assert!(verify(&pk2, &signed).is_err());
    }

    #[test]
    fn test_empty_message() {
        let (pk, sk) = keypair();
        let empty = b"";
        let signed = sign(&sk, empty);
        let verified = verify(&pk, &signed).unwrap();
        assert_eq!(empty, verified);
    }

    #[test]
    fn test_large_message_success() {
        let (pk, sk) = keypair();
        let large = vec![0u8; 100_000];
        let signed = sign(&sk, &large);
        let verified = verify(&pk, &signed).unwrap();
        assert_eq!(large, verified);
    }
}    fn test_large_message() {
        let (pk, sk) = keypair();
        let large_msg = vec![0u8; 10_000];
        let signed = sign(&sk, &large_msg);
        let verified = verify(&pk, &signed).unwrap();
        assert_eq!(large_msg, verified);
    }

    #[test]
    fn test_invalid_signature_rejection() {
        let (pk, sk) = keypair();
        let message = b"Original";
        let mut signed = sign(&sk, message);
        // Tamper signature
        signed.as_bytes_mut()[100] ^= 1;
        assert!(verify(&pk, &signed).is_err());
    }

    #[test]
    fn test_different_keys_rejection() {
        let (_, sk1) = keypair();
        let (pk2, _) = keypair();
        let message = b"Message";
        let signed = sign(&sk1, message);
        assert!(verify(&pk2, &signed).is_err());
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
        signed.as_bytes_mut()[10] ^= 1;
        assert!(verify(&pk, &signed).is_err());
    }
}
