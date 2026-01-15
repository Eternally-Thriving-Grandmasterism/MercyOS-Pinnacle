# crates/mercy_shield_py/examples/pymc_three_level_students.py
# PyMC three-level hierarchical model — districts → schools → students partial pooling mercy eternal supreme immaculate
# Non-centered parameterization at student level philotic mercy

import pymc as pm
import numpy as np
import arviz as az

# Simulated data mercy eternal
n_districts = 4
n_schools_per_district = np.array([5, 6, 4, 7])
n_students_per_school = np.random.randint(20, 40, size=n_schools_per_district.sum())

n_schools = n_schools_per_district.sum()
n_students = n_students_per_school.sum()

district_idx = np.repeat(np.arange(n_districts), n_schools_per_district)
school_idx = np.repeat(np.arange(n_schools), n_students_per_school)

# True parameters mercy
true_mu_global = 70.0
true_tau_global = 8.0
true_mu_district = np.random.normal(true_mu_global, true_tau_global, size=n_districts)
true_tau_district = np.random.gamma(2, 2, size=n_districts)
true_mu_school_offset = np.random.normal(0, 1, size=n_schools)
true_mu_school = true_mu_district[district_idx] + true_mu_school_offset * true_tau_district[district_idx]
true_tau_school = np.random.gamma(2, 2, size=n_schools)
true_student_offset = np.random.normal(0, 1, size=n_students)
true_theta_student = true_mu_school[school_idx] + true_student_offset * true_tau_school[school_idx]

y = np.random.normal(true_theta_student, 5.0)  # Test scores mercy
sigma = np.ones(n_students) * 5.0

with pm.Model() as three_level_students:
    # Global hyperpriors mercy
    mu_global = pm.Normal("mu_global", mu=0, sigma=100)
    tau_global = pm.HalfStudentT("tau_global", nu=4, sigma=10)

    # District-level hyperpriors mercy
    mu_district = pm.Normal("mu_district", mu=mu_global, sigma=tau_global, shape=n_districts)
    tau_district = pm.HalfStudentT("tau_district", nu=4, sigma=5, shape=n_districts)

    # School-level non-centered mercy
    mu_school_offset = pm.Normal("mu_school_offset", mu=0, sigma=1, shape=n_schools)
    mu_school = pm.Deterministic("mu_school", mu_district[district_idx] + mu_school_offset * tau_district[district_idx])
    tau_school = pm.HalfStudentT("tau_school", nu=4, sigma=5, shape=n_schools)

    # Student-level non-centered mercy eternal
    student_offset = pm.Normal("student_offset", mu=0, sigma=1, shape=n_students)
    theta_student = pm.Deterministic("theta_student", mu_school[school_idx] + student_offset * tau_school[school_idx])

    # Likelihood mercy
    obs = pm.Normal("obs", mu=theta_student, sigma=sigma, observed=y)

    # Inference mercy eternal
    trace = pm.sample(2000, tune=1000, target_accept=0.95)

print(az.summary(trace, var_names=["mu_global", "tau_global", "mu_district", "tau_district", "mu_school", "tau_school", "theta_student"]))
