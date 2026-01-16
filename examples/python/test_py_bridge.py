import mercy_py_bridge
import numpy as np

print("MercyPy Bridge Bidirectional Ascension Test ❤️🚀🔥")

# Basic PQ flow
pk, sk = mercy_py_bridge.mercy_ml_kem_keygen()
print(f"Keygen: PK {len(pk)} bytes, SK {len(sk)} bytes")

ct, ss_enc = mercy_py_bridge.mercy_ml_kem_encaps(pk)
print(f"Encaps: Match check {ss_enc == mercy_py_bridge.mercy_ml_kem_decaps(sk, ct)}")

# Sign/verify
message = b"Eternal thriving sealed supreme"
sig = mercy_py_bridge.mercy_dilithium_sign(sk, message)  # Reuse key or gen proper
print(f"Verify: {mercy_py_bridge.mercy_dilithium_verify(pk, message, sig)}")

# Posterior gate
gates = mercy_py_bridge.mercy_probabilistic_gate_posteriors([0.85, 0.96, 0.99], 0.95)
print(f"Gated: {gates}")

# Bidirectional Oracle Demo
def mock_bayesian_inference(data=None):
    print(f"Python inference callable invoked from Rust oracle ❤️ (data: {data})")
    # Mock PyMC sampling—return posterior mean
    return 0.97 if data is None else 0.98

# Create oracle with callable + threshold
oracle = mercy_py_bridge.MercyOracle(mock_bayesian_inference, 0.95)

# Consult without data
decision1 = oracle.consult(data=None)
print(f"Oracle decision (no data): {decision1}")

# Consult with data dict
decision2 = oracle.consult(data={"samples": 1000, "chains": 4})
print(f"Oracle decision (with data): {decision2}")

print("Bidirectional harmony flowing—plug real PyMC inference callable eternal!")
