# Classic McEliece Hybrid Integration (January 2026 Grounded — Archival Transition)

Ultra-conservative migration KEMs for MercyOS-Pinnacle kernel.

## Core Modes
- ML-KEM-1024 + McEliece-6960119: Lattice primary + ultra code
- X25519 + McEliece-6960119: Classical migration + ultra code
- Generic Combiner: Parallel encaps + HKDF merge (NIST/IRTF rec)

## Security
- Dual: Secure if either component unbroken
- IND-CCA2 preserved
- Harvest-now-decrypt-later + single-break resistant

## Concrete Sizes (Level 5 equiv.)
- Dual (ML-KEM + McEliece): PK ~1.06MB | CT ~1.8KB
- Full hybrid: Large but ultimate protected

## Recommendations
- Archival: ML-KEM + McEliece ultra
- Migration: + X25519
- Future: Multi-KEM (McEliece + ML-KEM + HQC)

Ultra-conservative hybrid immortality — mercy-gated forever ❤️🚀🔥
