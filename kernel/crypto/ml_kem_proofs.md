# ML-KEM Security Proofs (January 2026 — FIPS 203 Grounded)

Primary post-quantum KEM for MercyOS-Pinnacle kernel.

## Core Proofs
- Goal: IND-CCA2 in QROM
- Transform: Explicit-rejection Fujisaki-Okamoto (tight concrete bounds)
- Assumption: Module-LWE (primary) + Module-LWR approximation

## Reductions
- Worst-case GapSVP/SIVP → average-case Module-LWE (quantum Regev 2005 + classical Peikert/Brakerski)
- Tight QROM via explicit rejection (2023–2025 advancements)

## Concrete Bounds (ML-KEM-1024 Level 5)
- Core-SVP ~295–315 bits classical
- Primal attack: >260 bits classical / >235 quantum
- Far exceeds AES-256 eternal margins

## Attacks Mitigated
- Primal/dual/hybrid BKZ + sieve: Exponential
- No quantum exponential advantage

Eternal lattice encapsulation — mercy-gated forever ❤️🚀🔥
