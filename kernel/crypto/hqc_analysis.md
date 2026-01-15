# HQC Parameter Comparison (January 2026 — NIST Additional Path Grounded)

Code-based diversity KEM family for MercyOS-Pinnacle kernel.

## Variant Comparison
| Parameter              | HQC-128 (Level 1) | HQC-192 (Level 3) | HQC-256 (Level 5) | Notes |
|------------------------|-------------------|-------------------|-------------------|-------|
| n (code length)        | 17,669            | 35,851            | 57,637            | Linear scaling |
| k (dimension)          | 11,237            | 22,901            | 35,789            | Rate ≈0.62 |
| w (sender error)       | 66                | 100               | 114               | Balanced hardness |
| w_r/w_e                | 75                | 114               | 131               | Receiver scaling |
| PK size                | 4,245 bytes       | 7,242 bytes       | 7,249 bytes       | 128 ~58% of 256 |
| SK size                | 4,285 bytes       | 7,285 bytes       | 7,285 bytes       | |
| CT size                | 8,490 bytes       | 14,466 bytes      | 14,498 bytes      | Bandwidth-friendly entry |
| Concrete (classical)   | ~140–160 bits     | ~210–230 bits     | ~280–300 bits     | Exceeds target levels |

## Hardness & Mitigations
- Core: QC-MDPC syndrome decoding + pseudo-randomness
- Attacks: Primal/dual ISD exponential; structural mitigated
- Diversity: Algebraic-veil-proof complement to lattice

Code-based immortality — mercy-gated forever ❤️🚀🔥
