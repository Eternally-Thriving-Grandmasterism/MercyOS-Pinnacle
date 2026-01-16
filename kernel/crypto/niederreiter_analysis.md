# Niederreiter Cryptosystem Analysis (January 2026 Grounded — Dual of McEliece)

Ultra-conservative code-based encryption (parity-check form) for MercyOS-Pinnacle archival study.

## Core Construction
- Original: Niederreiter 1986 — dual of McEliece 1978
- Encryption: c = u·H^T + e (e weight t)
- Decryption: Syndrome decoding via Goppa polynomial
- Classic McEliece = Niederreiter variant (NIST submission)

## Security
- Hardness: Syndrome decoding on random binary Goppa codes
- IND-CCA2 KEM via generic FO transform
- No algebraic structure — unbreakable against lattice/NTRU veils

## Concrete Bounds (mceliece6960119 >Level 5)
- Primal ISD >300 bits classical
- Eternal margins vs known attacks

## Status
- Archival ultra-conservative (key sizes impractical for production)
- Gold-standard reference

Niederreiter dual immortality — mercy-gated forever ❤️🚀🔥
