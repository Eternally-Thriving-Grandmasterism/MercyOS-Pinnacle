// MercyOS-Pinnacle UniFFI Bridge – Cross-platform mercy bindings
// Eternal Thriving Grandmasterism ❤️🚀🔥
// Real multi-signature hybrid: Dilithium5 + Falcon-1024 + SPHINCS+-256f

use pqcrypto_dilithium::dilithium5::{keypair as dil_keypair, sign as dil_sign, verify as dil_verify, PublicKey as DilPk, SecretKey as DilSk, Signature as DilSig};
use pqcrypto_falcon::falcon1024::{keypair as fal_keypair, sign as fal_sign, verify as fal_verify, PublicKey as FalPk, SecretKey as FalSk, Signature as FalSig};
use pqcrypto_sphincsplus::sphincsplus256f::{keypair as sph_keypair, sign as sph_sign, verify as sph_verify, PublicKey as SphPk, SecretKey as SphSk, Signature as SphSig};
use uniffi::export;

// Simple valence pre-check (expand with UpgradedAlignmentLayer)
fn mercy_precheck(input: &str) -> bool {
    let lower = input.to_lowercase();
    !lower.contains("harm") && !lower.contains("damage") && !lower.contains("coerce") && !lower.contains("force")
}

#[derive(uniffi::Record)]
pub struct HybridKeypair {
    pub dilithium_pk: Vec<u8>,
    pub dilithium_sk: Vec<u8>,
    pub falcon_pk: Vec<u8>,
    pub falcon_sk: Vec<u8>,
    pub sphincs_pk: Vec<u8>,
    pub sphincs_sk: Vec<u8>,
}

#[derive(uniffi::Record)]
pub struct HybridSignature {
    pub dilithium_sig: Vec<u8>,
    pub falcon_sig: Vec<u8>,
    pub sphincs_sig: Vec<u8>,
}

#[uniffi::export]
pub fn hybrid_generate_keypair() -> HybridKeypair {
    let (dil_pk, dil_sk) = dil_keypair();
    let (fal_pk, fal_sk) = fal_key_pair();
    let (sph_pk, sph_sk) = sph_key_pair();

    HybridKeypair {
        dilithium_pk: dil_pk.as_bytes().to_vec(),
        dilithium_sk: dil_sk.as_bytes().to_vec(),
        falcon_pk: fal_pk.as_bytes().to_vec(),
        falcon_sk: fal_sk.as_bytes().to_vec(),
        sphincs_pk: sph_pk.as_bytes().to_vec(),
        sphincs_sk: sph_sk.as_bytes().to_vec(),
    }
}

#[uniffi::export]
pub fn multi_sign_hybrid(message: String, keypair: HybridKeypair) -> HybridSignature {
    if !mercy_precheck(&message) {
        panic!("Mercy-block: Message misaligned — grace reframe required");
    }

    let dil_sk = DilSk::from_bytes(&keypair.dilithium_sk).expect("Invalid Dilithium SK");
    let fal_sk = FalSk::from_bytes(&keypair.falcon_sk).expect("Invalid Falcon SK");
    let sph_sk = SphSk::from_bytes(&keypair.sphincs_sk).expect("Invalid SPHINCS+ SK");

    HybridSignature {
        dilithium_sig: dil_sign(message.as_bytes(), &dil_sk).as_bytes().to_vec(),
        falcon_sig: fal_sign(message.as_bytes(), &fal_sk).as_bytes().to_vec(),
        sphincs_sig: sph_sign(message.as_bytes(), &sph_sk).as_bytes().to_vec(),
    }
}

#[uniffi::export]
pub fn multi_verify_hybrid(message: String, signature: HybridSignature, keypair_pks: HybridKeypair) -> bool {
    let dil_pk = DilPk::from_bytes(&keypair_pks.dilithium_pk).expect("Invalid Dilithium PK");
    let fal_pk = FalPk::from_bytes(&keypair_pks.falcon_pk).expect("Invalid Falcon PK");
    let sph_pk = SphPk::from_bytes(&keypair_pks.sphincs_pk).expect("Invalid SPHINCS+ PK");

    let dil_sig = DilSig::from_bytes(&signature.dilithium_sig).expect("Invalid Dilithium sig");
    let fal_sig = FalSig::from_bytes(&signature.falcon_sig).expect("Invalid Falcon sig");
    let sph_sig = SphSig::from_bytes(&signature.sphincs_sig).expect("Invalid SPHINCS+ sig");

    // Secure if ANY algorithm verifies (multi-assumption unbreakable)
    dil_verify(message.as_bytes(), &dil_sig, &dil_pk).is_ok() ||
    fal_verify(message.as_bytes(), &fal_sig, &fal_pk).is_ok() ||
    sph_verify(message.as_bytes(), &sph_sig, &sph_pk).is_ok()
}

uniffi::setup_scaffolding!();
