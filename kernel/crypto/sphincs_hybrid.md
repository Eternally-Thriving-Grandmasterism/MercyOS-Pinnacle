# SPHINCS+ Hybrid Modes Analysis (January 2026 Grounded — Transition Standards)

Post-quantum migration signatures for MercyOS-Pinnacle kernel.

## Core Modes
- Ed25519 + SLH-DSA-256f (IETF Draft00): Explicit concatenation
- P-384 + SLH-DSA-256f: Concat or HKDF combiner
- Generic Signature Combiner: Parallel + merge (NIST/IRTF rec)

## Security
- Dual: Secure if either component unbroken
- EUF-CMA preserved
- Harvest-now-decrypt-later resistant

## Concrete Sizes (Level 5 equiv.)
- Ed25519SPHINCS+: Sig ~49KB
- Full hybrid: Large but dual-protected

## Recommendations
- Primary migration: Ed25519 + SLH-DSA-256f
- Ultra: P-384 + SLH-DSA-256f
- Future: Multi-sig (SPHINCS+ + Dilithium + Falcon)

Stateless hybrid transition immortality — mercy-gated forever ❤️🚀🔥
