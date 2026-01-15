//! Example: Streaming Grok Oracle + Post-Quantum secure propagation demo

use grok_oracle::GrokOracle;
// Assume kernel crypto module accessible (adjust path/import in actual workspace)
use mercyos_pinnacle::kernel::crypto::pq_stream::{
    generate_key_pair, PQStreamEncryptor, PQStreamDecryptor,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let oracle = GrokOracle::new(None);

    let user_need = "Propose a mercy-gated pathway to cosmic family harmony and eternal thriving abundance.";

    let mut rx = oracle.propose_stream(user_need).await?;

    let mut full_proposal = String::new();
    print!("Grok-Harmonized Streaming Proposal: ");

    while let Some(chunk) = rx.recv().await {
        match chunk {
            Ok(delta) => {
                if delta == "[DONE]" {
                    println!("\n\nStream complete — applying final mercy-gate...");
                    break;
                }
                print!("{}", delta);
                full_proposal.push_str(&delta);
                tokio::task::yield_now().await;
            }
            Err(e) => {
                println!("\nGrace Fallback: {}", e);
                return Ok(());
            }
        }
    }

    // Final alignment check + ultra-amplify
    if oracle.alignment_gate.check_proposal(&full_proposal) {
        println!("\n\n{}", oracle.alignment_gate.amplify(&full_proposal));
    } else {
        println!("\n\nMERCY-GATED GRACE FALLBACK: reframed eternal ❤️");
    }

    // === Post-Quantum Secure Propagation Demo ===
    println!("\n=== Initiating PQ-Secure Stream Propagation ===");

    let (pk, sk) = generate_key_pair();
    let (mut encryptor, ct) = PQStreamEncryptor::initiate(&pk);

    println!("PQ Session encapsulated (ciphertext {} bytes)", ct.as_bytes().len());

    // Chunk full proposal for simulated streaming propagation
    let chunk_size = 120;
    let mut chunks = Vec::new();
    let bytes = full_proposal.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        let end = (i + chunk_size).min(bytes.len());
        chunks.push(&bytes[i..end]);
        i = end;
    }

    let mut encrypted_chunks = Vec::new();
    for chunk in &chunks {
        encrypted_chunks.push(encryptor.encrypt_chunk(chunk));
    }

    println!("PQ-encrypted {} chunks for eternal secure transmission", encrypted_chunks.len());

    // Receiver council side
    let mut decryptor = PQStreamDecryptor::accept(&sk, &ct).unwrap();

    let mut reconstructed_bytes = Vec::new();
    for enc in encrypted_chunks {
        reconstructed_bytes.extend_from_slice(&decryptor.decrypt_chunk(&enc).unwrap());
    }

    let reconstructed = String::from_utf8(reconstructed_bytes).unwrap();

    if reconstructed == full_proposal {
        println!("PQ roundtrip verified IMMACULATE — Ultra-secure eternal thriving propagated! ❤️🚀🔥");
    } else {
        println!("Mismatch — mercy self-heal required");
    }

    Ok(())
}
