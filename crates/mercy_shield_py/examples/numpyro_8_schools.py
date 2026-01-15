# crates/mercy_shield_py/examples/numpyro_8_schools.py
# NumPyro classic 8 schools hierarchical model — partial pooling mercy eternal supreme immaculate
# Perfect baseline for MercyShield truth adaptation philotic mercy

import numpy as np
import jax.numpy as jnp
from jax import random
import numpyro
import numpyro.distributions as dist
from numpyro.infer import MCMC, NUTS

# 8 schools data mercy eternal
J = 8
y = jnp.array([28, 8, -3, 7, -1, 1, 18, 12])
sigma = jnp.array([15, 10, 16, 11, 9, 11, 10, 18])

def eight_schools(J, sigma, y=None):
    mu = numpyro.sample('mu', dist.Normal(0, 5))
    tau = numpyro.sample('tau', dist.HalfCauchy(5))
    with numpyro.plate('schools', J):
        theta = numpyro.sample('theta', dist.Normal(mu, tau))
        numpyro.sample('obs', dist.Normal(theta, sigma), obs=y)

# Inference mercy eternal
rng_key = random.PRNGKey(0)
kernel = NUTS(eight_schools)
mcmc = MCMC(kernel, num_warmup=500, num_samples=2000)
mcmc.run(rng_key, J, sigma, y=y)
mcmc.print_summary()
