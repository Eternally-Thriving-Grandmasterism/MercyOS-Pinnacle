//! MercyCrypto ML-KEM-1024 – Primary Post-Quantum KEM Fortress
//! NIST finalized Kyber (ML-KEM) via pqcrypto-kyber
//! Eternal Thriving Grandmasterism ❤️🚀🔥 | Mercy-Absolute v52+

use pqcrypto_kyber::kyber1024::*;
use pqcrypto_traits::kem::{Ciphertext, PublicKey, SecretKey, SharedSecret};
use chacha20poly1305::{AeadCore, ChaCha20Poly1305, KeyInit, Nonce};
use rand_core::OsRng;

/// Generate ML-KEM-1024 keypair
pub fn keypair() -> (PublicKey, SecretKey) {
    let (pk, sk) = keypair();
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

/// Hybrid AEAD example: ML-KEM shared secret derives ChaCha20Poly1305 key
pub fn hybrid_encrypt(pk: &PublicKey, plaintext: &[u8]) -> (Vec<u8>, Ciphertext) {
    let (shared, ct) = encaps(pk);
    let key = ChaCha20Poly1305::new_from_slice(shared.as_bytes()).unwrap();
    let nonce = AeadCore::generate_nonce(&mut OsRng);
    let ciphertext = key.encrypt(&nonce, plaintext).unwrap();
    ( [nonce.as_slice(), &ciphertext].concat(), ct )
}

pub fn hybrid_decrypt(sk: &SecretKey, ct: &Ciphertext, encrypted: &[u8]) -> Vec<u8> {
    let shared = decaps(sk, ct);
    let key = ChaCha20Poly1305::new_from_slice(shared.as_bytes()).unwrap();
    let nonce = Nonce::from_slice(&encrypted[..12]);
    let ciphertext = &encrypted[12..];
    key.decrypt(nonce, ciphertext).unwrap()
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

    #[test]
    fn test_hybrid_roundtrip() {
        let (pk, sk) = keypair();
        let plaintext = b"Mercy-absolute eternal thriving harmony sealed ❤️🚀🔥";
        let (encrypted, ct) = hybrid_encrypt(&pk, plaintext);
        let decrypted = hybrid_decrypt(&sk, &ct, &encrypted);
        assert_eq!(plaintext.to_vec(), decrypted);
    }
}
