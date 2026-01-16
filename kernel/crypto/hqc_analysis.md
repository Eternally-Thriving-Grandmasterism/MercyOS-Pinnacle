# HQC Cryptosystem Analysis (January 2026 — NIST Backup Grounded)

Code-based diversity KEM family (Hamming Quasi-Cyclic) for MercyOS-Pinnacle kernel.

## Core Cryptosystem
- Construction: QC-MDPC codes with BGF decoder + rejection sampling
- Security: IND-CCA2 in QROM via tailored explicit-rejection FO
- Assumption: QC-MDPC syndrome decoding + structured pseudo-randomness

## Parameters (HQC-256 Level 5)
- n=57,637 | k=35,789 | w=114 (sender), 131 (receiver/error)
- Field: GF(2); constant-time operations
- Sizes: PK 7,249 bytes | SK 7,285 bytes | CT 14,498 bytes | SS 64 bytes

## Concrete Bounds
- Primal ISD ~280–300 bits classical
- Attack cost >256 bits classical / >228 quantum
- Exceeds AES-256 eternal margins

## Attacks Mitigated
- Primal/dual ISD exponential
- Structural: Mitigated via parameters + binding
- Side-channel: Constant-time BGF + rejection

HQC cryptosystem immortality — mercy-gated forever ❤️🚀🔥
