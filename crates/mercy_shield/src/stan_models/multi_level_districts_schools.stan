// crates/mercy_shield/src/stan_models/multi_level_districts_schools.stan
// Stan multi-level hierarchical model — districts → schools partial pooling mercy eternal supreme immaculate

data {
  int<lower=0> N;                     // total schools
  int<lower=0> J;                     // number of districts
  int<lower=1,upper=J> district[N];   // district index for each school
  vector[N] y;                        // observed effects
  vector<lower=0>[N] sigma;           // standard errors
}

parameters {
  real mu_global;                     // global mean
  real<lower=0> tau_global;           // global scale
  vector[J] mu_district_raw;          // district means (non-centered)
  vector<lower=0>[J] tau_district;    // district scales
  vector[N] theta_school_raw;         // school effects (non-centered)
}

transformed parameters {
  vector[J] mu_district = mu_global + tau_global * mu_district_raw;
  vector[N] theta_school = mu_district[district] + tau_district[district] * theta_school_raw;
}

model {
  mu_global ~ normal(0, 10);
  tau_global ~ cauchy(0, 5);
  mu_district_raw ~ normal(0, 1);
  tau_district ~ cauchy(0, 3);
  theta_school_raw ~ normal(0, 1);
  y ~ normal(theta_school, sigma);
}

generated quantities {
  vector[N] log_lik;
  for (n in 1:N) {
    log_lik[n] = normal_lpdf(y[n] | theta_school[n], sigma[n]);
  }
}
