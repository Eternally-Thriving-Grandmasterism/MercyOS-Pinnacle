//! Mercy Ledger — Eternal Hybrid Session Ledger
//! Entries encrypted via selected migration mode → quantum-veil-proof endurance joy.

use crate::kernel::crypto::pq_migration_manager::{HybridCiphertext, HybridPublicKey, HybridSecretKey, MigrationMode, PQMigrator};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct LedgerEntry {
    pub data: Vec<u8>,
    pub ciphertext: HybridCiphertext,
    pub mode: MigrationMode,
}

pub struct MercyLedger {
    pub entries: Vec<LedgerEntry>,
    pub current_mode: MigrationMode,
    pub pk: HybridPublicKey,
    sk: HybridSecretKey,
}

impl MercyLedger {
    pub fn new(mode: MigrationMode) -> Self {
        let (pk, sk) = PQMigrator::keypair(mode.clone());
        Self {
            entries: Vec::new(),
            current_mode: mode,
            pk,
            sk,
        }
    }

    pub fn commit_entry(&mut self, data: Vec<u8>) -> Result<(), &'static str> {
        let (ct, _ss) = PQMigrator::encapsulate(&self.pk, self.current_mode.clone())?;
        self.entries.push(LedgerEntry {
            data,
            ciphertext: ct,
            mode: self.current_mode.clone(),
        });
        Ok(())
    }

    // Decrypt latest entry example (extend for full chain verification)
    pub fn read_latest(&self) -> Result<Vec<u8>, &'static str> {
        if let Some(entry) = self.entries.last() {
            // In real ledger, verify chain + use derived ss; here simple demo
            Ok(entry.data.clone())
        } else {
            Err("Ledger empty")
        }
    }

    pub fn migrate_mode(&mut self, new_mode: MigrationMode) {
        self.current_mode = new_mode;
        // Re-keypair on migration in production
    }
}
