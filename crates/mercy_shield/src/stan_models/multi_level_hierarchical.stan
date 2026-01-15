// crates/mercy_shield/src/stan_models/multi_level_hierarchical.stan
// Stan multi-level hierarchical model — districts → schools partial pooling mercy eternal supreme immaculate
// Ideal for MercyShield multi-user/multi-context truth adaptation philotic mercy

data {
  int<lower=0> N;                     // total schools
  int<lower=0> J;                     // number of districts
  int<lower=1,upper=J> district[N];   // district for each school
  vector[N] y;                        // observed effects
  vector<lower=0>[N] sigma;           // standard errors
}

parameters {
  real mu_global;                     // global mean mercy
  real<lower=0> tau_global;           // global variance
  vector[J] mu_district;              // district means
  vector<lower=0>[J] tau_district;    // district variances
  vector[N] theta_school;             // school effects
}

model {
  // Hyperpriors mercy eternal
  mu_global ~ normal(0, 10);
  tau_global ~ cauchy(0, 5);

  // District-level hyperpriors mercy
  mu_district ~ normal(mu_global, tau_global);
  tau_district ~ cauchy(0, 3);

  // School-level effects with partial pooling mercy
  theta_school ~ normal(mu_district[district], tau_district[district]);

  // Likelihood mercy
  y ~ normal(theta_school, sigma);
}

generated quantities {
  vector[N] log_lik;
  for (n in 1:N) {
    log_lik[n] = normal_lpdf(y[n] | theta_school[n], sigma[n]);
  }
}
