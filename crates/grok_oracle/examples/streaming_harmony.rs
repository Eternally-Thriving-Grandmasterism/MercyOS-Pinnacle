//! Example: Streaming Grok Oracle + Multi-PQ Encryption & Signature demo

use grok_oracle::GrokOracle;
use mercyos_pinnacle::kernel::crypto::pq_stream::{
    generate_key_pair as kyber_key_pair, PQStreamEncryptor, PQStreamDecryptor,
};
use mercyos_pinnacle::kernel::crypto::pq_hqc::PQHQCModule;
use mercyos_pinnacle::kernel::crypto::pq_sign::PQSignatureModule;
use mercyos_pinnacle::kernel::crypto::pq_falcon::PQFalconModule;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // ... (previous Grok streaming + amplify + multi-sign code unchanged up to amplified)

    println!("\n\n{}", amplified);

    // === Multi-Algorithm PQ Encryption + Signature Demo ===
    println!("\n=== Initiating Diversity-PQ Secure Signed Propagation ===");

    // Existing Dilithium + Falcon signing (unchanged)
    let dilithium_module = PQSignatureModule::new();
    let dilithium_sig = dilithium_module.sign(amplified.as_bytes());
    let falcon_module = PQFalconModule::new();
    let falcon_sig = falcon_module.sign(amplified.as_bytes());

    // NEW: HQC-256 code-based KEM for session (diversity vs Kyber lattice)
    let hqc_module = PQHQCModule::new();
    let hqc_pk = hqc_module.public_key();
    println!("HQC-256 council identity generated (PK {} bytes)", hqc_pk.as_bytes().len());

    let (ct, ss) = hqc_module.initiate();
    println!("HQC-256 session encapsulated (ciphertext {} bytes)", ct.as_bytes().len());

    // Use HQC shared secret as AEAD key (direct 64-byte → ChaCha20Poly1305 compatible via HKDF if needed)
    let cipher = chacha20poly1305::ChaCha20Poly1305::new((&ss.as_bytes()[..32]).into());

    // Bundle: proposal + signatures
    let bundle = [
        amplified.as_bytes(),
        dilithium_sig.as_bytes(),
        falcon_sig.as_bytes(),
    ].concat();

    // Single-chunk encrypt for demo (extend to streaming in production)
    let mut nonce = [0u8; 12];
    let mut encrypted_bundle = bundle.clone();
    let tag = cipher.encrypt_in_place_detached(&nonce.into(), b"", &mut encrypted_bundle)
        .expect("HQC-derived AEAD encrypt failed");
    encrypted_bundle.extend_from_slice(&tag);

    println!("HQC-256 secured multi-signed bundle ({} → {} bytes)", bundle.len(), encrypted_bundle.len());

    // Receiver side: recover SS + decrypt + verify
    let ss_recv = hqc_module.accept(&ct).unwrap();
    let cipher_recv = chacha20poly1305::ChaCha20Poly1305::new((&ss_recv.as_bytes()[..32]).into());

    let (mut ct_bundle, recv_tag) = encrypted_bundle.split_at_mut(bundle.len());
    cipher_recv.decrypt_in_place_detached(&nonce.into(), b"", &mut ct_bundle, (&recv_tag).into())
        .expect("HQC-derived AEAD auth failed");

    // Verify signatures on recovered proposal (offsets same as sender)
    let offset1 = amplified.as_bytes().len();
    let offset2 = offset1 + dilithium_sig.as_bytes().len();
    let received_proposal = &ct_bundle[..offset1];

    dilithium_module.verify(received_proposal, &dilithium_sig)?;
    falcon_module.verify(received_proposal, &falcon_sig)?;

    println!("HQC-256 + Dilithium + Falcon multi-PQ roundtrip verified IMMACULATE — Code/lattice diversity eternal thriving propagated! ❤️🚀🔥");

    Ok(())
}
