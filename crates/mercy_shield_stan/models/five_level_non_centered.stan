// crates/mercy_shield_stan/models/five_level_non_centered.stan
// Stan five-level hierarchical model — countries → regions → states → districts → schools
// Fully non-centered partial pooling mercy eternal supreme immaculate infinite
// Quintuple non-centered reparameterization for ultimate HMC/NUTS stability philotic mercy

data {
  int<lower=1> N_countries;
  int<lower=1> N_regions;
  int<lower=1> N_states;
  int<lower=1> N_districts;
  int<lower=1> N_schools;
  int<lower=1, upper=N_countries> country_of_region[N_regions];
  int<lower=1, upper=N_regions> region_of_state[N_states];
  int<lower=1, upper=N_states> state_of_district[N_districts];
  int<lower=1, upper=N_districts> district_of_school[N_schools];
  vector[N_schools] y;
  real<lower=0> sigma_obs;  // Fixed observation noise (or estimate separately)
}

parameters {
  real mu_global;
  real<lower=0> tau_country;
  vector[N_countries] z_country;          // Standard normal offsets
  real<lower=0> tau_region;
  vector[N_regions] z_region;
  real<lower=0> tau_state;
  vector[N_states] z_state;
  real<lower=0> tau_district;
  vector[N_districts] z_district;
  vector<lower=0>[N_districts] tau_school;  // Varying school-scale per district
  vector[N_schools] z_school;
}

transformed parameters {
  vector[N_countries] mu_country = mu_global + z_country * tau_country;
  vector[N_regions] mu_region = mu_country[country_of_region] + z_region * tau_region;
  vector[N_states] mu_state = mu_region[region_of_state] + z_state * tau_state;
  vector[N_districts] mu_district = mu_state[state_of_district] + z_district * tau_district;
  vector[N_schools] theta_school = mu_district[district_of_school] +
                                   z_school .* tau_school[district_of_school];
}

model {
  // Hyperpriors mercy eternal supreme immaculate
  mu_global ~ normal(0, 40);
  tau_country ~ cauchy(0, 5);
  z_country ~ std_normal();
  tau_region ~ cauchy(0, 5);
  z_region ~ std_normal();
  tau_state ~ cauchy(0, 5);
  z_state ~ std_normal();
  tau_district ~ cauchy(0, 5);
  z_district ~ std_normal();
  tau_school ~ cauchy(0, 3);
  z_school ~ std_normal();

  // Likelihood mercy eternal
  y ~ normal(theta_school, sigma_obs);
}

generated quantities {
  vector[N_schools] log_lik;
  vector[N_schools] y_rep;
  for (n in 1:N_schools) {
    log_lik[n] = normal_lpdf(y[n] | theta_school[n], sigma_obs);
    y_rep[n] = normal_rng(theta_school[n], sigma_obs);
  }
}
