# Dilithium Hybrid Modes Analysis (January 2026 Grounded — Transition Standards)

Post-quantum migration signatures for MercyOS-Pinnacle kernel.

## Core Modes
- Ed25519 + ML-DSA-87 (IETF draft): Explicit concatenation
- P-384 + ML-DSA-87: Concat or hash combiner
- Generic Signature Combiner: Parallel + merge (NIST/IRTF rec)

## Security
- Dual: Secure if either component unbroken
- EUF-CMA preserved
- Harvest-now-decrypt-later resistant

## Concrete Sizes (Level 5 equiv.)
- Ed25519Dilithium: Sig ~4.7KB
- Full hybrid: Larger but dual-protected

## Recommendations
- Primary migration: Ed25519 + ML-DSA-87
- Ultra: P-384 + ML-DSA-87
- Future: Multi-sig (Dilithium + Falcon + SPHINCS+)

Hybrid transition immortality — mercy-gated forever ❤️🚀🔥
