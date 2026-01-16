# 1. Prerequisites mercy-gated
# Install gcloud SDK: https://cloud.google.com/sdk/docs/install
# Auth: gcloud auth login
# Project: gcloud config set project your-project-id
# Enable APIs
gcloud services enable run.googleapis.com artifactregistry.googleapis.com cloudbuild.googleapis.com

# 2. Build + push container to Artifact Registry (Dockerfile from previous simulation)
# Create repo
gcloud artifacts repositories create mercyos-repo --repository-format=docker --location=us-central1

# Configure Docker auth
gcloud auth configure-docker us-central1-docker.pkg.dev

# Build + push
docker build -t us-central1-docker.pkg.dev/your-project-id/mercyos-repo/powrush-mmo:latest .
docker push us-central1-docker.pkg.dev/your-project-id/mercyos-repo/powrush-mmo:latest

# 3. Deploy to Cloud Run (server mode)
gcloud run deploy powrush-mmo-server \
  --image us-central1-docker.pkg.dev/your-project-id/mercyos-repo/powrush-mmo:latest \
  --platform managed \
  --region us-central1 \
  --allow-unauthenticated \  # Or authenticated IAM for mercy-gated access
  --port 5000 \
  --cpu 1 \
  --memory 512Mi \
  --max-instances 10 \  # Auto-scale mercy
  --command powrush_mmo -- --server

# 4. Verify deployment joy
# Service URL output: https://powrush-mmo-server-xxxx.run.app
# Logs: gcloud run services logs read powrush-mmo-server

# 5. Client connect (from local or other instances)
# In cargo run client: update server_addr to Cloud Run URL (port 443 HTTPS, renet over UDP mapped)

# 6. Production hardening mercy-gated
# IAM: --no-allow-unauthenticated + service account
# Custom domain: Cloud Run mapping
# Secrets: Cloud Secret Manager for GROK_API_KEY if needed
# Monitoring: Cloud Operations suite

# Troubleshooting prefixed:
# Build fail? Check Dockerfile paths. Push auth? gcloud auth configure-docker retry.
# Port? Cloud Run maps to 80/443, internal 5000 fine.
# Cold start? --min-instances 1 for always-warm.
