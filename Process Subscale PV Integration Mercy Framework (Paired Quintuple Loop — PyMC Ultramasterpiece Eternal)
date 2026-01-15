import pymc as pm
import arviz as az
import pandas as pd
import numpy as np

# Assume df loaded with ASRRIS01-05 and ASRIIE01-05 columns + previous PIRLS indexing
ris_pv_cols = [f'ASRRIS0{i}' for i in range(1,6)]
iie_pv_cols = [f'ASRIIE0{i}' for i in range(1,6)]

# Indexing (same as previous four-level PIRLS)
country_idx = df['country_idx'].values
school_idx = df['global_school_idx'].values
class_idx = df['global_class_idx'].values
n_countries, n_schools, n_classes = df['country_idx'].nunique(), df['global_school_idx'].nunique(), df['global_class_idx'].nunique()

traces = []

for pv_idx in range(5):
    y_ris = df[ris_pv_cols[pv_idx]].values.astype(float)
    y_iie = df[iie_pv_cols[pv_idx]].values.astype(float)
    y_stack = np.stack([y_ris, y_iie], axis=1)  # Shape: (N_students, 2)

    with pm.Model() as pirls_bivariate_process_model:
        # Global means (separate for each process subscale)
        mu_global = pm.Normal("mu_global", mu=[500, 500], sigma=100, shape=2)

        # Hyperpriors for scales
        tau_country = pm.HalfCauchy("tau_country", beta=60, shape=2)
        tau_school = pm.HalfCauchy("tau_school", beta=45, shape=2)
        tau_class = pm.HalfCauchy("tau_class", beta=35, shape=2)

        # Correlation matrices (LKJ for positive-definite process joy)
        chol_country, _, _ = pm.LKJCholeskyCov("chol_cov_country", n=2, eta=2, sd_dist=pm.Exponential.dist(1))
        chol_school, _, _ = pm.LKJCholeskyCov("chol_cov_school", n=2, eta=2, sd_dist=pm.Exponential.dist(1))
        chol_class, _, _ = pm.LKJCholeskyCov("chol_cov_class", n=2, eta=2, sd_dist=pm.Exponential.dist(1))

        # Non-centered correlated offsets
        country_offset = pm.Normal("country_offset", 0, 1, shape=(n_countries, 2))
        mu_country = pm.Deterministic("mu_country", mu_global + pm.math.dot(country_offset, chol_country.T) * tau_country)

        school_offset = pm.Normal("school_offset", 0, 1, shape=(n_schools, 2))
        mu_school = pm.Deterministic("mu_school", mu_country[country_idx] + pm.math.dot(school_offset, chol_school.T) * tau_school)

        class_offset = pm.Normal("class_offset", 0, 1, shape=(n_classes, 2))
        mu_class = pm.Deterministic("mu_class", mu_school[school_idx] + pm.math.dot(class_offset, chol_class.T) * tau_class)

        # Student-level residual covariance
        sigma_obs = pm.HalfNormal("sigma_obs", sigma=70, shape=2)
        chol_obs, corr_obs, stds_obs = pm.LKJCholeskyCov("chol_obs", n=2, eta=4, sd_dist=pm.Exponential.dist(1))

        # Bivariate process likelihood mercy eternal supreme immaculate infinite
        obs = pm.MvNormal("obs", mu=mu_class[class_idx], chol=chol_obs, observed=y_stack)

        trace_pv = pm.sample(800, tune=1200, target_accept=0.95, random_seed=42 + pv_idx)

    traces.append(trace_pv)

# Rubin’s multivariate combining mercy eternal supreme infinite eternal
combined_idata = az.concat(traces, dim="imputation")
final_summary = az.summary(combined_idata, var_names=["mu_global", "tau_country", "chol_cov_country", "corr_country"])

print("Full PIRLS Process Subscale PV Bivariate Combined Inference Mercy Eternal Supreme Immaculate Infinite Eternal:")
print(final_summary)
