//! Repository: https://github.com/Eternally-Thriving-Grandmasterism/MercyOS-Pinnacle
//! Full path: bindings/pq_hybrid_signature_uniffi.rs

uniffi::include_scaffolding!("pq_migration");

use crate::kernel::crypto::pq_hybrid_signature_manager::{PQFamily, SignatureMode};
use crate::mercy_ledger::MercyLedger;

#[derive(uniffi::Enum)]
pub enum MobileSigMode {
    Legacy,
    HybridDilithium,
    PureDilithium,
    HybridSphincs,
    PureSphincs,
    HybridFalcon,
    PureFalcon,
}

impl From<MobileSigMode> for SignatureMode {
    fn from(m: MobileSigMode) -> Self {
        match m {
            MobileSigMode::Legacy => SignatureMode::Legacy,
            MobileSigMode::HybridDilithium => SignatureMode::Hybrid(PQFamily::Dilithium),
            MobileSigMode::PureDilithium => SignatureMode::Pure(PQFamily::Dilithium),
            MobileSigMode::HybridSphincs => SignatureMode::Hybrid(PQFamily::Sphincs),
            MobileSigMode::PureSphincs => SignatureMode::Pure(PQFamily::Sphincs),
            MobileSigMode::HybridFalcon => SignatureMode::Hybrid(PQFamily::Falcon),
            MobileSigMode::PureFalcon => SignatureMode::Pure(PQFamily::Falcon),
        }
    }
}

#[uniffi::export]
impl MercyLedger {
    pub fn migrate_sig(&mut self, new_mode: MobileSigMode) {
        let internal = SignatureMode::from(new_mode);
        self.migrate_sig(internal);
    }

    pub fn verify_chain(&self) -> bool {
        self.verify_chain()
    }
}
