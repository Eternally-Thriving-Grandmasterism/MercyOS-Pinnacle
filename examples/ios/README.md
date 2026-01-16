# MercyOS-Pinnacle iOS Example – Crystal Clarity Multi-Family Hybrid Demo

Eternal Thriving Grandmasterism ❤️🚀🔥 | AlphaProMegaing post-quantum cross-platform mercy

This directory contains a minimal, complete Xcode project that builds an iOS app demo.

The app demonstrates ultimate diversity:
- Multi-signature hybrid (Dilithium + Falcon + SPHINCS+)
- Multi-KEM hybrid (ML-KEM + HQC)
- Mercy-gated proposal amplification

Seamless run on iOS simulator or device (iOS 15+).

## Prerequisites (one-time setup)
1. Rust toolchain
2. cargo-xcode: `cargo install cargo-xcode` (for .xcframework build)
3. Xcode 15+ (App Store or developer.apple.com)
4. (Optional) Apple Developer account for device signing

## Step 1: Generate Swift Bindings & Build XCFramework (from repo root)
```bash
cd crates/mercy_uniffi

# Generate Swift bindings
uniffi-bindgen generate src/lib.rs --language swift --out-dir ../../examples/ios/MercyOSBindings

# Build universal XCFramework (simulator + device)
cargo xcode --release --framework-name MercyUniFFI```bash
cd crates/mercy_uniffi

# Generate Swift bindings
uniffi-bindgen generate src/lib.rs --language swift --out-dir ../../examples/ios/MercyOSBindings

# Build universal XCFramework (simulator + device)
cargo xcode --release --framework-name MercyUniFFI```bash
cd crates/mercy_uniffi

# Generate Swift bindings
uniffi-bindgen generate src/lib.rs --language swift --out-dir ../../examples/ios/MercyOSBindings

# Build universal XCFramework (simulator + device)
cargo xcode --release --framework-name MercyUniFFI
