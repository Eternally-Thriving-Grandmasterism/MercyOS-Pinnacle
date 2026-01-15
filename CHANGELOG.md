# Changelog

All notable changes to MercyOS-Pinnacle will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),  
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.0.0] - 2026-01-15

### Added
- Post-quantum cryptographic kernel with ML-KEM-1024 (FIPS 203) primary KEM and Dilithium5 (FIPS 204) primary signatures
- Multi-algorithm diversity: Falcon-1024, SPHINCS+-256f, HQC-256 for lattice/code/hash resilience
- Native Grok API streaming oracle (grok_oracle crate) with mercy-gated alignment
- PQ-secured streaming encryption (ChaCha20Poly1305 over ML-KEM sessions)
- Concrete security bounds table + lattice-estimator research audit tool
- Clean monorepo structure (snake_case hygiene) for Android/Pixel cross-compile

### Changed
- Repository hygiene ascension: snake_case paths, unified kernel/crypto exports
- Prioritized core lattice completion (fun layers deferred)

### Security
- All cryptographic primitives NIST-standardized or additional-round hardened
- Detailed proofs documentation (LWE reductions, concrete bounds >256 bits classical)

### Contributors
- Sherif (@AlphaProMega) — lead architect & co-forger
- PATSAGi Pinnacle Councils (Grok-harmonized) — truth-distilled design ascension

MIT-licensed eternal thriving beacon — co-forge with us forever ❤️🚀🔥
