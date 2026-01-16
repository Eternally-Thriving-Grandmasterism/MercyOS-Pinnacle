import mercy_py_bridge
import pymc as pm
import numpy as np
import arviz as az  # Optional for summaries

print("Real PyMC Bidirectional Oracle Ascension ❤️🚀🔥")

# Classic Eight Schools data (treatment effects hierarchical model)
eight_schools_data = {
    "J": 8,
    "y": np.array([28.0, 8.0, -3.0, 7.0, -1.0, 1.0, 18.0, 12.0]),
    "sigma": np.array([15.0, 10.0, 16.0, 11.0, 9.0, 11.0, 10.0, 18.0]),
}

def eight_schools_inference(data: dict):
    """Real PyMC hierarchical inference—returns posterior mean of mu (overall effect)"""
    with pm.Model() as model:
        # Hyperpriors
        mu = pm.Normal("mu", mu=0, sigma=10)
        tau = pm.HalfNormal("tau", sigma=10)
        
        # School-level effects
        theta = pm.Normal("theta", mu=mu, sigma=tau, shape=data["J"])
        
        # Likelihood
        obs = pm.Normal("obs", mu=theta, sigma=data["sigma"], observed=data["y"])
        
        # Sample (NUTS eternal)
        trace = pm.sample(2000, tune=1000, target_accept=0.95, random_seed=42)
    
    # Optional summary
    print(az.summary(trace, var_names=["mu", "tau", "theta"]))
    
    # Return posterior mean of overall effect for mercy-gating
    posterior_mean = float(trace.posterior["mu"].mean())
    print(f"Posterior mean mu: {posterior_mean:.2f}")
    return posterior_mean

# Create bidirectional MercyOracle with real PyMC callable (threshold e.g., positive effect > 5)
oracle = mercy_py_bridge.MercyOracle(eight_schools_inference, 5.0)

# Consult oracle—triggers actual PyMC sampling from Rust side, gates in PQ-secure Rust
decision = oracle.consult(data=eight_schools_data)
print(f"\nMercy-gated oracle decision (mu > 5.0): {decision} ❤️")

# Run multiple consultations (sampling each time—cache trace in production eternal)
print("\nSecond consultation (re-sample):")
decision2 = oracle.consult(data=eight_schools_data)
print(f"Decision: {decision2}")

print("\nReal PyMC integration harmony flowing supreme—replace eight_schools_inference with your quint_complete_model logic (load data, run trace, return key posterior) eternal!")
