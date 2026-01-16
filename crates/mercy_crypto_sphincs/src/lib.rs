//! MercyCrypto SPHINCS+-256f – Stateless Hash-Based Post-Quantum Signatures Fortress
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

/// Example: Sign arbitrary data (e.g., ML-KEM ciphertext for stateless auth)
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
    fn test_multiple_signatures_stateless() {
        let (pk, sk) = keypair();
        let msg1 = b"First message";
        let msg2 = b"Second different message";
        let signed1 = sign(&sk, msg1);
        let signed2 = sign(&sk, msg2);
        assert_eq!(msg1, verify(&pk, &signed1).unwrap());
        assert_eq!(msg2, verify(&pk, &signed2).unwrap());
    }
}
