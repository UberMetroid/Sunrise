# 🐳 Project Sunrise: Containerized Destiny 2 Server

A lightweight, high-performance, containerized emulation server for **Project Sunrise** (Destiny 2 offline exploration sandbox).

---

## ⚡ Quick Start (Docker Compose)

Run the server with a single command from inside this directory:

```bash
# 1. Navigate to the container directory
cd linux/container

# 2. Build and launch the container in the background
docker compose up -d

# 3. View live server telemetry
docker compose logs -f
```

---

## 🚀 Running with Docker CLI

```bash
# Build the image (from linux/ directory)
docker build -f container/Dockerfile -t sunrise-linux:latest .

# Run the container
docker run -d \
  --name sunrise-server \
  -p 7777:7777 \
  -v sunrise-data:/data \
  --restart unless-stopped \
  sunrise-linux:latest
```

---

## 🦭 Running with Podman (Rootless)

```bash
podman build -f container/Dockerfile -t sunrise-linux:latest .
podman run -d \
  --name sunrise-server \
  -p 7777:7777 \
  -v sunrise-data:/data:Z \
  --restart unless-stopped \
  sunrise-linux:latest
```

---

## ⚙️ Configuration & Environment Variables

| Variable | Default | Description |
|---|---|---|
| `SUNRISE_BIND_ADDRESS` | `0.0.0.0` | IP interface to listen on (`0.0.0.0` for all LAN/WAN) |
| `SUNRISE_PORT` | `7777` | TCP port for BAP protocol telemetry |
| `RUST_LOG` | `info` | Log verbosity level (`info`, `debug`, `trace`) |
