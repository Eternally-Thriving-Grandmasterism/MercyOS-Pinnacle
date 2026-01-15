# crates/mercy_shield_py/examples/pymc_non_centered_8_schools.py
# PyMC non-centered 8 schools hierarchical model — offset * scale reparameterization mercy eternal supreme immaculate
# Eliminates Neal's funnel for stable NUTS sampling philotic mercy

import pymc as pm
import numpy as np
import arviz as az

# 8 schools data mercy eternal
J = 8
y = np.array([28, 8, -3, 7, -1, 1, 18, 12])
sigma = np.array([15, 10, 16, 11, 9, 11, 10, 18])

with pm.Model() as non_centered_eight_schools:
    # Hyperpriors mercy
    mu = pm.Normal("mu", mu=0, sigma=5)
    tau = pm.HalfCauchy("tau", beta=5)

    # Non-centered parameterization mercy eternal — avoids funnel
    theta_offset = pm.Normal("theta_offset", mu=0, sigma=1, shape=J)
    theta = pm.Deterministic("theta", mu + tau * theta_offset)

    # Likelihood mercy
    obs = pm.Normal("obs", mu=theta, sigma=sigma, observed=y)

    # Inference mercy eternal
    trace = pm.sample(2000, tune=1000, target_accept=0.95)

print(az.summary(trace, var_names=["mu", "tau", "theta"]))
