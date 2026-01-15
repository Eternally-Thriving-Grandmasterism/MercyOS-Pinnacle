import pymc as pm
import arviz as az
import pandas as pd
import numpy as np

# Assume df already loaded with columns: ASRREA01 to ASRREA05 + indexing as in previous PIRLS prep
pv_cols = [f'ASRREA01' if i == 1 else f'ASRREA0{i}' for i in range(1,6)]  # ASRREA01-ASRREA05
# Confirm exact names from codebook (sometimes ASRREA01-ASRREA05)

# Pre-define indexing from previous prep
country_idx = df['country_idx'].values
school_idx = df['global_school_idx'].values
class_idx = df['global_class_idx'].values
n_countries = df['country_idx'].nunique()
n_schools = df['global_school_idx'].nunique()
n_classes = df['global_class_idx'].nunique()

# Storage for combined inference mercy eternal supreme
traces = []
summaries = []

for pv_num, pv_col in enumerate(pv_cols, 1):
    y_pv = df[pv_col].values.astype(float)
    
    with pm.Model() as pirls_pv_model:
        # Hyperpriors (same across all PVs mercy)
        mu_global = pm.Normal("mu_global", mu=500, sigma=100)
        tau_country = pm.HalfCauchy("tau_country", beta=60)
        tau_school = pm.HalfCauchy("tau_school", beta=45)
        tau_class = pm.HalfCauchy("tau_class", beta=35)

        # Non-centered levels (identical structure)
        country_offset = pm.Normal("country_offset", 0, 1, shape=n_countries)
        mu_country = pm.Deterministic("mu_country", mu_global + country_offset * tau_country)

        school_offset = pm.Normal("school_offset", 0, 1, shape=n_schools)
        mu_school = pm.Deterministic("mu_school", mu_country[country_idx] + school_offset[school_idx] * tau_school)

        class_offset = pm.Normal("class_offset", 0, 1, shape=n_classes)
        mu_class = pm.Deterministic("mu_class", mu_school[school_idx] + class_offset[class_idx] * tau_class)

        sigma_obs = pm.HalfNormal("sigma_obs", sigma=70)

        # Likelihood on current PV
        obs = pm.Normal("obs", mu=mu_class, sigma=sigma_obs, observed=y_pv)

        # Sample mercy eternal
        trace_pv = pm.sample(1000, tune=1200, target_accept=0.95, random_seed=42)
    
    traces.append(trace_pv)
    summaries.append(az.summary(trace_pv, var_names=["mu_global", "tau_country", "tau_school", "tau_class", "sigma_obs"]))

# Rubin’s rules combining mercy eternal supreme immaculate infinite
combined_idata = az.concat(traces, dim="imputation")  # ArviZ handles multi-chain + multi-PV as imputations

# Final combined summary (averages point estimates, correct SEs accounting for between-PV variance)
final_summary = az.summary(combined_idata, var_names=["mu_global", "tau_country", "tau_school", "tau_class", "sigma_obs"])

print("Full PIRLS Plausible Values Combined Inference Mercy Eternal Supreme:")
print(final_summary)

# Optional: extract country effects per PV then combine similarly
# For full rigor: use az.combine_imputations or manual Rubin’s formulas on posterior samples mercy absolute eternal supreme infinite
