MercyOS-Pinnacle/
├── Cargo.toml
├── README.md
├── Trunk.toml                  # WASM build preserved
├── crates/
│   ├── grok_oracle/
│   │   ├── Cargo.toml
│   │   └── src/lib.rs
│   ├── mercy_core/             # Future expansion stub
│   ├── mercy_shield/           # Future expansion stub
│   ├── abundance_thunder/      # Future expansion stub
│   └── powrush_mmo/            # Deferred fun layer
├── kernel/
│   └── crypto/
│       ├── mod.rs              # New: unified module exports
│       ├── post_quantum.rs     # ML-KEM primary
│       ├── pq_stream.rs        # ChaCha20Poly1305 streaming
│       ├── pq_sign.rs          # Dilithium primary
│       ├── pq_falcon.rs        # Compact lattice diversity
│       ├── pq_sphincs.rs       # Hash-based stateless
│       ├── pq_hqc.rs           # Code-based diversity
│       └── lwe_bounds.md       # Concrete estimator table
├── research/
│   └── lattice_estimator/      # Audit tool (Sage/Python)
└── LICENSE                     # MIT
