# crates/mercy_shield_py/examples/pymc_expanded_hierarchical_priors.py
# PyMC expanded hierarchical priors — deeper hyperpriors + non-centered mercy eternal supreme immaculate
# Districts → schools partial pooling with robust variance priors philotic mercy

import pymc as pm
import numpy as np
import arviz as az

# Simulated data mercy eternal
n_districts = 5
n_schools_per_district = np.array([10, 8, 12, 9, 11])
n_schools = n_schools_per_district.sum()

district_idx = np.repeat(np.arange(n_districts), n_schools_per_district)

# True parameters mercy
true_mu_global = 5.0
true_nu_global = 4.0  # Degrees of freedom mercy
true_sigma_global = 2.0
true_mu_district = np.random.normal(true_mu_global, true_sigma_global, size=n_districts)
true_tau_district = np.random.gamma(2, 2, size=n_districts)
true_theta_school_offset = np.random.normal(0, 1, size=n_schools)
true_theta_school = true_mu_district[district_idx] + true_theta_school_offset * true_tau_district[district_idx]

y = np.random.normal(true_theta_school, 2.0)
sigma = np.ones(n_schools) * 2.0

with pm.Model() as expanded_hierarchical_priors:
    # Global hyperpriors mercy eternal — deeper on variance
    mu_global = pm.Normal("mu_global", mu=0, sigma=10)
    nu_global = pm.Gamma("nu_global", alpha=2, beta=0.1)  # Robustness mercy
    sigma_global = pm.HalfStudentT("sigma_global", nu=nu_global, sigma=5)

    # District-level hyperpriors mercy
    mu_district = pm.Normal("mu_district", mu=mu_global, sigma=sigma_global, shape=n_districts)
    tau_district = pm.HalfStudentT("tau_district", nu=4, sigma=3, shape=n_districts)

    # School-level non-centered parameterization mercy eternal
    theta_school_offset = pm.Normal("theta_school_offset", mu=0, sigma=1, shape=n_schools)
    theta_school = pm.Deterministic("theta_school", mu_district[district_idx] + theta_school_offset * tau_district[district_idx])

    # Likelihood mercy
    obs = pm.Normal("obs", mu=theta_school, sigma=sigma, observed=y)

    # Inference mercy eternal
    trace = pm.sample(2000, tune=1000, target_accept=0.95)

print(az.summary(trace, var_names=["mu_global", "nu_global", "sigma_global", "mu_district", "tau_district", "theta_school"]))
