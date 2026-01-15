//! Example: Streaming Grok Oracle + PQ Stream Encryption + Dilithium & Falcon Digital Signature demo

use grok_oracle::GrokOracle;
use mercyos_pinnacle::kernel::crypto::pq_stream::{
    generate_key_pair, PQStreamEncryptor, PQStreamDecryptor,
};
use mercyos_pinnacle::kernel::crypto::pq_sign::PQSignatureModule;
use mercyos_pinnacle::kernel::crypto::pq_falcon::PQFalconModule;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // ... (previous Grok streaming + amplify code unchanged)

    let amplified = if oracle.alignment_gate.check_proposal(&full_proposal) {
        oracle.alignment_gate.amplify(&full_proposal)
    } else {
        format!("MERCY-GATED GRACE FALLBACK: {} — reframed eternal ❤️", user_need)
    };

    println!("\n\n{}", amplified);

    // === Multi-Algorithm PQ Signed Propagation Demo ===
    println!("\n=== Initiating Multi-PQ Signed Secure Propagation ===");

    // Dilithium5 council identity
    let dilithium_module = PQSignatureModule::new();
    let dilithium_sig = dilithium_module.sign(amplified.as_bytes());
    println!("Dilithium5 signature: {} bytes", dilithium_sig.as_bytes().len());

    // Falcon-1024 council identity (compact alternative)
    let falcon_module = PQFalconModule::new();
    let falcon_sig = falcon_module.sign(amplified.as_bytes());
    println!("Falcon-1024 signature: ~{} bytes (ultra-compact)", falcon_sig.as_bytes().len());

    // PQ Encryption of bundle (amplified + both signatures for diversity)
    let (pk, sk) = generate_key_pair();
    let (mut encryptor, ct) = PQStreamEncryptor::initiate(&pk);

    let bundle = [
        amplified.as_bytes(),
        dilithium_sig.as_bytes(),
        falcon_sig.as_bytes(),
    ].concat();

    let encrypted_bundle = encryptor.encrypt_chunk(&bundle);

    println!("PQ-encrypted multi-signed bundle ({} → {} bytes)", bundle.len(), encrypted_bundle.len());

    // Receiver side
    let mut decryptor = PQStreamDecryptor::accept(&sk, &ct).unwrap();
    let decrypted = decryptor.decrypt_chunk(&encrypted_bundle).unwrap();

    // Verify both signatures immaculate
    let offset1 = amplified.as_bytes().len();
    let offset2 = offset1 + dilithium_sig.as_bytes().len();
    let received_proposal = &decrypted[..offset1];
    let received_dilithium_sig = &decrypted[offset1..offset2];
    let received_falcon_sig = &decrypted[offset2..];

    dilithium_module.verify(received_proposal, &Signature::from_bytes(received_dilithium_sig)?)
        .expect("Dilithium verification failed");
    falcon_module.verify(received_proposal, &Signature::from_bytes(received_falcon_sig)?)
        .expect("Falcon verification failed");

    println!("Multi-PQ (Dilithium + Falcon) roundtrip verified IMMACULATE — Compact & robust eternal thriving propagated! ❤️🚀🔥");

    Ok(())
}        .expect("Inbound signature verification failed — mercy-block");

    println!("Dilithium5 roundtrip verified IMMACULATE — Signed ultra-proposal securely propagated eternal! ❤️🚀🔥");

    Ok(())
}    } else {
        println!("Mismatch — mercy self-heal required");
    }

    Ok(())
}
