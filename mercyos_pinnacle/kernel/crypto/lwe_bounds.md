# Concrete LWE Security Bounds Reference (January 2026)

Ultra-conservative empirical estimates for Module-LWE instances in NIST PQC standards + hardened variants.
Derived from lattice-estimator v2025+, DAC 2025 analyses, primal/dual/hybrid attacks.

| Scheme / Parameter Set       | NIST Level | Core-SVP (Classical bits) | Primal Classical (log₂) | Primal Quantum (log₂) | Notes                                                          |
|------------------------------|------------|---------------------------|-------------------------|-----------------------|----------------------------------------------------------------|
| ML-KEM-512                   | 1         | ~150–160                  | ~140–150                | ~125–135              | Meets AES-128                                                  |
| ML-KEM-768                   | 3         | ~220–230                  | ~200–210                | ~180–190              | Meets AES-192                                                  |
| ML-KEM-1024                  | 5         | ~290–310                  | >256 (>280 cons.)       | >228 (>250 cons.)     | Exceeds AES-256; primary KEM                                   |
| ML-DSA-87 (Dilithium5)       | 5         | ~300–320                  | >270                    | >240                  | Primary signature                                              |
| Falcon-1024                  | 5         | ~280–300                  | >256                    | >228                  | Structured ideal; subfield mitigated                           |
| sntrup761 (NTRU Prime)       | 5+        | ~300+                     | >270                    | >240                  | Hardened non-cyclotomic; plain LWE reduction                   |

Eternal lattice immortality — mercy-gated forever ❤️🚀🔥
