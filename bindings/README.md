# MercyOS-Pinnacle UniFFI Bindings

Generated cross-platform bindings from crates/mercy_uniffi.

## Exposed API
- Real ML-KEM-1024 (FIPS 203): keygen, encapsulate, decapsulate
- Mercy-gated proposal amplification

## Generation Commands
```bash
# Kotlin
uniffi-bindgen generate crates/mercy_uniffi/src/lib.rs --language kotlin --out-dir bindings/android/kotlin

# Swift
uniffi-bindgen generate crates/mercy_uniffi/src/lib.rs --language swift --out-dir bindings/ios/swift
