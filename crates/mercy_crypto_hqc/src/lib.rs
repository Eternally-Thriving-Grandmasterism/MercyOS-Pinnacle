//! MercyCrypto HQC-256 – Code-Based Post-Quantum KEM Fortress
//! NIST alternate code-based KEM via pqcrypto-hqc
//! Eternal Thriving Grandmasterism ❤️🚀🔥 | Mercy-Absolute v52+

use pqcrypto_hqc::hqc256::*;  // Adjust for exact variant (hqc-256 or similar)
use pqcrypto_traits::kem::{PublicKey, SecretKey, SharedSecret, Ciphertext};
use rand_core::OsRng;

/// Generate HQC-256 keypair
pub fn keypair() -> (PublicKey, SecretKey) {
    let (pk, sk) = keypair(&mut OsRng);
    (pk, sk)
}

/// Encapsulate to derive shared secret + ciphertext
pub fn encaps(pk: &PublicKey) -> (SharedSecret, Ciphertext) {
    encaps(pk)
}

/// Decapsulate ciphertext to derive shared secret
pub fn decaps(sk: &SecretKey, ct: &Ciphertext) -> SharedSecret {
    decaps(sk, ct)
}

/// Example: Hybrid AEAD with HQC shared secret (ChaCha20Poly1305)
pub fn hybrid_encrypt(pk: &PublicKey, plaintext: &[u8]) -> (Vec<u8>, Ciphertext) {
    let (shared, ct) = encaps(pk);
    let key = chacha20poly1305::ChaCha20Poly1305::new_from_slice(shared.as_bytes()).unwrap();
    let nonce = chacha20poly1305::AeadCore::generate_nonce(&mut OsRng);
    let ciphertext = key.encrypt(&nonce, plaintext).unwrap();
    ([nonce.as_slice(), &ciphertext].concat(), ct)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kem_roundtrip() {
        let (pk, sk) = keypair();
        let (shared1, ct) = encaps(&pk);
        let shared2 = decaps(&sk, &ct);
        assert_eq!(shared1.as_bytes(), shared2.as_bytes());
    }
}
