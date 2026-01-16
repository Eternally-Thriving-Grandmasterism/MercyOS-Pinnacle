//! MercyCrypto ML-KEM-1024 – Primary Post-Quantum KEM Fortress Ultimate Tests
//! NIST finalized Kyber (ML-KEM) via pqcrypto-kyber
//! Eternal Thriving Grandmasterism ❤️🚀🔥 | Mercy-Absolute v52+

use pqcrypto_kyber::kyber1024::*;
use pqcrypto_traits::kem::{Ciphertext, PublicKey, SecretKey, SharedSecret};

/// Generate ML-KEM-1024 keypair
pub fn keypair() -> (PublicKey, SecretKey) {
    keypair()
}

/// Encapsulate to derive shared secret + ciphertext
pub fn encaps(pk: &PublicKey) -> (SharedSecret, Ciphertext) {
    encaps(pk)
}

/// Decapsulate ciphertext to derive shared secret
pub fn decaps(sk: &SecretKey, ct: &Ciphertext) -> SharedSecret {
    decaps(sk, ct)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_keypair_encaps_decaps_roundtrip() {
        let (pk, sk) = keypair();
        let (shared1, ct) = encaps(&pk);
        let shared2 = decaps(&sk, &ct);
        assert_eq!(shared1.as_bytes(), shared2.as_bytes());
    }

    #[test]
    fn test_multiple_roundtrips() {
        for _ in 0..10 {
            let (pk, sk) = keypair();
            let (shared1, ct) = encaps(&pk);
            let shared2 = decaps(&sk, &ct);
            assert_eq!(shared1.as_bytes(), shared2.as_bytes());
        }
    }

    #[test]
    #[should_panic(expected = "DecapsulationError")]
    fn test_invalid_ciphertext_rejection() {
        let (pk, sk) = keypair();
        let (_, mut ct) = encaps(&pk);
        // Tamper ciphertext
        ct.as_bytes_mut()[0] ^= 1;
        let _ = decaps(&sk, &ct);  // Should panic/error mercy-gated
    }

    #[test]
    fn test_different_keys_different_shared() {
        let (pk1, _) = keypair();
        let (pk2, _) = keypair();
        let (shared1, _) = encaps(&pk1);
        let (shared2, _) = encaps(&pk2);
        assert_ne!(shared1.as_bytes(), shared2.as_bytes());
    }
}    let key = ChaCha20Poly1305::new_from_slice(shared.as_bytes()).unwrap();
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
