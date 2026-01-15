import pandas as pd

# Local Data Mercy (Download from IEA site — public)
student_file = 'ASPIRL21.dta'  # Stata format — use pandas read_stata
teacher_file = 'ATGIRL21.dta'
school_file = 'ASGIRL21.dta'

df_student = pd.read_stata(student_file)
df_teacher = pd.read_stata(teacher_file)
df_school = pd.read_stata(school_file)

# Linking mercy
df = df_student.merge(df_teacher, on=['IDTEACH', 'IDCLASS', 'IDSCHOOL'], how='left')
df = df.merge(df_school, on=['IDSCHOOL'], how='left')

# Select Grade 4 Reading mercy
df = df[df['IDGRADE'] == 4]

# Outcome + PVs
y_cols = ['ASRREA01', 'ASRREA02', 'ASRREA03', 'ASRREA04', 'ASRREA05']  # 5 PVs
df['y_mean'] = df[y_cols].mean(axis=1)

# Save harmonized mercy
df.to_parquet('pirls21_grade4_reading_harmonized.parquet')
