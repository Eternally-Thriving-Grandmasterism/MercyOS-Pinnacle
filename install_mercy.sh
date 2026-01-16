#!/bin/bash
# MercyOS-Pinnacle Ultimate Easy Install Sacred
# AlphaProMegaing one-command ascension joy eternal ❤️🚀🔥

set -e

echo "MercyOS-Pinnacle ascension beginning—mercy overrides scarcity supreme immaculate ❤️"

# Auto-detect platform
if [[ "$OSTYPE" == "linux-android" ]]; then
    PLATFORM="android"
elif [[ "$OSTYPE" == "darwin"* ]]; then
    PLATFORM="macos"
elif [[ "$OSTYPE" == "linux-gnu"* ]]; then
    PLATFORM="linux"
elif [[ "$OSTYPE" == "msys" || "$OSTYPE" == "win32" ]]; then
    PLATFORM="windows"
else
    PLATFORM="unknown"
fi

echo "Platform detected: $PLATFORM"

# Clean old/nested mercy
if [ -d "MercyOS-Pinnacle" ]; then
    echo "Cleaning old ascension..."
    rm -rf MercyOS-Pinnacle
fi

# Fresh clone eternal
git clone https://github.com/Eternally-Thriving-Grandmasterism/MercyOS-Pinnacle.git
cd MercyOS-Pinnacle

# Platform-specific mercy
if [ "$PLATFORM" == "android" ]; then
    echo "Android mercy—Termux native build joy"
    pkg update -y
    pkg install rust clang make libllvm openssl zlib git -y
    cargo install cargo-ndk
    rustup target add aarch64-linux-android
    # NDK manual if needed (prompt user)
    cargo ndk -t aarch64-linux-android build --release
    echo "Native libs built—APK ready in examples/android"
else
    echo "Desktop/Server mercy—native build"
    cargo build --release
fi

echo "MercyOS-Pinnacle ascension complete—run joy eternal! ❤️🚀🔥"
echo "Next: cargo run --release --bin mercy_print -- --help"
