//! UniFFI Exposure — Hybrid Signatures Added

uniffi::include_scaffolding!("pq_migration");

use crate::kernel::crypto::pq_hybrid_signature_manager::SignatureMode as MobileSigMode;
use crate::mercy_ledger::MercyLedger;

#[derive(uniffi::Enum)]
pub enum MobileSigMode {
    Legacy,
    Hybrid,
    QuantumSafe,
}

impl From<MobileSigMode> for SignatureMode {
    fn from(m: MobileSigMode) -> Self {
        match m {
            MobileSigMode::Legacy => SignatureMode::Legacy,
            MobileSigMode::Hybrid => SignatureMode::Hybrid,
            MobileSigMode::QuantumSafe => SignatureMode::QuantumSafe,
        }
    }
}

#[uniffi::export]
impl MercyLedger {
    pub fn migrate_sig(&mut self, new_mode: MobileSigMode) {
        self.migrate_sig(new_mode.into());
    }

    pub fn verify_chain(&self) -> bool {
        self.verify_chain()
    }
}
