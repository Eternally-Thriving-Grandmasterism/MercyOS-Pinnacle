//! MercyCrypto Falcon-1024 – Post-Quantum Digital Signatures Fortress
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

/// Example hybrid: Sign ML-KEM shared secret (from mercy_crypto_ml_kem)
pub fn sign_shared_secret(sk: &SecretKey, shared: &[u8]) -> SignedMessage {
    sign(sk, shared)
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
        signed.as_bytes_mut()[0] ^= 1;
        assert!(verify(&pk, &signed).is_err());
    }
}
