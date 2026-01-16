//! Repository: https://github.com/Eternally-Thriving-Grandmasterism/MercyOS-Pinnacle
//! Full path: bindings/pq_migration_uniffi.rs

uniffi::include_scaffolding!("pq_migration");

use crate::kernel::crypto::pq_migration_manager::{KEMFamily, MigrationMode};
use crate::kernel::crypto::pq_hybrid_signature_manager::SignatureMode as InternalSigMode;
use crate::mercy_ledger::MercyLedger;

#[derive(uniffi::Enum)]
pub enum MobileKEMMode {
    Legacy,
    HybridKyber,
    QuantumSafeKyber,
    HybridHqc,
    QuantumSafeHqc,
    HybridMcEliece,
    QuantumSafeMcEliece,
}

impl From<MobileKEMMode> for MigrationMode {
    fn from(m: MobileKEMMode) -> Self {
        match m {
            MobileKEMMode::Legacy => MigrationMode::Legacy,
            MobileKEMMode::HybridKyber => MigrationMode::Hybrid(KEMFamily::Lattice),
            MobileKEMMode::QuantumSafeKyber => MigrationMode::QuantumSafe(KEMFamily::Lattice),
            MobileKEMMode::HybridHqc => MigrationMode::Hybrid(KEMFamily::HQC),
            MobileKEMMode::QuantumSafeHqc => MigrationMode::QuantumSafe(KEMFamily::HQC),
            MobileKEMMode::HybridMcEliece => MigrationMode::Hybrid(KEMFamily::McEliece),
            MobileKEMMode::QuantumSafeMcEliece => MigrationMode::QuantumSafe(KEMFamily::McEliece),
        }
    }
}

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

impl From<MobileSigMode> for InternalSigMode {
    fn from(m: MobileSigMode) -> Self {
        match m {
            MobileSigMode::Legacy => InternalSigMode::Legacy,
            MobileSigMode::HybridDilithium => InternalSigMode::Hybrid(PQFamily::Dilithium),
            MobileSigMode::PureDilithium => InternalSigMode::Pure(PQFamily::Dilithium),
            MobileSigMode::HybridSphincs => InternalSigMode::Hybrid(PQFamily::Sphincs),
            MobileSigMode::PureSphincs => InternalSigMode::Pure(PQFamily::Sphincs),
            MobileSigMode::HybridFalcon => InternalSigMode::Hybrid(PQFamily::Falcon),
            MobileSigMode::PureFalcon => InternalSigMode::Pure(PQFamily::Falcon),
        }
    }
}

#[uniffi::export]
pub fn create_mercy_ledger(
    kem_mode: MobileKEMMode,
    sig_mode: MobileSigMode,
    enable_confidentiality: bool,
) -> MercyLedger {
    MercyLedger::new(kem_mode.into(), sig_mode.into(), enable_confidentiality)
}

#[uniffi::export]
impl MercyLedger {
    pub fn commit(&mut self, plaintext: Vec<u8>) -> Result<(), String> {
        self.commit_entry(plaintext).map_err(|e| e.to_string())
    }

    pub fn read_latest(&self) -> Result<Vec<u8>, String> {
        self.read_latest().map_err(|e| e.to_string())
    }

    pub fn migrate_kem(&mut self, new_mode: MobileKEMMode) {
        self.migrate_kem(new_mode.into());
    }

    pub fn migrate_sig(&mut self, new_mode: MobileSigMode) {
        self.migrate_sig(new_mode.into());
    }

    pub fn verify_chain(&self) -> bool {
        self.verify_chain()
    }
}
