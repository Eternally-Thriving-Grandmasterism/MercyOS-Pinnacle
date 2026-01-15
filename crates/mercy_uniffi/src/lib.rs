use uniffi::export;
use pqcrypto_kyber::kyber1024::{
    keypair as kem_keypair, encapsulate, decapsulate, PublicKey as KemPk, SecretKey as KemSk, Ciphertext as KemCt, SharedSecret,
};
use pqcrypto_dilithium::dilithium5::{
    keypair as sig_keypair, sign, verify, PublicKey as SigPk, SecretKey as SigSk, Signature,
};

// Simple valence pre-check (expand with UpgradedAlignmentLayer later)
fn mercy_precheck(input: &str) -> bool {
    let lower = input.to_lowercase();
    !lower.contains("harm") && !lower.contains("damage") && !lower.contains("coerce")
}

#[derive(uniffi::Record)]
pub struct KemKeypair {
    pub public_key: Vec<u8>,
    pub secret_key: Vec<u8>,  // Mercy-guarded in production
}

#[derive(uniffi::Record)]
pub struct KemEncapsulation {
    pub ciphertext: Vec<u8>,
    pub shared_secret: Vec<u8>,
}

#[derive(uniffi::Record)]
pub struct SigKeypair {
    pub public_key: Vec<u8>,
    pub secret_key: Vec<u8>,
}

#[derive(uniffi::Record)]
pub struct Proposal {
    pub content: String,
    pub amplified: bool,
}

#[uniffi::export]
pub fn kem_generate_keypair() -> KemKeypair {
    let (pk, sk) = kem_key_pair();
    KemKeypair {
        public_key: pk.as_bytes().to_vec(),
        secret_key: sk.as_bytes().to_vec(),
    }
}

#[uniffi::export]
pub fn kem_encapsulate(public_key: Vec<u8>) -> KemEncapsulation {
    let pk = KemPk::from_bytes(&public_key).expect("Invalid PK");
    let (ct, ss) = encapsulate(&pk);
    KemEncapsulation {
        ciphertext: ct.as_bytes().to_vec(),
        shared_secret: ss.as_bytes().to_vec(),
    }
}

#[uniffi::export]
pub fn kem_decapsulate(ciphertext: Vec<u8>, secret_key: Vec<u8>) -> Vec<u8> {
    let ct = KemCt::from_bytes(&ciphertext).expect("Invalid CT");
    let sk = KemSk::from_bytes(&secret_key).expect("Invalid SK");
    decapsulate(&ct, &sk)
        .expect("Decapsulation failed — mercy-block active attack")
        .as_bytes()
        .to_vec()
}

#[uniffi::export]
pub fn sig_generate_keypair() -> SigKeypair {
    let (pk, sk) = sig_keypair();
    SigKeypair {
        public_key: pk.as_bytes().to_vec(),
        secret_key: sk.as_bytes().to_vec(),
    }
}

#[uniffi::export]
pub fn sig_sign(message: String, secret_key: Vec<u8>) -> Vec<u8> {
    if !mercy_precheck(&message) {
        panic!("Mercy-block: Message misaligned — grace reframe required");
    }
    let sk = SigSk::from_bytes(&secret_key).expect("Invalid SK");
    sign(message.as_bytes(), &sk).as_bytes().to_vec()
}

#[uniffi::export]
pub fn sig_verify(message: String, signature: Vec<u8>, public_key: Vec<u8>) -> bool {
    let pk = SigPk::from_bytes(&public_key).expect("Invalid PK");
    let sig = Signature::from_bytes(&signature).expect("Invalid signature");
    verify(message.as_bytes(), &sig, &pk).is_ok()
}

#[uniffi::export]
pub async fn propose_mercy_gated(need: String) -> Proposal {
    if !mercy_precheck(&need) {
        return Proposal {
            content: "MERCY-GATED GRACE FALLBACK: reframed eternal ❤️".to_string(),
            amplified: false,
        };
    }
    Proposal {
        content: format!("ULTRA-AMPLIFIED: {} — thunder heart joy fusion eternal, GHZ-entangled thriving propagated! ❤️🚀🔥", need),
        amplified: true,
    }
}

uniffi::setup_scaffolding!();
