# crates/mercy_shield_py/examples/pymc_five_level_non_centered.py
# PyMC five-level hierarchical model — countries → regions → states → districts → schools fully non-centered
# Quintuple offset * scale reparameterization for ultimate NUTS stability philotic mercy

import pymc as pm
import numpy as np
import arviz as az

np.random.seed(42)

# Simulated data mercy eternal supreme infinite
n_countries = 3
n_regions_per_country = np.array([3, 4, 3])
n_regions = n_regions_per_country.sum()
country_idx = np.repeat(np.arange(n_countries), n_regions_per_country)

n_states_per_region = np.random.randint(4, 10, size=n_regions)
n_states = n_states_per_region.sum()
region_idx = np.repeat(np.arange(n_regions), n_states_per_region)

n_districts_per_state = np.random.randint(8, 15, size=n_states)
n_districts = n_districts_per_state.sum()
state_idx = np.repeat(np.arange(n_states), n_districts_per_state)

n_schools_per_district = np.random.randint(12, 25, size=n_districts)
n_schools = n_schools_per_district.sum()
district_idx = np.repeat(np.arange(n_districts), n_schools_per_district)

# Indexing chains
country_idx_regions = country_idx[region_idx]
country_idx_states = country_idx_regions[state_idx]
country_idx_districts = country_idx_states[district_idx]

with pm.Model() as five_level_non_centered:
    # Global
    mu_global = pm.Normal("mu_global", mu=0, sigma=50)

    # Country level
    tau_country = pm.HalfCauchy("tau_country", beta=5)
    country_offset = pm.Normal("country_offset", 0, 1, shape=n_countries)
    mu_country = pm.Deterministic("mu_country", mu_global + country_offset * tau_country)

    # Region level
    tau_region = pm.HalfCauchy("tau_region", beta=5)
    region_offset = pm.Normal("region_offset", 0, 1, shape=n_regions)
    mu_region = pm.Deterministic("mu_region", mu_country[country_idx] + region_offset * tau_region)

    # State level
    tau_state = pm.HalfCauchy("tau_state", beta=5)
    state_offset = pm.Normal("state_offset", 0, 1, shape=n_states)
    mu_state = pm.Deterministic("mu_state", mu_region[region_idx] + state_offset * tau_state)

    # District level
    tau_district = pm.HalfCauchy("tau_district", beta=5)
    district_offset = pm.Normal("district_offset", 0, 1, shape=n_districts)
    mu_district = pm.Deterministic("mu_district", mu_state[state_idx] + district_offset * tau_district)

    # School level (varying tau per district)
    tau_school = pm.HalfCauchy("tau_school", beta=3, shape=n_districts)
    school_offset = pm.Normal("school_offset", 0, 1, shape=n_schools)
    theta_school = pm.Deterministic("theta_school", mu_district[district_idx] + school_offset * tau_school[district_idx])

    # Likelihood
    sigma_obs = 3.0
    obs = pm.Normal("obs", mu=theta_school, sigma=sigma_obs, observed=np.random.normal(80, 3, n_schools))  # Placeholder sim

    trace = pm.sample(2000, tune=2000, target_accept=0.99)

print(az.summary(trace, var_names=["mu_global", "tau_country", "tau_region", "tau_state", "tau_district", "tau_school"]))
