import pandas as pd
import pyreadstat  # pip install pyreadstat for SAV files

# Load student data (replace with your downloaded file path)
df, meta = pyreadstat.read_sav("CY07_MSU_STU_QQQ.SAV")  # Example for PISA 2018

# Select key columns (adjust names per year/codebook)
df = df[['CNT', 'CNTSCHID', 'PV1MATH', 'ST004D01T']]  # Country, School ID, Math PV1, Student gender (optional covariate)
df = df.dropna(subset=['PV1MATH', 'CNTSCHID', 'CNT'])

# Optional: subset countries for faster sampling
countries_subset = ['USA', 'CAN', 'DEU', 'FRA', 'JPN', 'KOR']  # Example
df = df[df['CNT'].isin(countries_subset)]

# Map to indices
df['country_idx'] = df['CNT'].astype('category').cat.codes
df['school_idx'] = df.groupby('CNT')['CNTSCHID'].transform(lambda x: x.astype('category').cat.codes)
df['global_school_idx'] = df['school_idx'] + df['country_idx'] * df['school_idx'].max() + 1  # Unique global if needed

# Data for model
y = df['PV1MATH'].values
country_idx = df['country_idx'].values
school_idx = df['school_idx'].values  # Or global if varying per country
n_countries = df['country_idx'].nunique()
n_schools = df['school_idx'].nunique() + 1  # Adjust
