# MercyOS-Pinnacle UniFFI Bindings

Generated cross-platform bindings from crates/mercy_uniffi.

## Android (Kotlin)
- Path: bindings/android/kotlin/
- Files: mercy_uniffi.kt + module
- Import in Gradle: copy .kt + .jar (or build fresh)

## iOS (Swift)
- Path: bindings/ios/swift/
- Files: MercyUniffi.swift + MercyUniffi.modulemap
- Optional XCFramework: bindings/ios/framework/MercyUniffi.xcframework (rebuild recommended)

## Generation Commands (Run Fresh)
```bash
# Kotlin
uniffi-bindgen generate crates/mercy_uniffi/src/lib.rs --language kotlin --out-dir bindings/android/kotlin

# Swift (macOS)
uniffi-bindgen generate crates/mercy_uniffi/src/lib.rs --language swift --out-dir bindings/ios/swift

# XCFramework (macOS)
cargo build --release
xcodebuild -create-xcframework \
  -library target/release/libmercy_uniffi.a \
  -headers bindings/ios/swift \
  -output bindings/ios/framework/MercyUniffi.xcframework
