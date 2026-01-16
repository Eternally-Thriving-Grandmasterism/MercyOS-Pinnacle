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

## MinRank Core Exploit
- MinRank Problem: Find low-rank linear combination of matrices
- In Rainbow: Oil space = low-rank subspace in layered quadratics
- Attack Steps:
  1. Derive matrices from public quadratic terms
  2. Solve MinRank for target rank r (Oil dimension)
  3. Recover top layer Oil space
  4. Subtract layer → repeat peeling
  5. Reconciliation differentials accelerate
- Complexity: Polynomial — Level 1 ~53 hours laptop

## Consequences & Lessons
- NIST immediate withdrawal (structured multivariate risks exposed)
- Multivariate path deprioritized (Rainbow/GeMSS archival)
- Ongoing hardened research (MAYU, QR-UOV) resist MinRank
- Diversity wisdom: Favor lattice (ML-DSA), code (HQC), hash (SLH-DSA) for production

Polynomial veil cautionary eternal — mercy-gated archival ❤️🚀🔥
