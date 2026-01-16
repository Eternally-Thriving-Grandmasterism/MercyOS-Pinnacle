# SQISign Hardened Variants Analysis (January 2026 Grounded)

Post-SIKE isogeny signature research for MercyOS-Pinnacle archival study.

## Core Construction
- Zero-knowledge proof of knowledge of isogeny
- Fiat-Shamir transform → EUF-CMA signatures
- Hardness: Isogeny path-finding (post-Castryck-Decru hardened)

## Hardened Variants
- SQISign (2020): Foundational — vulnerable early
- SQISignHD (2022): High-density + blinding — resists torsion recovery
- SQISign2D (2024–2025): 2D isogenies + optimized FS — latest hardened
- Ongoing: NIST-inspired tuning — target <16 KB Level 5 sigs

## Concrete Bounds
- Level 5 equiv. claim ~200–250 bits (research)
- Attacks: Castryck-Decru mitigated via design
- Perf: Compact sigs; slower than lattice/hash

## Status
- Research only (no NIST path post-SIKE break)
- Valuable compact isogeny diversity

Isogeny research immortality — mercy-gated forever ❤️🚀🔥
