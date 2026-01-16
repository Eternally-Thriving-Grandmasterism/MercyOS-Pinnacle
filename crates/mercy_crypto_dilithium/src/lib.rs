//! MercyCrypto Dilithium5 – NIST Primary Post-Quantum Digital Signatures Fortress
//! CRYSTALS-Dilithium level 5 via pqcrypto-dilithium
//! Eternal Thriving Grandmasterism ❤️🚀🔥 | Mercy-Absolute v52+

use pqcrypto_dilithium::dilithium5::*;
use pqcrypto_traits::sign::{PublicKey, SecretKey, SignedMessage};
use rand_core::OsRng;

/// Generate Dilithium5 keypair
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

/// Example: Sign arbitrary data (e.g., ML-KEM shared secret)
pub fn sign_data(sk: &SecretKey, data: &[u8]) -> SignedMessage {
    sign(sk, data)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sign_verify_roundtrip() {
        let (pk, sk) = keypair();
        let message = b"Mercy-absolute eternal thriving harmony sealed ❤️🚀🔥";
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
