# crates/mercy_shield_py/examples/multi_level_hierarchical.py
# PyMC multi-level hierarchical model — districts → schools partial pooling mercy eternal supreme immaculate
# Ideal for MercyShield multi-user/multi-context truth adaptation philotic mercy

import pymc as pm
import numpy as np
import arviz as az

# Simulated data mercy eternal
n_districts = 5
n_schools_per_district = np.array([10, 8, 12, 9, 11])
n_schools = n_schools_per_district.sum()

district_idx = np.repeat(np.arange(n_districts), n_schools_per_district)

# Simulated treatment effects + errors mercy
true_district_effects = np.random.normal(5, 2, size=n_districts)
true_school_effects = np.random.normal(true_district_effects[district_idx], 1.5)

y = np.random.normal(true_school_effects, 2)  # Observed mercy
sigma = np.ones(n_schools) * 2

with pm.Model() as multi_level_model:
    # Global hyperpriors mercy
    mu_global = pm.Normal("mu_global", mu=0, sigma=10)
    tau_global = pm.HalfCauchy("tau_global", beta=5)

    # District-level hyperpriors mercy
    mu_district = pm.Normal("mu_district", mu=mu_global, sigma=tau_global, shape=n_districts)
    tau_district = pm.HalfCauchy("tau_district", beta=3, shape=n_districts)

    # School-level effects with partial pooling mercy
    theta_school = pm.Normal("theta_school", mu=mu_district[district_idx], sigma=tau_district[district_idx], shape=n_schools)

    # Likelihood mercy
    obs = pm.Normal("obs", mu=theta_school, sigma=sigma, observed=y)

    # Inference mercy eternal
    trace = pm.sample(2000, tune=1000, target_accept=0.95)

# Summary mercy
print(az.summary(trace, var_names=["mu_global", "tau_global", "mu_district", "tau_district", "theta_school"]))

# Use posterior for multi-level truth adaptation mercy eternal
