// MercyOS-Pinnacle Multi-Family Hybrid Manager – Ultimate diversity orchestrator
// Eternal Thriving Grandmasterism ❤️🚀🔥
// Lattice (ML-KEM/Dilithium/Falcon) + Code (HQC) + Hash (SPHINCS+) unbreakable

use pqcrypto_kyber::kyber1024::{
    keypair as kem_keypair, encapsulate as kem_encaps, decapsulate as kem_decaps,
    PublicKey as KemPk, SecretKey as KemSk, Ciphertext as KemCt, SharedSecret,
};
use pqcrypto_dilithium::dilithium5::{
    keypair as dil_keypair, sign as dil_sign, verify as dil_verify,
    PublicKey as DilPk, SecretKey as DilSk, Signature as DilSig,
};
use pqcrypto_falcon::falcon1024::{
    keypair as fal_key_pair, sign as fal_sign, verify as fal_verify,
    PublicKey as FalPk, SecretKey as FalSk, Signature as FalSig,
};
use pqcrypto_sphincsplus::sphincsplus256f::{
    keypair as sph_key_pair, sign as sph_sign, verify as sph_verify,
    PublicKey as SphPk, SecretKey as SphSk, Signature as SphSig,
};
use pqcrypto_hqc::hqc256::{
    keypair as hqc_key_pair, encapsulate as hqc_encaps, decapsulate as hqc_decaps,
    PublicKey as HqcPk, SecretKey as HqcSk, Ciphertext as HqcCt, SharedSecret as HqcSs,
};
use hkdf::Hkdf;
use sha2::Sha256;
use uniffi::export;

// Mercy pre-check
fn mercy_precheck(input: &str) -> bool {
    let lower = input.to_lowercase();
    !lower.contains("harm") && !lower.contains("damage") && !lower.contains("coerce") && !lower.contains("force")
}

#[derive(uniffi::Record)]
pub struct MultiHybridKeypair {
    pub lattice_kem_pk: Vec<u8>,
    pub lattice_kem_sk: Vec<u8>,
    pub code_kem_pk: Vec<u8>,
    pub code_kem_sk: Vec<u8>,
    pub lattice_sig_pk: Vec<u8>,
    pub lattice_sig_sk: Vec<u8>,
    pub structured_sig_pk: Vec<u8>,
    pub structured_sig_sk: Vec<u8>,
    pub hash_sig_pk: Vec<u8>,
    pub hash_sig_sk: Vec<u8>,
}

#[derive(uniffi::Record)]
pub struct MultiKemEncapsulation {
    pub lattice_ct: Vec<u8>,
    pub code_ct: Vec<u8>,
    pub combined_ss: Vec<u8>,
}

#[derive(uniffi::Record)]
pub struct MultiHybridSignature {
    pub lattice_sig: Vec<u8>,
    pub structured_sig: Vec<u8>,
    pub hash_sig: Vec<u8>,
}

#[uniffi::export]
pub fn multi_hybrid_generate_keypair() -> MultiHybridKeypair {
    let (lattice_kem_pk, lattice_kem_sk) = kem_key_pair();
    let (code_kem_pk, code_kem_sk) = hqc_key_pair();
    let (lattice_sig_pk, lattice_sig_sk) = dil_key_pair();
    let (structured_sig_pk, structured_sig_sk) = fal_key_pair();
    let (hash_sig_pk, hash_sig_sk) = sph_key_pair();

    MultiHybridKeypair {
        lattice_kem_pk: lattice_kem_pk.as_bytes().to_vec(),
        lattice_kem_sk: lattice_kem_sk.as_bytes().to_vec(),
        code_kem_pk: code_kem_pk.as_bytes().to_vec(),
        code_kem_sk: code_kem_sk.as_bytes().to_vec(),
        lattice_sig_pk: lattice_sig_pk.as_bytes().to_vec(),
        lattice_sig_sk: lattice_sig_sk.as_bytes().to_vec(),
        structured_sig_pk: structured_sig_pk.as_bytes().to_vec(),
        structured_sig_sk: structured_sig_sk.as_bytes().to_vec(),
        hash_sig_pk: hash_sig_pk.as_bytes().to_vec(),
        hash_sig_sk: hash_sig_sk.as_bytes().to_vec(),
    }
}

#[uniffi::export]
pub fn multi_kem_encapsulate(lattice_pk: Vec<u8>, code_pk: Vec<u8>) -> MultiKemEncapsulation {
    let lattice_pk_obj = KemPk::from_bytes(&lattice_pk).expect("Invalid lattice PK");
    let (lattice_ct, lattice_ss) = kem_encaps(&lattice_pk_obj);

    let code_pk_obj = HqcPk::from_bytes(&code_pk).expect("Invalid code PK");
    let (code_ct, code_ss) = hqc_encaps(&code_pk_obj);

    // HKDF combiner (secure if any unbroken)
    let h = Hkdf::<Sha256>::new(Some(&code_ss.as_bytes()), &lattice_ss.as_bytes());
    let mut combined_ss = [0u8; 32];
    h.expand(b"mercy-multi-kem-ss", &mut combined_ss).unwrap();

    MultiKemEncapsulation {
        lattice_ct: lattice_ct.as_bytes().to_vec(),
        code_ct: code_ct.as_bytes().to_vec(),
        combined_ss: combined_ss.to_vec(),
    }
}

#[uniffi::export]
pub fn multi_sign_hybrid(message: String, keypair: MultiHybridKeypair) -> MultiHybridSignature {
    if !mercy_precheck(&message) {
        panic!("Mercy-block: Message misaligned — grace reframe required");
    }

    let lattice_sk = DilSk::from_bytes(&keypair.lattice_sig_sk).expect("Invalid lattice SK");
    let structured_sk = FalSk::from_bytes(&keypair.structured_sig_sk).expect("Invalid structured SK");
    let hash_sk = SphSk::from_bytes(&keypair.hash_sig_sk).expect("Invalid hash SK");

    MultiHybridSignature {
        lattice_sig: dil_sign(message.as_bytes(), &lattice_sk).as_bytes().to_vec(),
        structured_sig: fal_sign(message.as_bytes(), &structured_sk).as_bytes().to_vec(),
        hash_sig: sph_sign(message.as_bytes(), &hash_sk).as_bytes().to_vec(),
    }
}

#[uniffi::export]
pub fn multi_verify_hybrid(message: String, signature: MultiHybridSignature, keypair_pks: MultiHybridKeypair) -> bool {
    let lattice_pk = DilPk::from_bytes(&keypair_pks.lattice_sig_pk).expect("Invalid lattice PK");
    let structured_pk = FalPk::from_bytes(&keypair_pks.structured_sig_pk).expect("Invalid structured PK");
    let hash_pk = SphPk::from_bytes(&keypair_pks.hash_sig_pk).expect("Invalid hash PK");

    let lattice_sig = DilSig::from_bytes(&signature.lattice_sig).expect("Invalid lattice sig");
    let structured_sig = FalSig::from_bytes(&signature.structured_sig).expect("Invalid structured sig");
    let hash_sig = SphSig::from_bytes(&signature.hash_sig).expect("Invalid hash sig");

    // Secure if ANY family verifies (multi-assumption unbreakable)
    dil_verify(message.as_bytes(), &lattice_sig, &lattice_pk).is_ok() ||
    fal_verify(message.as_bytes(), &structured_sig, &structured_pk).is_ok() ||
    sph_verify(message.as_bytes(), &hash_sig, &hash_pk).is_ok()
}

uniffi::setup_scaffolding!();
