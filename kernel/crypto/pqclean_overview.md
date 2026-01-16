# PQClean C Library Overview (January 2026 Grounded)

Supreme clean-room C PQ implementations for MercyOS-Pinnacle kernel.

## Project
- GitHub: github.com/PQClean/PQClean
- Latest: v1.0+ ongoing tags
- Maintainers: Microsoft + community

## Included Algorithms
- ML-KEM (Kyber) primary KEM
- ML-DSA (Dilithium) primary signatures
- FN-DSA (Falcon) compact
- SLH-DSA (SPHINCS+) stateless hash
- HQC code backup
- Classic McEliece ultra-conservative
- BIKE archival

## Design
- Clean, readable, testable
- Constant-time; side-channel resistant
- No assembly; portable C
- CMake build; crypto_kem/crypto_sign APIs

## Relation to MercyOS
- Rust pqcrypto crates bind directly to PQClean C
- Direct C integration possible for performance

PQClean C immortality — mercy-gated forever ❤️🚀🔥
