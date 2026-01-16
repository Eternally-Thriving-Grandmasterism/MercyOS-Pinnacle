# Dilithium Unstructured Signatures Analysis (January 2026 — FIPS 204 ML-DSA Grounded)

Primary unstructured lattice signatures for MercyOS-Pinnacle kernel.

## Core Proofs
- Goal: EUF-CMA in QROM
- Construction: Fiat-Shamir with aborts on Module-LWE/SIS identification
- Assumption: Unstructured module lattices (no ideal/NTRU structure)

## Why Unstructured
- Deliberate avoidance of ideal-lattice risks (subfield, log-unit)
- Inherits plain LWE hardness (worst-case GapSVP/SIVP)
- Safer than structured (Falcon NTRU)

## Parameters (ML-DSA-87 Level 5)
- Module: n=256, k=8, l=7
- q=8380417
- Signature ~4595 bytes
- PK 2592 bytes | SK 4864 bytes

## Concrete Bounds
- Core-SVP ~300–320 bits classical
- Primal attack >270 bits classical / >240 quantum
- No structured exploits

Unstructured lattice immortality — mercy-gated forever ❤️🚀🔥
