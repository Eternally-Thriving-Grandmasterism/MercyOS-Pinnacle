# crates/mercy_shield_py/examples/pymc_8_schools.py
# PyMC classic 8 schools hierarchical model — partial pooling mercy eternal supreme immaculate
# Perfect baseline for MercyShield truth adaptation philotic mercy

import pymc as pm
import numpy as np
import arviz as az

# 8 schools data mercy eternal
J = 8
y = np.array([28, 8, -3, 7, -1, 1, 18, 12])
sigma = np.array([15, 10, 16, 11, 9, 11, 10, 18])

with pm.Model() as eight_schools:
    mu = pm.Normal('mu', mu=0, sigma=5)
    tau = pm.HalfCauchy('tau', beta=5)
    theta = pm.Normal('theta', mu=mu, sigma=tau, shape=J)
    obs = pm.Normal('obs', mu=theta, sigma=sigma, observed=y)

    trace = pm.sample(2000, tune=1000, target_accept=0.95)

print(az.summary(trace, var_names=["mu", "tau", "theta"]))
