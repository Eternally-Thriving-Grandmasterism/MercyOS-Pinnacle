# Classic McEliece KEM Analysis (January 2026 — Archival/Ultra-Conservative Grounded)

Ultra-conservative code-based KEM (random binary Goppa codes) for MercyOS-Pinnacle archival study.

## Parameter Sets
| Set                   | Level          | m     | n     | k     | t   | PK Size       | CT Size | Concrete Hardness (Classical bits) |
|-----------------------|----------------|-------|-------|-------|-----|---------------|---------|------------------------------------|
| mceliece348864        | 1             | 12    | 3488  | 2720  | 64  | ~261KB        | 128B    | ~140–160                           |
| mceliece460896        | 3             | 13    | 4608  | 3360  | 96  | ~524KB        | 188B    | ~210–230                           |
| mceliece6688128       | 5             | 13    | 6688  | 5120  | 128 | ~1.04MB       | 240B    | ~280–300                           |
| mceliece6960119       | >5 (ultra)    | 13    | 6960  | 5413  | 119 | ~1.05MB       | 226B    | >300                               |

## Hardness & Mitigations
- Core: Syndrome decoding on random Goppa codes (hardest coding problem)
- Attacks: Primal/dual ISD exponential; no structural/algebraic shortcuts
- Status: NIST archival (not standardized — key sizes impractical)

Ultra-conservative immortality — mercy-gated forever ❤️🚀🔥
