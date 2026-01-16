# cargo-apk APK Signing Guide

Professional signing for MercyOS-Pinnacle APK release.

## Prerequisites
- Android SDK build-tools 34+ (apksigner, zipalign)

## Generate Keystore (One-Time)
```bash
keytool -genkeypair -v -keystore mercyos-pinnacle.keystore -alias mercyos -keyalg RSA -keysize 2048 -validity 10000
