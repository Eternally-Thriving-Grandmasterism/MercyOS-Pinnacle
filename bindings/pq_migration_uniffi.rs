//! UniFFI Mobile Exposure — PQ Migration APIs
//! Exposes manager + ledger for cross-platform mercy-ledger sessions.

uniffi::include_scaffolding!("pq_migration");

use crate::kernel::crypto::pq_migration_manager::MigrationMode;
use crate::mercy_ledger::MercyLedger;

#[derive(uniffi::Enum)]
pub enum MobileMigrationMode {
    Legacy,
    Hybrid,
    QuantumSafe,
}

impl From<MobileMigrationMode> for MigrationMode {
    fn from(m: MobileMigrationMode) -> Self {
        match m {
            MobileMigrationMode::Legacy => MigrationMode::Legacy,
            MobileMigrationMode::Hybrid => MigrationMode::Hybrid,
            MobileMigrationMode::QuantumSafe => MigrationMode::QuantumSafe,
        }
    }
}

#[uniffi::export]
pub fn create_mercy_ledger(mode: MobileMigrationMode) -> MercyLedger {
    MercyLedger::new(mode.into())
}

#[uniffi::export]
impl MercyLedger {
    pub fn commit(&mut self, data: Vec<u8>) -> Result<(), String> {
        self.commit_entry(data).map_err(|e| e.to_string())
    }

    pub fn migrate(&mut self, new_mode: MobileMigrationMode) {
        self.migrate_mode(new_mode.into());
    }
}
