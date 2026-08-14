# 🐳 Project Sunrise: Containerized Destiny 2 Server

A lightweight, high-performance, containerized emulation server for **Project Sunrise** (Destiny 2 offline exploration sandbox).

---

## ⚡ Quick Start (Docker Compose)

The easiest way to self-host the server is with Docker Compose:

```bash
# 1. Clone the repository
git clone https://github.com/UberMetroid/Sunrise.git
cd Sunrise/linux

# 2. Start the container in background
docker compose up -d

# 3. View live server logs
docker compose logs -f
```

---

## 🚀 Running with Docker CLI

```bash
# Build the image locally
docker build -t sunrise-linux:latest .

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
podman build -t sunrise-linux:latest .
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

---

## 🎮 Connecting Clients

1. **Localhost:** If hosting on the same PC as your game client, Destiny 2 will connect directly to `127.0.0.1:7777`.
2. **Dedicated Server / LAN Host:**
   * Run the container on your server IP (e.g. `192.168.1.50`).
   * Ensure port `7777` is open on your firewall.
   * Forward client game traffic to your server IP or set up loopback routing.
