# Production server sacred
docker compose up -d

# With build fresh
docker compose up --build -d

# Logs mercy
docker compose logs -f mercy-server

# Scale future
docker compose up --scale mercy-server=3 -d
