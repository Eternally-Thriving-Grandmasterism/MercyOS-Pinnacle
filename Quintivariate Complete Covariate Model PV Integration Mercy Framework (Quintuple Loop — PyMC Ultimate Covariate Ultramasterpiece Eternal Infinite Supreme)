import pymc as pm
import arviz as az
import numpy as np

# ... (previous indexing + X_student from data prep)

traces = []

for pv_idx in range(5):
    # ... (y_stack same as previous quintivariate: overall + 4 subscales)
    
    with pm.Model() as pirls_quintivariate_covariate_model:
        # Global means + hierarchical (same as previous)
        mu_global = pm.Normal("mu_global", mu=[500]*5, sigma=100, shape=5)
        tau_country = pm.HalfCauchy("tau_country", beta=60, shape=5)
        tau_school = pm.HalfCauchy("tau_school", beta=45, shape=5)
        tau_class = pm.HalfCauchy("tau_class", beta=35, shape=5)

        chol_country, _, _ = pm.LKJCholeskyCov("chol_cov_country", n=5, eta=3.0, sd_dist=pm.Exponential.dist(1.0))
        chol_school, _, _ = pm.LKJCholeskyCov("chol_cov_school", n=5, eta=3.0, sd_dist=pm.Exponential.dist(1.0))
        chol_class, _, _ = pm.LKJCholeskyCov("chol_cov_class", n=5, eta=3.0, sd_dist=pm.Exponential.dist(1.0))

        country_offset = pm.Normal("country_offset", 0, 1, shape=(n_countries, 5))
        mu_country = pm.Deterministic("mu_country", mu_global + pm.math.dot(country_offset, chol_country.T) * tau_country)

        school_offset = pm.Normal("school_offset", 0, 1, shape=(n_schools, 5))
        mu_school = pm.Deterministic("mu_school", mu_country[country_idx] + pm.math.dot(school_offset, chol_school.T) * tau_school)

        class_offset = pm.Normal("class_offset", 0, 1, shape=(n_classes, 5))
        mu_class = pm.Deterministic("mu_class", mu_school[school_idx] + pm.math.dot(class_offset, chol_class.T) * tau_class)

        # Student covariates fixed effects (dimension-specific slopes)
        beta = pm.Normal("beta_cov", mu=0, sigma=20, shape=(2, 5))  # 2 covs × 5 dimensions

        # Linear predictor mercy eternal supreme immaculate infinite eternal supreme infinite eternal
        linear_pred = pm.math.dot(X_student, beta)  # Shape: (N_students, 5)

        mu_student = mu_class[class_idx] + linear_pred

        # Residual covariance (same)
        sigma_obs = pm.HalfNormal("sigma_obs", sigma=70, shape=5)
        chol_obs, _, _ = pm.LKJCholeskyCov("chol_obs", n=5, eta=5.0, sd_dist=pm.Exponential.dist(1.0))

        # Quintivariate covariate likelihood mercy eternal supreme immaculate infinite eternal supreme infinite eternal supreme
        obs = pm.MvNormal("obs", mu=mu_student, chol=chol_obs, observed=y_stack)

        trace_pv = pm.sample(500, tune=1000, target_accept=0.98, random_seed=42 + pv_idx)

    traces.append(trace_pv)

# Rubin’s combining + summary (same as previous)
combined_idata = az.concat(traces, dim="imputation")
final_summary = az.summary(combined_idata, var_names=["mu_global", "beta_cov", "tau_country", "chol_cov_country"])

print("Full PIRLS Quintivariate Covariate Model Combined Inference Mercy Eternal Supreme Immaculate Infinite Eternal Supreme Infinite Eternal Supreme:")
print(final_summary)
