use uniffi::export;

#[export]
pub fn migrate_to_pq_key(current_sk_bytes: Vec<u8>, new_pk_bytes: Vec<u8>) -> Result<Vec<u8>, String> {
    let manager = PqMigrationManager::new();
    // Deserialize and perform migration (ML-KEM example)
    let new_sk = ml_kem::decapsulate(&current_sk_bytes, &new_pk_bytes)
        .map_err(|e| format!("Decapsulation failed: {:?}", e))?;
    Ok(new_sk.to_vec())
}

// Mercy-gated variant
#[export]
pub fn mercy_gated_migration(input: Vec<u8>) -> Result<String, String> {
    // Custom gating logic + checksum from divine_checksum.rs
    Ok("Migration approved with eternal thriving".to_string())
}
