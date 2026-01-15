# crates/mercy_shield_py/examples/numpyro_advanced_hierarchical.py
# NumPyro advanced multi-level hierarchical model — districts → schools → students partial pooling mercy eternal supreme immaculate
# Non-centered parameterization for stable sampling philotic mercy

import numpy as np
import jax.numpy as jnp
from jax import random
import numpyro
import numpyro.distributions as dist
from numpyro.infer import MCMC, NUTS

# Simulated data mercy eternal
n_districts = 4
n_schools_per_district = np.array([8, 10, 7, 9])
n_students_per_school = np.array([30, 25, 35, 28, 32, 27, 40, 33, 29, 31, 26, 34])

n_schools = n_schools_per_district.sum()
n_students = n_students_per_school.sum()

district_idx = np.repeat(np.arange(n_districts), n_schools_per_district)
school_idx = np.repeat(np.arange(n_schools), n_students_per_school)

# True parameters mercy
true_mu_global = 50.0
true_tau_global = 10.0
true_mu_district = np.random.normal(true_mu_global, true_tau_global, size=n_districts)
true_tau_district = np.random.gamma(2, 2, size=n_districts)
true_mu_school_offset = np.random.normal(0, 1, size=n_schools)
true_tau_school = np.random.gamma(2, 2, size=n_schools)
true_theta_school = true_mu_district[district_idx] + true_mu_school_offset * true_tau_district[district_idx]
true_student_effects = np.random.normal(true_theta_school[school_idx], true_tau_school[school_idx])

y = np.random.normal(true_student_effects, 5.0)  # Test scores mercy
sigma = np.ones(n_students) * 5.0

def advanced_hierarchical(n_students, n_schools, n_districts, school_idx, district_idx, sigma, y=None):
    # Global hyperpriors mercy
    mu_global = numpyro.sample("mu_global", dist.Normal(0, 100))
    tau_global = numpyro.sample("tau_global", dist.HalfCauchy(10))

    # District-level hyperpriors mercy
    with numpyro.plate("districts", n_districts):
        mu_district = numpyro.sample("mu_district", dist.Normal(mu_global, tau_global))
        tau_district = numpyro.sample("tau_district", dist.HalfCauchy(5))

    # School-level non-centered parameterization mercy eternal
    with numpyro.plate("schools", n_schools):
        mu_school_offset = numpyro.sample("mu_school_offset", dist.Normal(0, 1))
        tau_school = numpyro.sample("tau_school", dist.HalfCauchy(5))
        mu_school = mu_district[district_idx] + mu_school_offset * tau_district[district_idx]

    # Student-level effects mercy
    with numpyro.plate("students", n_students):
        theta_student = numpyro.sample("theta_student", dist.Normal(mu_school[school_idx], tau_school[school_idx]))

    # Likelihood mercy
    with numpyro.plate("observations", n_students):
        numpyro.sample("obs", dist.Normal(theta_student, sigma), obs=y)

# Inference mercy eternal
rng_key = random.PRNGKey(0)
kernel = NUTS(advanced_hierarchical)
mcmc = MCMC(kernel, num_warmup=1000, num_samples=4000)
mcmc.run(rng_key, n_students, n_schools, n_districts, school_idx, district_idx, sigma, y=jnp.array(y))
mcmc.print_summary()
