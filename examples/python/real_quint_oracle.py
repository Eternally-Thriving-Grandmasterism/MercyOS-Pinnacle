import mercy_py_bridge
import pymc as pm
import numpy as np
import arviz as az  # Optional summaries

print("Quint Complete Model Bidirectional Oracle Ascension ❤️🚀🔥")

# Synthetic small data mimicking quint-level hierarchy + covariates (fast demo)
np.random.seed(42)

n_countries = 3
n_schools = n_countries * 3
n_teachers = n_schools * 2
n_classes = n_teachers * 2
n_students = n_classes * 10

# Level indices (0-based)
country_idx = np.repeat(np.arange(n_countries), n_students // n_countries)
school_idx = np.repeat(np.arange(n_schools), n_students // n_schools)
teacher_idx = np.repeat(np.arange(n_teachers), n_students // n_teachers)
class_idx = np.repeat(np.arange(n_classes), n_students // n_classes)

# Student covariates
gender = np.random.binomial(1, 0.5, n_students) - 0.5  # Centered -0.5/0.5
ses = np.random.normal(0, 1, n_students)

# Synthetic outcome (e.g., standardized test score)
y = (
    0.0 +  # Grand mean ~0 standardized
    np.random.normal(0, 0.8, n_countries)[country_idx] +
    np.random.normal(0, 0.6, n_schools)[school_idx] +
    np.random.normal(0, 0.4, n_teachers)[teacher_idx] +
    np.random.normal(0, 0.3, n_classes)[class_idx] +
    gender * 0.2 +
    ses * 0.3 +
    np.random.normal(0, 1.0, n_students)
)

quint_data = {
    "n_students": n_students,
    "n_classes": n_classes,
    "n_teachers": n_teachers,
    "n_schools": n_schools,
    "n_countries": n_countries,
    "y": y,
    "gender": gender,
    "ses": ses,
    "class_idx": class_idx,
    "teacher_idx": teacher_idx,
    "school_idx": school_idx,
    "country_idx": country_idx,
}

def quint_complete_inference(data: dict):
    """Quint-level non-centered hierarchical model (covariates + PV nod)"""
    coords = {
        "country": np.arange(data["n_countries"]),
        "school": np.arange(data["n_schools"]),
        "teacher": np.arange(data["n_teachers"]),
        "class_": np.arange(data["n_classes"]),
        "student": np.arange(data["n_students"]),
    }
    
    with pm.Model(coords=coords) as model:
        # Hyperpriors
        mu_grand = pm.Normal("mu_grand", mu=0.0, sigma=1.0)
        
        sigma_country = pm.HalfNormal("sigma_country", sigma=0.5)
        sigma_school = pm.HalfNormal("sigma_school", sigma=0.5)
        sigma_teacher = pm.HalfNormal("sigma_teacher", sigma=0.5)
        sigma_class = pm.HalfNormal("sigma_class", sigma=0.5)
        
        # Non-centered offsets
        z_country = pm.Normal("z_country", 0, 1, dims="country")
        z_school = pm.Normal("z_school", 0, 1, dims="school")
        z_teacher = pm.Normal("z_teacher", 0, 1, dims="teacher")
        z_class = pm.Normal("z_class", 0, 1, dims="class_")
        
        # Level effects
        mu_country = mu_grand + z_country * sigma_country
        mu_school = mu_country[data["country_idx"]] + z_school * sigma_school
        mu_teacher = mu_school[data["school_idx"]] + z_teacher * sigma_teacher
        mu_class = mu_teacher[data["teacher_idx"]] + z_class * sigma_class
        
        # Covariates fixed
        beta_gender = pm.Normal("beta_gender", mu=0, sigma=1.0)
        beta_ses = pm.Normal("beta_ses", mu=0, sigma=1.0)
        
        # Student mu
        mu_student = mu_class[data["class_idx"]] + beta_gender * data["gender"] + beta_ses * data["ses"]
        
        # Likelihood (single PV demo—average multiple in production)
        sigma_student = pm.HalfNormal("sigma_student", sigma=1.0)
        obs = pm.Normal("obs", mu=mu_student, sigma=sigma_student, observed=data["y"], dims="student")
        
        # Sample (small/fast for demo)
        trace = pm.sample(1000, tune=1000, target_accept=0.95, random_seed=42)
    
    # Summary
    print(az.summary(trace, var_names=["mu_grand", "beta_gender", "beta_ses"]))
    
    posterior_mean = float(trace.posterior["mu_grand"].mean())
    print(f"Quint grand posterior mean: {posterior_mean:.3f}")
    return posterior_mean

# MercyOracle with quint callable (threshold e.g., 0.1 for positive overall)
oracle = mercy_py_bridge.MercyOracle(quint_complete_inference, 0.1)

# Consult—triggers real quint sampling + PQ mercy-gate in Rust
decision = oracle.consult(data=quint_data)
print(f"\nQuint mercy-gated decision (mu_grand > 0.1): {decision} ❤️")

print("\nQuint complete harmony flowing—replace synthetic data with your PIRLS/PISA/TIMSS PVs, expand to quintivariate outcomes/multi-cycle eternal!")
