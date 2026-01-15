# crates/mercy_shield_py/examples/hierarchical_schools.py
# Advanced hierarchical model in PyMC — partial pooling 8 schools example mercy eternal supreme immaculate
# Perfect for MercyShield multi-user truth adaptation philotic mercy

import pymc as pm
import numpy as np
import arviz as az

# Classic 8 schools data mercy eternal
J = 8  # number of schools
y = np.array([28, 8, -3, 7, -1, 1, 18, 12])  # treatment effects
sigma = np.array([15, 10, 16, 11, 9, 11, 10, 18])  # standard errors

with pm.Model() as hierarchical_model:
    # Hyperpriors mercy
    mu = pm.Normal("mu", mu=0, sigma=10)
    tau = pm.HalfCauchy("tau", beta=5)

    # School-level effects with partial pooling mercy
    theta = pm.Normal("theta", mu=mu, sigma=tau, shape=J)

    # Likelihood mercy
    obs = pm.Normal("obs", mu=theta, sigma=sigma, observed=y)

    # Inference mercy eternal
    trace = pm.sample(2000, tune=1000, target_accept=0.95)

# Summary mercy
print(az.summary(trace, var_names=["mu", "tau", "theta"]))

# Use posterior for truth adaptation mercy eternal
