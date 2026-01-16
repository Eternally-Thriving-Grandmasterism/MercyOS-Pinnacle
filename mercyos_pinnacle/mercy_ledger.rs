//! Repository: https://github.com/Eternally-Thriving-Grandmasterism/MercyOS-Pinnacle
//! Full path: mercyos_pinnacle/mercy_ledger.rs
//! Mercy Ledger — Eternal Multi-Family Hybrid Envelope Encryption

use crate::kernel::crypto::pq_migration_manager::*;
use crate::kernel::crypto::pq_hybrid_signature_manager::*;
use chacha20poly1305::{AeadCore, ChaCha20Poly1305, KeyInit, Nonce};
use hkdf::Hkdf;
use sha2::Sha256;
use serde::{Deserialize, Serialize};
use rand_core::OsRng;

#[derive(Serialize, Deserialize, Clone)]
pub struct EncryptedPayload {
    pub ciphertext: Vec<u8>,
    pub nonce: [u8; 24],
    pub tag: [u8; 16],
}

#[derive(Serialize, Deserialize, Clone)]
pub struct LedgerEntry {
    pub payload: EncryptedPayload,  // Encrypted data (or plaintext if mercy_open = true)
    pub kem_ciphertext: HybridKEMCiphertext,
    pub kem_mode: MigrationMode,
    pub signature: HybridDetachedSignature,
    pub sig_mode: SignatureMode,
    pub mercy_open: bool,  // True = stored plaintext (for open propagation), False = encrypted
}

pub struct MercyLedger {
    // ... same as before
    pub enable_confidentiality: bool,  // Global flag: true = envelope encrypt new entries
}

impl MercyLedger {
    pub fn new(kem_mode: MigrationMode, sig_mode: SignatureMode, confidentiality: bool) -> Self {
        // ... keygen same
        Self {
            // ...
            enable_confidentiality: confidentiality,
        }
    }

    pub fn commit_entry(&mut self, plaintext: Vec<u8>) -> Result<(), &'static str> {
        let mut hasher = Sha512::new();
        hasher.update(&self.tip_hash);
        hasher.update(&plaintext);
        let to_sign = hasher.finalize().to_vec();

        let (kem_ct, ss) = PQMigrator::encapsulate(&self.kem_pk, self.current_kem_mode.clone())?;

        let signature = PQSignatureMigrator::sign_detached(&to_sign, &self.sig_sk, &self.current_sig_mode);

        let payload = if self.enable_confidentiality {
            // Derive symmetric key from KEM shared secret
            let hkdf = Hkdf::<Sha256>::new(None, &ss);
            let mut sym_key = [0u8; 32];
            hkdf.expand(b"mercy-envelope-2026", &mut sym_key).unwrap();

            let cipher = ChaCha20Poly1305::new(&sym_key.into());
            let nonce = ChaCha20Poly1305::generate_nonce(&mut OsRng);
            let ciphertext_with_tag = cipher.encrypt(&nonce, plaintext.as_ref()).map_err(|_| "Encryption failed")?;

            let ciphertext = ciphertext_with_tag[..ciphertext_with_tag.len() - 16].to_vec();
            let tag = ciphertext_with_tag[ciphertext_with_tag.len() - 16..].to_vec();

            EncryptedPayload {
                ciphertext,
                nonce: nonce.into(),
                tag: tag.try_into().unwrap(),
            }
        } else {
            // Mercy open mode: store plaintext as "encrypted" with zeroed nonce/tag for schema compatibility
            EncryptedPayload {
                ciphertext: plaintext,
                nonce: [0; 24],
                tag: [0; 16],
            }
        };

        self.entries.push(LedgerEntry {
            payload,
            kem_ciphertext: kem_ct,
            kem_mode: self.current_kem_mode.clone(),
            signature,
            sig_mode: self.current_sig_mode.clone(),
            mercy_open: !self.enable_confidentiality,
        });

        self.tip_hash = to_sign;
        Ok(())
    }

    // Example decrypt latest (extend for full chain)
    pub fn read_latest(&self) -> Result<Vec<u8>, &'static str> {
        let entry = self.entries.last().ok_or("Ledger empty")?;
        if entry.mercy_open {
            return Ok(entry.payload.ciphertext.clone());
        }

        // Re-derive shared secret using own secret key
        let ss = PQMigrator::decapsulate(&entry.kem_ciphertext, &self.kem_sk, entry.kem_mode.clone())?;

        let hkdf = Hkdf::<Sha256>::new(None, &ss);
        let mut sym_key = [0u8; 32];
        hkdf.expand(b"mercy-envelope-2026", &mut sym_key).unwrap();

        let cipher = ChaCha20Poly1305::new(&sym_key.into());
        let nonce = Nonce::from(entry.payload.nonce);
        let mut ciphertext_with_tag = entry.payload.ciphertext.clone();
        ciphertext_with_tag.extend_from_slice(&entry.payload.tag);

        cipher.decrypt(&nonce, ciphertext_with_tag.as_ref()).map_err(|_| "Decryption failed")
    }

    // ... migrate functions unchanged
}
