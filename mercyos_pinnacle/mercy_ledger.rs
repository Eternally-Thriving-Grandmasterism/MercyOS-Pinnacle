//! Repository: https://github.com/Eternally-Thriving-Grandmasterism/MercyOS-Pinnacle
//! Full path: mercyos_pinnacle/mercy_ledger.rs
//! Mercy Ledger — Eternal Hybrid Multi-Family NIST Ledger

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

    // ... (commit_entry, verify_chain, migrate_kem unchanged)

    pub fn migrate_sig(&mut self, new_mode: SignatureMode) {
        let (new_pk, new_sk) = PQSignatureMigrator::keypair(new_mode.clone());
        self.sig_pk = new_pk;
        self.sig_sk = new_sk;
        self.current_sig_mode = new_mode;
    }
}
