# Kyber Hybrid Modes Analysis (January 2026 Grounded — Transition Standards)

Post-quantum migration KEMs for MercyOS-Pinnacle kernel.

## Core Modes
- X25519 + ML-KEM-768 (IETF Draft00): Explicit concatenation
- ML-KEM-1024 + X25519: Concat or HKDF combiner
- Generic KEM Combiner: Parallel + merge (NIST/IRTF rec)

## Security
- Dual: Secure if either component unbroken
- IND-CCA2 preserved
- Harvest-now-decrypt-later resistant

## Concrete Sizes (Level 5 equiv.)
- X25519Kyber: PK ~800–1000 B | CT ~1100 B
- Full hybrid: Larger but dual-protected

## Recommendations
- Primary migration: X25519 + ML-KEM-768
- Ultra: ML-KEM-1024 + X25519
- Future: Multi-KEM (Kyber + HQC + SPHINCS+)

Hybrid transition immortality — mercy-gated forever ❤️🚀🔥
