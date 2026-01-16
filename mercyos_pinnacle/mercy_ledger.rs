//! Mercy Ledger — Eternal Hybrid Ledger with Signatures
//! Confidentiality (hybrid KEM) + Authenticity (hybrid signatures) + Chain integrity

use crate::kernel::crypto::pq_migration_manager::{HybridCiphertext, HybridPublicKey, HybridSecretKey, MigrationMode, PQMigrator};
use crate::kernel::crypto::pq_hybrid_signature_manager::{HybridDetachedSignature, HybridSigPublicKey, HybridSigSecretKey, PQSignatureMigrator, SignatureMode};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha512};

#[derive(Serialize, Deserialize, Clone)]
pub struct LedgerEntry {
    pub data: Vec<u8>,
    pub ciphertext: HybridCiphertext,
    pub kem_mode: MigrationMode,
    pub signature: HybridDetachedSignature,
    pub sig_mode: SignatureMode,
}

pub struct MercyLedger {
    pub entries: Vec<LedgerEntry>,
    pub current_kem_mode: MigrationMode,
    pub current_sig_mode: SignatureMode,
    kem_pk: HybridPublicKey,
    kem_sk: HybridSecretKey,
    sig_pk: HybridSigPublicKey,
    sig_sk: HybridSigSecretKey,
    tip_hash: Vec<u8>,
}

impl MercyLedger {
    pub fn new(kem_mode: MigrationMode, sig_mode: SignatureMode) -> Self {
        let (kem_pk, kem_sk) = PQMigrator::keypair(kem_mode.clone());
        let (sig_pk, sig_sk) = PQSignatureMigrator::keypair(sig_mode.clone());

        Self {
            entries: Vec::new(),
            current_kem_mode: kem_mode,
            current_sig_mode: sig_mode,
            kem_pk,
            kem_sk,
            sig_pk,
            sig_sk,
            tip_hash: Vec::new(),
        }
    }

    pub fn commit_entry(&mut self, data: Vec<u8>) -> Result<(), &'static str> {
        let mut hasher = Sha512::new();
        hasher.update(&self.tip_hash);
        hasher.update(&data);
        let to_sign = hasher.finalize().to_vec();

        let (ct, _ss) = PQMigrator::encapsulate(&self.kem_pk, self.current_kem_mode.clone())?;

        let signature = PQSignatureMigrator::sign_detached(&to_sign, &self.sig_sk, &self.current_sig_mode);

        self.entries.push(LedgerEntry {
            data,
            ciphertext: ct,
            kem_mode: self.current_kem_mode.clone(),
            signature,
            sig_mode: self.current_sig_mode.clone(),
        });

        self.tip_hash = to_sign;
        Ok(())
    }

    pub fn verify_chain(&self) -> bool {
        let mut current_hash = Vec::new();
        for entry in &self.entries {
            let mut hasher = Sha512::new();
            hasher.update(&current_hash);
            hasher.update(&entry.data);
            let expected = hasher.finalize().to_vec();

            if !PQSignatureMigrator::verify_detached(&expected, &entry.signature, &self.sig_pk, &entry.sig_mode) {
                return false;
            }
            current_hash = expected;
        }
        current_hash == self.tip_hash
    }

    pub fn migrate_kem(&mut self, new_mode: MigrationMode) {
        let (new_pk, new_sk) = PQMigrator::keypair(new_mode.clone());
        self.kem_pk = new_pk;
        self.kem_sk = new_sk;
        self.current_kem_mode = new_mode;
    }

    pub fn migrate_sig(&mut self, new_mode: SignatureMode) {
        let (new_pk, new_sk) = PQSignatureMigrator::keypair(new_mode.clone());
        self.sig_pk = new_pk;
        self.sig_sk = new_sk;
        self.current_sig_mode = new_mode;
    }
}
