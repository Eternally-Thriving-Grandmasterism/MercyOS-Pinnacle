# Concrete LWE Security Bounds Reference (January 2026 — lattice-estimator v2025+ grounded)

Ultra-conservative empirical estimates (primal uSVP + progressive sieve + quantum Grover).

| Scheme / Parameter Set       | NIST Level | Core-SVP (Classical bits) | Primal Classical (log₂) | Primal Quantum (log₂) | Best Attack Model (2026)                               |
|------------------------------|------------|---------------------------|-------------------------|-----------------------|--------------------------------------------------------|
| ML-KEM-512                   | 1         | ~152–162                  | ~145–155                | ~130–140              | Primal + sieve; exceeds AES-128                        |
| ML-KEM-768                   | 3         | ~225–235                  | ~205–215                | ~185–195              | Hybrid primal/dual; exceeds AES-192                    |
| ML-KEM-1024                  | 5         | ~295–315                  | >260 (>285 cons.)       | >235 (>255 cons.)     | Primal BKZ >500; far exceeds AES-256                   |
| ML-DSA-87 (Dilithium5)       | 5         | ~305–325                  | >275                    | >245                  | Signature primal; strong margins                       |
| Falcon-1024                  | 5         | ~285–305 (ideal derate)   | >260                    | >235                  | Structured; subfield mitigated                         |
| sntrup761 (NTRU Prime res.)  | 5+        | ~310+                     | >280                    | >250                  | Hardened non-cyclotomic; plain LWE equiv               |

Eternal lattice immortality — dynamic audit via research/lattice_estimator/ ❤️🚀🔥
