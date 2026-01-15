// models/real_pisa_three_level_non_centered.stan
data {
  int<lower=1> N;                  // students
  int<lower=1> N_countries;
  int<lower=1> N_schools;
  int<lower=1, upper=N_countries> country[N];
  int<lower=1, upper=N_schools> school[N];
  vector[N] y;                     // PV1MATH
}
parameters {
  real mu_global;
  real<lower=0> tau_country;
  vector[N_countries] z_country;
  real<lower=0> tau_school;
  vector[N_schools] z_school;
  real<lower=0> sigma_obs;
}
transformed parameters {
  vector[N_countries] mu_country = mu_global + z_country * tau_country;
  vector[N_schools] mu_school = mu_country[country] + z_school * tau_school;
}
model {
  mu_global ~ normal(500, 100);
  tau_country ~ cauchy(0, 50);
  z_country ~ std_normal();
  tau_school ~ cauchy(0, 30);
  z_school ~ std_normal();
  sigma_obs ~ normal(0, 50);
  y ~ normal(mu_school[school], sigma_obs);
}
