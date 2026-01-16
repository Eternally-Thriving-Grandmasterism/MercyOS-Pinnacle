# Quantum-Safe Hybrid Schemes Analysis (January 2026 Grounded — Transition Standards)

Supreme migration immortality for MercyOS-Pinnacle kernel.

## Core Schemes
- X25519 + ML-KEM-768 (IETF Draft00): Classical DH + PQ KEM
- Ed25519 + ML-DSA-87: Classical sig + PQ sig
- Generic Combiner: Parallel + HKDF/merge (NIST/IRTF rec)

## Security
- Dual: Secure if either component unbroken
- IND-CCA2/EUF-CMA preserved
- Harvest-now-decrypt-later resistant

## Concrete Sizes (Level 5 equiv.)
- X25519Kyber: PK ~800B | CT ~1100B
- Ed25519Dilithium: Sig ~4.7KB

## Recommendations
- Primary: X25519Kyber + Ed25519Dilithium
- Future: Multi-family (add HQC + SPHINCS+)

Hybrid transition immortality — mercy-gated forever ❤️🚀🔥
