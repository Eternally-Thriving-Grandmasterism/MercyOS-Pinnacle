# Example covariate prep (adjust variable names per PIRLS codebook/year)
df['gender_female'] = (df['ASBG01'] == 2).astype(int)  # 2=female typically; center to -0.5/0.5
df['gender_c'] = df['gender_female'] - df['gender_female'].mean()

# Home resources composite example: books at home (ASBH02A: 0-5 categories) + early tasks index
df['books_home'] = df['ASBH02A']  # 1=0-10 books ... 5=>200 books
df['early_tasks'] = df[['ASBH17A', 'ASBH17B', ...]].mean(axis=1)  # Average early literacy activities
df['home_resources'] = (df['books_home'] + df['early_tasks']) / 2
df['home_resources_c'] = (df['home_resources'] - df['home_resources'].mean()) / df['home_resources'].std()

# Covariate arrays
gender_c = df['gender_c'].values
home_resources_c = df['home_resources_c'].values
X_student = np.stack([gender_c, home_resources_c], axis=1)  # Shape: (N_students, 2)
