// models/real_timss_four_level_non_centered.stan
data {
  int<lower=1> N;                  // students
  int<lower=1> N_countries;
  int<lower=1> N_schools;
  int<lower=1> N_classes;
  int<lower=1, upper=N_countries> country[N];
  int<lower=1, upper=N_schools> school[N];
  int<lower=1, upper=N_classes> class[N];
  vector[N] y;                     // e.g., BSMMAT01
}
parameters {
  real mu_global;
  real<lower=0> tau_country;
  vector[N_countries] z_country;
  real<lower=0> tau_school;
  vector[N_schools] z_school;
  real<lower=0> tau_class;
  vector[N_classes] z_class;
  real<lower=0> sigma_obs;
}
transformed parameters {
  vector[N_countries] mu_country = mu_global + z_country * tau_country;
  vector[N_schools] mu_school = mu_country[country] + z_school * tau_school;
  vector[N_classes] mu_class = mu_school[school] + z_class * tau_class;
}
model {
  mu_global ~ normal(500, 100);
  tau_country ~ cauchy(0, 50);
  z_country ~ std_normal();
  tau_school ~ cauchy(0, 40);
  z_school ~ std_normal();
  tau_class ~ cauchy(0, 30);
  z_class ~ std_normal();
  sigma_obs ~ normal(0, 60);
  y ~ normal(mu_class[class], sigma_obs);
}
