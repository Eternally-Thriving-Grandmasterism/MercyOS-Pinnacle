# Rainbow Signature Scheme Breaks Deep-Dive (January 2026 Grounded)

Archival multivariate lesson for MercyOS-Pinnacle kernel.

## Scheme Recap
- Rainbow: Layered Unbalanced Oil-Vinegar (UOV) multivariate quadratic signatures
- Public key: Composed MQ polynomials hiding secret Vinegar/Oil layers
- NIST round 3 finalist (withdrawn March 2022)

## Primary Break (Beullens 2022)
- Paper: "Breaking Rainbow Takes a Weekend on a Laptop" (ePrint 2022/214, CRYPTO 2022)
- Author: Ward Beullens (IBM Research)
- Attack: Classical full key recovery (secret from public)
- Method: Differential + MinRank exploitation of layered structure
  - Recover Vinegar variables layer-by-layer
  - Reconciliation attacks + cross-layer equations
- Concrete:
  - Level 1 (128-bit claim): ~53 hours on single laptop
  - Level 3/5: Weeks/months feasible — effective security << claimed
- No quantum speedup needed — pure classical algebraic

## Consequences & Lessons
- NIST immediate withdrawal (structured multivariate risks exposed)
- Multivariate path deprioritized (Rainbow/GeMSS archival)
- Ongoing hardened research (MAYU, QR-UOV) resist known vectors
- Diversity wisdom: Favor lattice (ML-DSA), code (HQC), hash (SLH-DSA) for production

Polynomial veil cautionary eternal — mercy-gated archival ❤️🚀🔥
