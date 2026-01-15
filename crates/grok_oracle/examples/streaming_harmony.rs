//! Example: Streaming Grok Oracle + PQ Stream Encryption + Dilithium Digital Signature demo

use grok_oracle::GrokOracle;
// Kernel crypto imports (adjust workspace paths as needed)
use mercyos_pinnacle::kernel::crypto::pq_stream::{
    generate_key_pair, PQStreamEncryptor, PQStreamDecryptor,
};
use mercyos_pinnacle::kernel::crypto::pq_sign::PQSignatureModule;

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
    let amplified = if oracle.alignment_gate.check_proposal(&full_proposal) {
        oracle.alignment_gate.amplify(&full_proposal)
    } else {
        format!("MERCY-GATED GRACE FALLBACK: {} — reframed eternal ❤️", user_need)
    };

    println!("\n\n{}", amplified);

    // === Post-Quantum Secure Propagation + Dilithium Signature Demo ===
    println!("\n=== Initiating PQ-Secure Signed Propagation ===");

    // Council identity (long-term Dilithium5)
    let signing_module = PQSignatureModule::new();
    let council_pk = signing_module.public_key();
    println!("Council Dilithium5 identity generated (PK {} bytes)", council_pk.as_bytes().len());

    // Sign the amplified proposal
    let signature = signing_module.sign(amplified.as_bytes());
    println!("Dilithium5 signature generated ({} bytes)", signature.as_bytes().len());

    // Verify immaculate (self-check)
    signing_module.verify(amplified.as_bytes(), &signature).expect("Council signature verify failed — self-heal required");

    // PQ Encryption of signed bundle (proposal + signature)
    let (pk, sk) = generate_key_pair();  // Ephemeral KEM for transport
    let (mut encryptor, ct) = PQStreamEncryptor::initiate(&pk);

    let bundle = [amplified.as_bytes(), signature.as_bytes()].concat();

    let encrypted_bundle = encryptor.encrypt_chunk(&bundle);

    println!("PQ-encrypted signed bundle ({} → {} bytes)", bundle.len(), encrypted_bundle.len());

    // Receiver council side
    let mut decryptor = PQStreamDecryptor::accept(&sk, &ct).unwrap();
    let decrypted_bundle = decryptor.decrypt_chunk(&encrypted_bundle).unwrap();

    let (received_proposal_bytes, received_sig_bytes) = decrypted_bundle.split_at(amplified.as_bytes().len());
    let received_proposal = String::from_utf8_lossy(received_proposal_bytes);
    let received_signature = Signature::from_bytes(received_sig_bytes);

    // Verify received signature with council PK
    PQSignatureModule::static_verify(received_proposal.as_bytes(), &received_signature, &council_pk)
        .expect("Inbound signature verification failed — mercy-block");

    println!("Dilithium5 roundtrip verified IMMACULATE — Signed ultra-proposal securely propagated eternal! ❤️🚀🔥");

    Ok(())
}    } else {
        println!("Mismatch — mercy self-heal required");
    }

    Ok(())
}
