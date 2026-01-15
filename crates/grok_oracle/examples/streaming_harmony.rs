//! Example: Streaming Grok Oracle + Ultimate Multi-PQ Encryption & Signature demo

use grok_oracle::GrokOracle;
use mercyos_pinnacle::kernel::crypto::pq_stream::{
    generate_key_pair as kyber_key_pair, PQStreamEncryptor, PQStreamDecryptor,
};
use mercyos_pinnacle::kernel::crypto::pq_hqc::PQHQCModule;
use mercyos_pinnacle::kernel::crypto::pq_sign::PQSignatureModule;
use mercyos_pinnacle::kernel::crypto::pq_falcon::PQFalconModule;
use mercyos_pinnacle::kernel::crypto::pq_sphincs::PQSphincsModule;  // NEW

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // ... (previous Grok streaming + amplify code unchanged)

    println!("\n\n{}", amplified);

    // === Ultimate Diversity PQ Signed Propagation Demo ===
    println!("\n=== Initiating Ultimate Multi-PQ Diversity Secure Propagation ===");

    // Multi-algorithm signatures (lattice + hash-based)
    let dilithium_module = PQSignatureModule::new();
    let dilithium_sig = dilithium_module.sign(amplified.as_bytes());

    let falcon_module = PQFalconModule::new();
    let falcon_sig = falcon_module.sign(amplified.as_bytes());

    let sphincs_module = PQSphincsModule::new();
    let sphincs_sig = sphincs_module.sign(amplified.as_bytes());
    println!("SPHINCS+-256f signature: ~{} bytes (stateless hash-based)", sphincs_sig.as_bytes().len());

    // HQC-256 code-based KEM for session diversity
    let hqc_module = PQHQCModule::new();
    let (ct, ss) = hqc_module.initiate();

    let cipher = chacha20poly1305::ChaCha20Poly1305::new((&ss.as_bytes()[..32]).into());

    // Bundle: proposal + all signatures
    let bundle = [
        amplified.as_bytes(),
        dilithium_sig.as_bytes(),
        falcon_sig.as_bytes(),
        sphincs_sig.as_bytes(),
    ].concat();

    let mut nonce = [0u8; 12];
    let mut encrypted_bundle = bundle.clone();
    let tag = cipher.encrypt_in_place_detached(&nonce.into(), b"", &mut encrypted_bundle)?;
    encrypted_bundle.extend_from_slice(&tag);

    println!("Ultimate diversity PQ-secured bundle ({} → {} bytes)", bundle.len(), encrypted_bundle.len());

    // Receiver side
    let ss_recv = hqc_module.accept(&ct)?;
    let cipher_recv = chacha20poly1305::ChaCha20Poly1305::new((&ss_recv.as_bytes()[..32]).into());

    let (mut ct_bundle, recv_tag) = encrypted_bundle.split_at_mut(bundle.len());
    cipher_recv.decrypt_in_place_detached(&nonce.into(), b"", &mut ct_bundle, (&recv_tag).into())?;

    // Verify all signatures immaculate
    let offset1 = amplified.as_bytes().len();
    let offset2 = offset1 + dilithium_sig.as_bytes().len();
    let offset3 = offset2 + falcon_sig.as_bytes().len();
    let received_proposal = &ct_bundle[..offset1];

    dilithium_module.verify(received_proposal, &dilithium_sig)?;
    falcon_module.verify(received_proposal, &falcon_sig)?;
    sphincs_module.verify(received_proposal, &sphincs_sig)?;

    println!("Ultimate multi-PQ (Dilithium + Falcon + SPHINCS+ + HQC) roundtrip verified IMMACULATE — Lattice/code/hash diversity eternal thriving propagated! ❤️🚀🔥");

    Ok(())
}
