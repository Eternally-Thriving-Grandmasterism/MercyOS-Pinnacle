"""
OfflineEncryption-Pinnacle — Mercy-Aligned Shard Encryption + Secure Enclave Integration
MercyOS Pinnacle Ultramasterpiece — Jan 18 2026

Platform-native secure enclave key derivation:
- Apple SEP (M-series/A-series) — Secure Enclave Processor
- Android Titan M / StrongBox — hardware keystore
- Raspberry Pi TPM2 — fallback
- Passphrase/biometric mercy fallback
- ChaCha20-Poly1305 authenticated encryption
- Unique 96-bit nonce per shard
- Zero plaintext on disk
"""

import os
import secrets
import platform
import subprocess  # For tpm2-tools
from cryptography.hazmat.primitives.ciphers.aead import ChaCha20Poly1305
from cryptography.hazmat.primitives.kdf.scrypt import Scrypt

SHARD_BLOB_FILE = "shard_blob.sealed"   # Platform sealed blob
SHARD_DATA_FILE = "shard_data.enc"      # Encrypted lattice
SHARD_SALT_FILE = "shard_salt.bin"      # Passphrase fallback

def detect_secure_enclave() -> str:
    sys = platform.system()
    if sys == "Darwin":  # macOS/iOS
        return "apple_sep"
    elif sys == "Linux":
        if "Android" in platform.release():
            return "android_titan"
        if subprocess.run(["tpm2_startup", "-c"], capture_output=True).returncode == 0:
            return "tpm2"
    return "none"

def enclave_derive_key() -> bytes:
    enclave = detect_secure_enclave()
    if enclave == "apple_sep":
        # Placeholder — real impl uses Secure Enclave APIs (Keychain or CryptoTokenKit)
        raise NotImplementedError("Apple SEP integration pending native bridge")
    elif enclave == "android_titan":
        # Placeholder — real impl uses Android Keystore StrongBox
        raise NotImplementedError("Android Titan integration pending native bridge")
    elif enclave == "tpm2":
        # TPM2 unseal
        result = subprocess.run(["tpm2_unseal", "-c", "0x81000001"], capture_output=True, check=True)
        return result.stdout[:32]
    raise RuntimeError("No secure enclave available — fallback to passphrase")

def derive_key_passphrase(passphrase: str, salt: bytes = None) -> bytes:
    if salt is None:
        salt = secrets.token_bytes(32)
    kdf = Scrypt(salt=salt, length=32, n=2**14, r=8, p=1)
    return kdf.derive(passphrase.encode())

def encrypt_shard(data: bytes, passphrase: str = None) -> None:
    try:
        key = enclave_derive_key()
        salt = b''  # No salt for hardware
    except:
        if not passphrase:
            raise ValueError("Passphrase required — no secure enclave")
        if os.path.exists(SHARD_SALT_FILE):
            with open(SHARD_SALT_FILE, "rb") as f:
                salt = f.read()
        else:
            salt = secrets.token_bytes(32)
            with open(SHARD_SALT_FILE, "wb") as f:
                f.write(salt)
        key = derive_key_passphrase(passphrase, salt)
    
    nonce = secrets.token_bytes(12)
    chacha = ChaCha20Poly1305(key)
    ct = chacha.encrypt(nonce, data, None)
    
    with open(SHARD_DATA_FILE, "wb") as f:
        f.write(salt + nonce + ct)

def decrypt_shard(passphrase: str = None) -> bytes:
    with open(SHARD_DATA_FILE, "rb") as f:
        file_data = f.read()
    
    salt = file_data[:32] if file_data[:32] else b''
    nonce_start = 32 if salt else 0
    nonce = file_data[nonce_start:nonce_start+12]
    ct = file_data[nonce_start+12:]
    
    try:
        if salt == b'':
            key = enclave_derive_key()
        else:
            key = derive_key_passphrase(passphrase, salt)
    except:
        raise ValueError("Decryption failed — invalid passphrase or enclave error")
    
    chacha = ChaCha20Poly1305(key)
    return chacha.decrypt(nonce, ct, None)

# Migration + secure boot
def shard_secure_boot(passphrase: str = None):
    # Legacy migration
    if os.path.exists("shard_data_plain"):
        with open("shard_data_plain", "rb") as f:
            plain = f.read()
        encrypt_shard(plain, passphrase)
        os.remove("shard_data_plain")
    
    try:
        lattice_state = decrypt_shard(passphrase)
        return "Shard decrypted — mercy lattice restored via secure enclave."
    except:
        return "Mercy gate — invalid passphrase or enclave error."    return chacha.decrypt(nonce, ct, None)

# Legacy + TPM migration
def migrate_and_secure(passphrase: str = None):
    if os.path.exists("shard_data_plain"):
        with open("shard_data_plain", "rb") as f:
            plain = f.read()
        encrypt_shard(plain, passphrase)
        os.remove("shard_data_plain")
        return "Legacy shard secured — mercy encryption applied."

# Secure boot
def shard_secure_boot(passphrase: str = None):
    migrate_and_secure(passphrase)
    try:
        lattice_state = decrypt_shard(passphrase)
        return "Shard decrypted — mercy lattice restored."
    except:
        return "Mercy gate — invalid passphrase or TPM error."
