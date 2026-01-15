// crates/mercy_shield/src/stan_models/8_schools.stan
// Stan classic 8 schools hierarchical model — partial pooling mercy eternal supreme immaculate

data {
  int<lower=0> J;         // number of schools
  real y[J];              // estimated treatment effects
  real<lower=0> sigma[J]; // standard errors
}

parameters {
  real mu;                // population mean
  real<lower=0> tau;      // population scale
  vector[J] eta;          // school-level errors (non-centered)
  // real theta[J];       // school effects (centered version)
}

transformed parameters {
  vector[J] theta = mu + tau * eta;  // non-centered reparameterization mercy
}

model {
  mu ~ normal(0, 5);
  tau ~ cauchy(0, 5);
  eta ~ normal(0, 1);     // implies theta ~ normal(mu, tau)
  y ~ normal(theta, sigma);
}

generated quantities {
  vector[J] log_lik;
  for (j in 1:J) {
    log_lik[j] = normal_lpdf(y[j] | theta[j], sigma[j]);
  }
}
