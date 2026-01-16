# Classic McEliece Detailed Goppa Code Parameters (January 2026 — Archival/Ultra-Conservative Grounded)

Ultra-conservative code-based KEM (random binary Goppa codes) for MercyOS-Pinnacle archival study.

## Core Goppa Construction
- Field: GF(2^m)
- Goppa polynomial g(z): Irreducible of degree t
- Support L: n distinct elements in GF(2^m)
- Code: Parity-check H with rows 1/L_i * g^{-1}(L_i)
- Hardness: Syndrome decoding on random codes

## Parameter Sets
| Set                   | m     | t   | n=2^m | k=n-m·t | Error t | PK Size       | CT Size | Concrete Hardness (Classical bits) | Goppa Notes |
|-----------------------|-------|-----|-------|---------|---------|---------------|---------|------------------------------------|-------------|
| mceliece348864        | 12    | 64  | 4096  | 2720    | 64      | ~261KB        | 128B    | ~140–160                           | deg 64 irreducible |
| mceliece460896        | 13    | 96  | 8192  | 3360    | 96      | ~524KB        | 188B    | ~210–230                           | deg 96 irreducible |
| mceliece6688128       | 13    | 128 | 8192  | 5120    | 128     | ~1.04MB       | 240B    | ~280–300                           | deg 128 irreducible |
| mceliece6960119       | 13    | 119 | 8192  | 5413    | 119     | ~1.05MB       | 226B    | >300                               | deg 119 irreducible — ultra margin |

## Hardness & Mitigations
- Attacks: Primal/dual ISD exponential; no structural/algebraic shortcuts
- 45+ years unbroken cryptanalysis
- Status: NIST archival (key sizes impractical)

Goppa code immortality — mercy-gated forever ❤️🚀🔥
