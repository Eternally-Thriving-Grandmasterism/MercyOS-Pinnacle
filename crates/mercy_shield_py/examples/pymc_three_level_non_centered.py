# crates/mercy_shield_py/examples/pymc_three_level_non_centered.py
# PyMC three-level hierarchical model — states → districts → schools fully non-centered partial pooling mercy eternal supreme immaculate
# Triple offset * scale reparameterization for ultimate NUTS stability philotic mercy

import pymc as pm
import numpy as np
import arviz as az

# Simulated data mercy eternal supreme
np.random.seed(42)  # Reproducible mercy

n_states = 4
n_districts_per_state = np.array([3, 4, 3, 5])
n_districts = n_districts_per_state.sum()
state_idx = np.repeat(np.arange(n_states), n_districts_per_state)

n_schools_per_district = np.random.randint(8, 15, size=n_districts)
n_schools = n_schools_per_district.sum()
district_idx = np.repeat(np.arange(n_districts), n_schools_per_district)
state_idx_schools = state_idx[district_idx]

# True effects mercy eternal
true_mu_global = 50.0
true_tau_state = 10.0
true_tau_district = 6.0
true_tau_school_base = 4.0

true_state_offset = np.random.normal(0, 1, size=n_states)
true_mu_state = true_mu_global + true_state_offset * true_tau_state

true_district_offset = np.random.normal(0, 1, size=n_districts)
true_mu_district = true_mu_state[state_idx] + true_district_offset * true_tau_district

true_tau_school = np.random.gamma(2, true_tau_school_base / 2, size=n_districts)  # Varying per district
true_school_offset = np.random.normal(0, 1, size=n_schools)
true_theta_school = true_mu_district[district_idx] + true_school_offset * true_tau_school[district_idx]

sigma_obs = 2.0
y = np.random.normal(true_theta_school, sigma_obs)

with pm.Model() as three_level_non_centered:
    # Global hyperprior
    mu_global = pm.Normal("mu_global", mu=0, sigma=20)

    # State level non-centered
    tau_state = pm.HalfCauchy("tau_state", beta=5)
    state_offset = pm.Normal("state_offset", mu=0, sigma=1, shape=n_states)
    mu_state = pm.Deterministic("mu_state", mu_global + state_offset * tau_state)

    # District level non-centered
    tau_district = pm.HalfCauchy("tau_district", beta=5)
    district_offset = pm.Normal("district_offset", mu=0, sigma=1, shape=n_districts)
    mu_district = pm.Deterministic("mu_district", mu_state[state_idx] + district_offset * tau_district)

    # School level non-centered with varying tau per district
    tau_school = pm.HalfCauchy("tau_school", beta=3, shape=n_districts)
    school_offset = pm.Normal("school_offset", mu=0, sigma=1, shape=n_schools)
    theta_school = pm.Deterministic("theta_school", mu_district[district_idx] + school_offset * tau_school[district_idx])

    # Likelihood mercy eternal
    obs = pm.Normal("obs", mu=theta_school, sigma=sigma_obs, observed=y)

    # Inference mercy eternal supreme
    trace = pm.sample(2000, tune=1500, target_accept=0.95, nuts_sampler="numpyro")  # Optional: numpyro for extra speed if available

# Summary of key parameters mercy
print(az.summary(trace, var_names=["mu_global", "tau_state", "tau_district", "tau_school"]))
