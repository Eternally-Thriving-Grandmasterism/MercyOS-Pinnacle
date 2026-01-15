import pandas as pd
import pyreadstat  # pip install pyreadstat for SAV files

# Load student data (replace with your downloaded file path, e.g., grade 8 student file)
df, meta = pyreadstat.read_sav("bsgXXXXXX.sav")  # Example filename pattern; check codebook

# Select key columns (adjust per year/codebook; example for grade 8 math)
df = df[['IDCNTRY', 'IDSCHOOL', 'IDCLASS', 'BSMMAT01']]  # Country, School ID, Class ID, Math PV1
df = df.dropna(subset=['BSMMAT01', 'IDCLASS', 'IDSCHOOL', 'IDCNTRY'])

# Optional: subset countries for faster sampling (e.g., 10-20 countries)
countries_subset = [840, 124, 276, 392, 410]  # Example ISO codes: USA, CAN, DEU, JPN, KOR
df = df[df['IDCNTRY'].isin(countries_subset)]

# Map to consecutive indices for modeling
df['country_idx'] = df['IDCNTRY'].astype('category').cat.codes
df['school_idx'] = df.groupby('IDCNTRY')['IDSCHOOL'].transform(lambda x: x.astype('category').cat.codes)
df['class_idx'] = df.groupby(['IDCNTRY', 'IDSCHOOL'])['IDCLASS'].transform(lambda x: x.astype('category').cat.codes)

# Global indexing for classes/schools if needed (cumulative)
df['global_school_idx'] = df['country_idx'] * (df['school_idx'].max() + 1) + df['school_idx']
df['global_class_idx'] = df['global_school_idx'] * (df['class_idx'].max() + 1) + df['class_idx']

# Data for model
y = df['BSMMAT01'].values
country_idx = df['country_idx'].values
school_idx = df['global_school_idx'].values  # Or per-country if varying
class_idx = df['global_class_idx'].values
n_countries = df['country_idx'].nunique()
n_schools = df['global_school_idx'].nunique()
n_classes = df['global_class_idx'].nunique()
