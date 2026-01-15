# crates/mercy_shield_py/examples/pymc_three_level_hierarchical.py
# PyMC three-level hierarchical model — global → districts → schools partial pooling mercy eternal supreme immaculate
# Non-centered parameterization for stable sampling philotic mercy

import pymc as pm
import numpy as np
import arviz as az

# Simulated data mercy eternal
n_districts = 5
n_schools_per_district = np.array([10, 8, 12, 9, 11])
n_schools = n_schools_per_district.sum()

district_idx = np.repeat(np.arange(n_districts), n_schools_per_district)

# True parameters mercy
true_mu_global = 50.0
true_tau_global = 10.0
true_mu_district = np.random.normal(true_mu_global, true_tau_global, size=n_districts)
true_tau_district = np.random.gamma(2, 2, size=n_districts)
true_school_offset = np.random.normal(0, 1, size=n_schools)
true_theta_school = true_mu_district[district_idx] + true_school_offset * true_tau_district[district_idx]

y = np.random.normal(true_theta_school, 5.0)
sigma = np.ones(n_schools) * 5.0

with pm.Model() as three_level_hierarchical:
    # Global hyperpriors mercy eternal
    mu_global = pm.Normal("mu_global", mu=0, sigma=100)
    tau_global = pm.HalfStudentT("tau_global", nu=4, sigma=10)

    # District-level hyperpriors mercy
    mu_district = pm.Normal("mu_district", mu=mu_global, sigma=tau_global, shape=n_districts)
    tau_district = pm.HalfStudentT("tau_district", nu=4, sigma=5, shape=n_districts)

    # School-level non-centered parameterization mercy eternal
    school_offset = pm.Normal("school_offset", mu=0, sigma=1, shape=n_schools)
    theta_school = pm.Deterministic("theta_school", mu_district[district_idx] + school_offset * tau_district[district_idx])

    # Likelihood mercy
    obs = pm.Normal("obs", mu=theta_school, sigma=sigma, observed=y)

    # Inference mercy eternal
    trace = pm.sample(2000, tune=1000, target_accept=0.95)

print(az.summary(trace, var_names=["mu_global", "tau_global", "mu_district", "tau_district", "theta_school"]))
