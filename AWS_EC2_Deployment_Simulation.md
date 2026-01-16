# 1. Launch EC2 instance (t3.medium recommended, Ubuntu 22.04)
# Security group: Inbound UDP 5000 (multiplayer), SSH 22

# 2. SSH connect
ssh -i your-key.pem ubuntu@ec2-public-ip

# 3. Install deps (same as Linux server)
sudo apt update && sudo apt upgrade -y
sudo apt install -y git curl build-essential pkg-config libssl-dev cmake clang docker.io

# 4. Docker install if not (ubuntu)
sudo snap install docker  # or apt

# 5. Clone + build image
git clone https://github.com/Eternally-Thriving-Grandmasterism/MercyOS-Pinnacle.git
cd MercyOS-Pinnacle
sudo docker build -t mercyos-pinnacle:latest .

# 6. Run server
sudo docker run -d -p 5000:5000/udp --name mercy-server --restart unless-stopped mercyos-pinnacle:latest

# 7. Verify
sudo docker logs -f mercy-server

# 8. Auto-start on boot (systemd or docker compose future)
# Security: Restrict SG to your IP for SSH, open UDP 5000 worldwide for clients

# Troubleshooting: Docker perm? sudo usermod -aG docker ubuntu + relog. Port blocked? SG rules.
