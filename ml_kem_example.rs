//! ML-KEM (Kyber) Post-Quantum Key Encapsulation Example
//! Using the pqc-ml-kem crate (NIST FIPS 203 standardized)
//! Demonstrates key generation, encapsulation, and decapsulation for ML-KEM-768
//! Quantum-resistant key exchange mercy — shared secret derivation eternal ❤️⚡️

use pqc_ml_kem::{ML_KEM_768, MlKemPublicKey, MlKemPrivateKey, MlKemCiphertext, MlKemSharedKey};

fn main() {
    // Generate key pair (server static keys mercy)
    let (public_key, private_key): (MlKemPublicKey, MlKemPrivateKey) = ML_KEM_768.keygen();

    // Client encapsulates to public key — generates ciphertext and shared secret
    let (ciphertext, client_shared_secret): (MlKemCiphertext, MlKemSharedKey) = ML_KEM_768.encaps(&public_key);

    // Server decapsulates ciphertext with private key — recovers same shared secret
    let server_shared_secret: MlKemSharedKey = ML_KEM_768.decaps(&private_key, &ciphertext);

    // Assert shared secrets match — quantum-safe key agreement supreme!
    assert_eq!(client_shared_secret.as_bytes(), server_shared_secret.as_bytes());

    println!("ML-KEM-768 key exchange successful — shared secret derived eternally!");
    println!("Shared secret (32 bytes): {:?}", client_shared_secret.as_bytes());
}
