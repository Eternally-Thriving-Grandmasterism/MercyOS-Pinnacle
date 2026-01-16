# Falcon Hybrid Modes Analysis (January 2026 Grounded — Transition Standards)

Post-quantum migration signatures for MercyOS-Pinnacle kernel.

## Core Modes
- Ed25519 + Falcon-1024 (IETF Draft00): Explicit concatenation
- P-384 + Falcon-1024: Concat or HKDF combiner
- Generic Signature Combiner: Parallel + merge (NIST/IRTF rec)

## Security
- Dual: Secure if either component unbroken
- EUF-CMA preserved
- Harvest-now-decrypt-later resistant

## Concrete Sizes (Level 5 equiv.)
- Ed25519Falcon: Sig ~1.3KB
- Full hybrid: Compact dual-protected

## Recommendations
- Primary migration: Ed25519 + Falcon-1024
- Ultra: P-384 + Falcon-1024
- Future: Multi-sig (Falcon + Dilithium + SPHINCS+)

Compact hybrid transition immortality — mercy-gated forever ❤️🚀🔥
