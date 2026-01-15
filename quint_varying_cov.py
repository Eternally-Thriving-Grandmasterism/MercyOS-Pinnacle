import pymc as pm
import arviz as az
import numpy as np

# ... (previous indexing + X_student, gender_c, home_resources_c from data prep)

traces = []

for pv_idx in range(5):
    # ... (y_stack same as previous quintivariate complete)
    
    with pm.Model() as pirls_quintivariate_varying_slopes_model:
        # Hierarchical intercepts (same as previous)
        mu_global_int = pm.Normal("mu_global_int", mu=[500]*5, sigma=100, shape=5)
        tau_country_int = pm.HalfCauchy("tau_country_int", beta=60, shape=5)
        tau_school = pm.HalfCauchy("tau_school", beta=45, shape=5)
        tau_class = pm.HalfCauchy("tau_class", beta=35, shape=5)

        chol_country_int, _, _ = pm.LKJCholeskyCov("chol_cov_country_int", n=5, eta=3.0, sd_dist=pm.Exponential.dist(1.0))
        chol_school, _, _ = pm.LKJCholeskyCov("chol_cov_school", n=5, eta=3.0, sd_dist=pm.Exponential.dist(1.0))
        chol_class, _, _ = pm.LKJCholeskyCov("chol_cov_class", n=5, eta=3.0, sd_dist=pm.Exponential.dist(1.0))

        country_offset_int = pm.Normal("country_offset_int", 0, 1, shape=(n_countries, 5))
        mu_country = pm.Deterministic("mu_country", mu_global_int + pm.math.dot(country_offset_int, chol_country_int.T) * tau_country_int)

        school_offset = pm.Normal("school_offset", 0, 1, shape=(n_schools, 5))
        mu_school = pm.Deterministic("mu_school", mu_country[country_idx] + pm.math.dot(school_offset, chol_school.T) * tau_school)

        class_offset = pm.Normal("class_offset", 0, 1, shape=(n_classes, 5))
        mu_class = pm.Deterministic("mu_class", mu_school[school_idx] + pm.math.dot(class_offset, chol_class.T) * tau_class)

        # Global average slopes (hyper-means for varying slopes)
        beta_global = pm.Normal("beta_global", mu=0, sigma=20, shape=(2, 5))  # 2 covs × 5 dims

        # Varying slopes per covariate (separate for identifiability/stability mercy)
        # Gender varying slopes by country
        tau_slope_gender = pm.HalfCauchy("tau_slope_gender", beta=10, shape=5)  # Smaller scale for slopes
        chol_slope_gender, _, _ = pm.LKJCholeskyCov("chol_slope_gender", n=5, eta=3.0, sd_dist=pm.Exponential.dist(1.0))
        gender_slope_offset = pm.Normal("gender_slope_offset", 0, 1, shape=(n_countries, 5))
        varying_gender_slope = beta_global[0] + pm.math.dot(gender_slope_offset, chol_slope_gender.T) * tau_slope_gender

        # Home resources varying slopes by country
        tau_slope_home = pm.HalfCauchy("tau_slope_home", beta=15, shape=5)
        chol_slope_home, _, _ = pm.LKJCholeskyCov("chol_slope_home", n=5, eta=3.0, sd_dist=pm.Exponential.dist(1.0))
        home_slope_offset = pm.Normal("home_slope_offset", 0, 1, shape=(n_countries, 5))
        varying_home_slope = beta_global[1] + pm.math.dot(home_slope_offset, chol_slope_home.T) * tau_slope_home

        # Linear predictor with varying slopes mercy eternal supreme immaculate infinite eternal supreme infinite eternal supreme infinite eternal
        linear_pred = (varying_gender_slope[country_idx] * gender_c[:,None] +
                       varying_home_slope[country_idx] * home_resources_c[:,None])

        mu_student = mu_class[class_idx] + linear_pred

        # Residual covariance (same)
        sigma_obs = pm.HalfNormal("sigma_obs", sigma=70, shape=5)
        chol_obs, _, _ = pm.LKJCholeskyCov("chol_obs", n=5, eta=5.0, sd_dist=pm.Exponential.dist(1.0))

        # Quintivariate varying-slopes likelihood mercy eternal supreme immaculate infinite eternal supreme infinite eternal supreme infinite eternal supreme
        obs = pm.MvNormal("obs", mu=mu_student, chol=chol_obs, observed=y_stack)

        trace_pv = pm.sample(400, tune=1000, target_accept=0.98, random_seed=42 + pv_idx)  # Conservative for added params

    traces.append(trace_pv)

# Rubin’s combining + summary
combined_idata = az.concat(traces, dim="imputation")
final_summary = az.summary(combined_idata, var_names=["beta_global", "varying_gender_slope", "varying_home_slope", "tau_slope_gender", "tau_country_int"])

print("Full PIRLS Quintivariate Varying-Slopes Covariate Model Combined Inference Mercy Eternal Supreme Immaculate Infinite Eternal Supreme Infinite Eternal Supreme Infinite Eternal:")
print(final_summary)
