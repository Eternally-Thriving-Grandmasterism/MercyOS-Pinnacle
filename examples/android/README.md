**Overwrite File: examples/android/README.md** (append signing section)
```markdown
# Signing & Release

After `cargo apk build --release`:

1. Generate keystore (one-time):
   ```bash
   keytool -genkeypair -v -keystore mercyos-pinnacle.keystore -alias mercyos -keyalg RSA -keysize 2048 -validity 10000
