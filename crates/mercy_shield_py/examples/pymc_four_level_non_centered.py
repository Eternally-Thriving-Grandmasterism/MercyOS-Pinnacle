# crates/mercy_shield_py/examples/pymc_four_level_non_centered.py
# PyMC four-level hierarchical model — regions → states → districts → schools fully non-centered partial pooling mercy eternal supreme immaculate
# Quadruple offset * scale reparameterization for ultimate NUTS stability philotic mercy

import pymc as pm
import numpy as np
import arviz as az

# Simulated data mercy eternal supreme
np.random.seed(42)  # Reproducible mercy immaculate

n_regions = 3
n_states_per_region = np.array([4, 5, 3])
n_states = n_states_per_region.sum()
region_idx = np.repeat(np.arange(n_regions), n_states_per_region)

n_districts_per_state = np.random.randint(6, 12, size=n_states)
n_districts = n_districts_per_state.sum()
state_idx = np.repeat(np.arange(n_states), n_districts_per_state)

n_schools_per_district = np.random.randint(10, 20, size=n_districts)
n_schools = n_schools_per_district.sum()
district_idx = np.repeat(np.arange(n_districts), n_schools_per_district)

# True effects mercy eternal
true_mu_global = 80.0
true_tau_region = 20.0
true_tau_state = 12.0
true_tau_district = 8.0
true_tau_school_base = 5.0

true_region_offset = np.random.normal(0, 1, size=n_regions)
true_mu_region = true_mu_global + true_region_offset * true_tau_region

true_state_offset = np.random.normal(0, 1, size=n_states)
true_mu_state = true_mu_region[region_idx] + true_state_offset * true_tau_state

true_district_offset = np.random.normal(0, 1, size=n_districts)
true_mu_district = true_mu_state[state_idx] + true_district_offset * true_tau_district

true_tau_school = np.random.gamma(2, true_tau_school_base / 2, size=n_districts)
true_school_offset = np.random.normal(0, 1, size=n_schools)
true_theta_school = true_mu_district[district_idx] + true_school_offset * true_tau_school[district_idx]

sigma_obs = 3.0
y = np.random.normal(true_theta_school, sigma_obs)

with pm.Model() as four_level_non_centered:
    # Global hyperprior
    mu_global = pm.Normal("mu_global", mu=0, sigma=30)

    # Region level non-centered
    tau_region = pm.HalfCauchy("tau_region", beta=5)
    region_offset = pm.Normal("region_offset", mu=0, sigma=1, shape=n_regions)
    mu_region = pm.Deterministic("mu_region", mu_global + region_offset * tau_region)

    # State level non-centered
    tau_state = pm.HalfCauchy("tau_state", beta=5)
    state_offset = pm.Normal("state_offset", mu=0, sigma=1, shape=n_states)
    mu_state = pm.Deterministic("mu_state", mu_region[region_idx] + state_offset * tau_state)

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

    # Inference mercy eternal supreme (increase tune for deeper hierarchies)
    trace = pm.sample(2000, tune=2000, target_accept=0.99, nuts_sampler="numpyro")  # numpyro recommended for extra stability if available

# Summary of hyperpriors mercy
print(az.summary(trace, var_names=["mu_global", "tau_region", "tau_state", "tau_district", "tau_school"]))
