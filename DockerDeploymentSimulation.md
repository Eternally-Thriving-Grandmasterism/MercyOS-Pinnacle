# 1. Build image
docker build -t mercyos-pinnacle:latest .

# 2. Run server container (localhost expose)
docker run -d -p 5000:5000/udp --name mercy-server mercyos-pinnacle:latest

# 3. Client connect from host/other container
# In another terminal: cargo run (client mode default)

# 4. Production: volume for persist, restart always
docker run -d -p 5000:5000/udp --restart unless-stopped mercyos-pinnacle:latest

# Troubleshooting: Port conflict? -p newport. Logs: docker logs mercy-server
