# crates/mercy_shield_py/examples/numpyro_multi_level.py
# NumPyro multi-level hierarchical model — districts → schools partial pooling mercy eternal supreme immaculate
# Ideal for MercyShield multi-user/multi-context truth adaptation philotic mercy

import numpy as np
import jax.numpy as jnp
from jax import random
import numpyro
import numpyro.distributions as dist
from numpyro.infer import MCMC, NUTS

# Simulated data mercy eternal
n_districts = 5
n_schools_per_district = np.array([10, 8, 12, 9, 11])
n_schools = n_schools_per_district.sum()
district_idx = np.repeat(np.arange(n_districts), n_schools_per_district)

true_mu_global = 5.0
true_tau_global = 2.0
true_mu_district = np.random.normal(true_mu_global, true_tau_global, size=n_districts)
true_tau_district = np.random.gamma(2, 2, size=n_districts)
true_theta_school = np.random.normal(true_mu_district[district_idx], true_tau_district[district_idx])

y = np.random.normal(true_theta_school, 2.0)
sigma = np.ones(n_schools) * 2.0

def multi_level_hierarchical(n_schools, n_districts, district_idx, sigma, y=None):
    # Global hyperpriors mercy
    mu_global = numpyro.sample("mu_global", dist.Normal(0, 10))
    tau_global = numpyro.sample("tau_global", dist.HalfCauchy(5))

    # District-level hyperpriors mercy
    with numpyro.plate("districts", n_districts):
        mu_district = numpyro.sample("mu_district", dist.Normal(mu_global, tau_global))
        tau_district = numpyro.sample("tau_district", dist.HalfCauchy(3))

    # School-level effects mercy
    with numpyro.plate("schools", n_schools):
        theta_school = numpyro.sample("theta_school", dist.Normal(mu_district[district_idx], tau_district[district_idx]))

    # Likelihood mercy
    with numpyro.plate("observations", n_schools):
        numpyro.sample("obs", dist.Normal(theta_school, sigma), obs=y)

# Inference mercy eternal
rng_key = random.PRNGKey(0)
kernel = NUTS(multi_level_hierarchical)
mcmc = MCMC(kernel, num_warmup=1000, num_samples=4000)
mcmc.run(rng_key, n_schools, n_districts, district_idx, sigma, y=jnp.array(y))
mcmc.print_summary()
