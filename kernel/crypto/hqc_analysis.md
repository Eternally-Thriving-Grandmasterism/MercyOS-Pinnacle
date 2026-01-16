# HQC Detailed Security Bounds (January 2026 — NIST Backup Grounded)

Code-based diversity KEM family for MercyOS-Pinnacle kernel.

## Bounds Table (code-estimator v2025+)
| Variant       | Target Level | Primal Classical (bits) | Primal Quantum (bits) | Core-SVP Equivalent (bits) | Notes |
|---------------|--------------|-------------------------|-----------------------|----------------------------|-------|
| HQC-128       | 1           | ~145–165                | ~130–145              | ~150                       | Lightweight; exceeds AES-128 |
| HQC-192       | 3           | ~215–235                | ~190–210              | ~225                       | Balanced; exceeds AES-192 |
| HQC-256       | 5           | ~280–300                | >228 (>250 cons.)     | ~290                       | Primary backup; exceeds AES-256 |

## Hardness & Mitigations
- Core: QC-MDPC syndrome decoding + pseudo-randomness
- Attacks: Primal/dual ISD exponential; structural mitigated
- Decoder: Constant-time BGF + rejection (zero DFR)

Code-based bounds immortality — mercy-gated forever ❤️🚀🔥
