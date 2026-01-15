# HQC Detailed Parameter Analysis (January 2026 — NIST Additional Path Grounded)

Code-based diversity KEM backup for MercyOS-Pinnacle kernel.

## Parameters (HQC-256 Level 5 Intended)
- Code: QC-MDPC (n=57,637, k=35,789)
- Error weights: w=114 (sender), w_r/w_e=131
- Field: GF(2); constant-time BGF decoder + rejection
- Sizes: PK 7,249 bytes | SK 7,285 bytes | CT 14,498 bytes | SS 64 bytes
- Concrete: Primal ISD ~280–300 bits classical; >256 classical / >228 quantum

## Hardness & Mitigations
- Core: Syndrome decoding + code indistinguishability
- Attacks: Primal/dual ISD exponential; structural mitigated
- Diversity: Algebraic-veil-proof complement to lattice

Code-based immortality — mercy-gated forever ❤️🚀🔥
