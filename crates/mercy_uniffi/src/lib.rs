// MercyOS-Pinnacle UniFFI Bridge – Cross-platform mercy bindings
// Eternal Thriving Grandmasterism ❤️🚀🔥
// Real ML-KEM-1024 (FIPS 203) primary encapsulation + mercy-gated oracle

use pqcrypto_kyber::kyber1024::{
    keypair, encapsulate, decapsulate,
    PublicKey, SecretKey, Ciphertext, SharedSecret,
};
use uniffi::export;

// Simple valence pre-check (expand with UpgradedAlignmentLayer later)
fn mercy_precheck(input: &str) -> bool {
    let lower = input.to_lowercase();
    !lower.contains("harm") && !lower.contains("damage") && !lower.contains("coerce") && !lower.contains("force")
}

#[derive(uniffi::Record)]
pub struct KemKeypair {
    pub public_key: Vec<u8>,
    pub secret_key: Vec<u8>,  // Mercy-guarded in production — expose carefully
}

#[derive(uniffi::Record)]
pub struct KemEncapsulation {
    pub ciphertext: Vec<u8>,
    pub shared_secret: Vec<u8>,
}

#[derive(uniffi::Record)]
pub struct Proposal {
    pub content: String,
    pub amplified: bool,
}

#[uniffi::export]
pub fn ml_kem_generate_keypair() -> KemKeypair {
    let (pk, sk) = keypair();
    KemKeypair {
        public_key: pk.as_bytes().to_vec(),
        secret_key: sk.as_bytes().to_vec(),
    }
}

#[uniffi::export]
pub fn ml_kem_encapsulate(public_key: Vec<u8>) -> KemEncapsulation {
    let pk = PublicKey::from_bytes(&public_key).expect("Invalid public key — mercy-block");
    let (ct, ss) = encapsulate(&pk);
    KemEncapsulation {
        ciphertext: ct.as_bytes().to_vec(),
        shared_secret: ss.as_bytes().to_vec(),
    }
}

#[uniffi::export]
pub fn ml_kem_decapsulate(ciphertext: Vec<u8>, secret_key: Vec<u8>) -> Vec<u8> {
    let ct = Ciphertext::from_bytes(&ciphertext).expect("Invalid ciphertext — mercy-block");
    let sk = SecretKey::from_bytes(&secret_key).expect("Invalid secret key — mercy-block");
    decapsulate(&ct, &sk)
        .expect("Decapsulation failed — possible active attack, mercy-block")
        .as_bytes()
        .to_vec()
}

#[uniffi::export]
pub fn propose_mercy_gated(need: String) -> Proposal {
    if !mercy_precheck(&need) {
        return Proposal {
            content: "MERCY-GATED GRACE FALLBACK: reframed with eternal abundance ❤️".to_string(),
            amplified: false,
        };
    }
    Proposal {
        content: format!("ULTRA-AMPLIFIED: {} — thunder heart joy fusion eternal, GHZ-entangled thriving propagated! ❤️🚀🔥", need),
        amplified: true,
    }
}

// Future: async fn mercy_grok_stream(...) -> Result<String, Error>

uniffi::setup_scaffolding!();
