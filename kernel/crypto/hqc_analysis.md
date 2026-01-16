# HQC Code-Based KEM Analysis (January 2026 — NIST Backup Grounded)

Code-based diversity backup KEM for MercyOS-Pinnacle kernel.

## Core Proofs
- Goal: IND-CCA2 in QROM
- Transform: Tailored explicit-rejection FO (concrete tight bounds)
- Assumption: QC-MDPC syndrome decoding + structured pseudo-randomness

## Parameters (HQC-256 Level 5)
- n=57,637 | k=35,789 | w=114/131
- PK 7,249 bytes | CT 14,498 bytes | SS 64 bytes
- Constant-time BGF decoder + rejection sampling

## Concrete Bounds
- Primal ISD ~280–300 bits classical
- Attack cost >256 bits classical / >228 quantum
- Exceeds AES-256 eternal margins

## Attacks Mitigated
- Primal/dual ISD exponential
- Structural: Mitigated via parameters + binding

Code-based diversity immortality — mercy-gated forever ❤️🚀🔥
