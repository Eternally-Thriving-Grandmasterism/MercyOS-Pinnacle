//! Post-Quantum Stream Encryption Module
//! ML-KEM (Kyber1024) session key encapsulation + ChaCha20Poly1305 AEAD chunked streaming
//! Eternal secure propagation for council proposals & Grok epiphanies
//! Forged January 2026 — MercyOS-Pinnacle Ultramasterpiece
//! MIT License — Open Beacon Eternal

use pqcrypto_kyber::kyber1024::{
    keypair, encapsulate, decapsulate,
    PublicKey, SecretKey, Ciphertext, SharedSecret,
};
use chacha20poly1305::{
    aead::{AeadInPlace, KeyInit},
    ChaCha20Poly1305,
    Nonce,
};
use chacha20poly1305::aead::generic_array::GenericArray;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum PQStreamError {
    #[error("Decapsulation failed")]
    DecapsFailed,
    #[error("Authentication failed")]
    AuthFailed,
    #[error("Invalid encrypted chunk (too short for tag)")]
    InvalidChunk,
}

pub struct PQStreamEncryptor {
    session_key: SharedSecret,
    nonce_counter: u64,
}

pub struct PQStreamDecryptor {
    session_key: SharedSecret,
    nonce_counter: u64,
}

impl PQStreamEncryptor {
    /// Initiate session against recipient's long-term public key
    pub fn initiate(recipient_pk: &PublicKey) -> (Self, Ciphertext) {
        let (ct, ss) = encapsulate(recipient_pk);
        (
            Self {
                session_key: ss,
                nonce_counter: 0,
            },
            ct,
        )
    }

    /// Encrypt one stream chunk (arbitrary bytes) → ciphertext || 16-byte tag
    pub fn encrypt_chunk(&mut self, plaintext: &[u8]) -> Vec<u8> {
        let cipher = ChaCha20Poly1305::new(GenericArray::from_slice(&self.session_key));

        let mut nonce_bytes = [0u8; 12];
        nonce_bytes[4..12].copy_from_slice(&self.nonce_counter.to_le_bytes());
        let nonce = Nonce::from_slice(&nonce_bytes);

        let mut ct = plaintext.to_vec();
        let tag = cipher
            .encrypt_in_place_detached(nonce, b"", &mut ct)
            .expect("PQ stream encrypt failed");

        self.nonce_counter += 1;

        ct.extend_from_slice(tag.as_slice());
        ct
    }
}

impl PQStreamDecryptor {
    /// Accept session from received ciphertext using own secret key
    pub fn accept(own_sk: &SecretKey, ct: &Ciphertext) -> Result<Self, PQStreamError> {
        let ss = decapsulate(ct, own_sk).ok_or(PQStreamError::DecapsFailed)?;
        Ok(Self {
            session_key: ss,
            nonce_counter: 0,
        })
    }

    /// Decrypt one encrypted chunk → plaintext or auth error
    pub fn decrypt_chunk(&mut self, encrypted: &[u8]) -> Result<Vec<u8>, PQStreamError> {
        if encrypted.len() < 16 {
            return Err(PQStreamError::InvalidChunk);
        }
        let (ct, tag_bytes) = encrypted.split_at(encrypted.len() - 16);
        let tag = GenericArray::from_slice(tag_bytes);

        let mut nonce_bytes = [0u8; 12];
        nonce_bytes[4..12].copy_from_slice(&self.nonce_counter.to_le_bytes());
        let nonce = Nonce::from_slice(&nonce_bytes);

        let cipher = ChaCha20Poly1305::new(GenericArray::from_slice(&self.session_key));

        let mut pt = ct.to_vec();
        cipher
            .decrypt_in_place_detached(nonce, b"", &mut pt, tag)
            .map_err(|_| PQStreamError::AuthFailed)?;

        self.nonce_counter += 1;
        Ok(pt)
    }
}

/// Convenience: generate long-term keypair for council identity
pub fn generate_keypair() -> (PublicKey, SecretKey) {
    keypair()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roundtrip_verifies_immaculate() {
        let (pk, sk) = generate_key_pair();

        let (mut encryptor, ct) = PQStreamEncryptor::initiate(&pk);

        let chunks = vec![
            b"ULTRA-AMPLIFIED eternal thriving",
            b" thunder heart joy fusion propagated",
            b" ❤️🚀🔥",
        ];

        let mut encrypted_chunks = Vec::new();
        for chunk in &chunks {
            encrypted_chunks.push(encryptor.encrypt_chunk(chunk));
        }

        let mut decryptor = PQStreamDecryptor::accept(&sk, &ct).unwrap();

        let mut decrypted = Vec::new();
        for enc in encrypted_chunks {
            decrypted.push(decryptor.decrypt_chunk(&enc).unwrap());
        }

        assert_eq!(chunks.concat(), decrypted.concat());
    }
}
