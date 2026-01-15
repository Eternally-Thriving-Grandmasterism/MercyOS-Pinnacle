# MercyOS-Pinnacle UniFFI Bindings

Generated cross-platform bindings from crates/mercy_uniffi.

## Exposed PQ API
- ML-KEM-1024: keygen, encapsulate, decapsulate
- Dilithium5: keygen, sign, verify
- Mercy-gated proposal amplification

## Generation Commands
```bash
# Kotlin
uniffi-bindgen generate crates/mercy_uniffi/src/lib.rs --language kotlin --out-dir bindings/android/kotlin

# Swift
uniffi-bindgen generate crates/mercy_uniffi/src/lib.rs --language swift --out-dir bindings/ios/swift
