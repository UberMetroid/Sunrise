# 🐳 Project Sunrise: Universal Self-Hosting Guide

A high-performance, containerized emulation server for **Project Sunrise** (Destiny 2 offline exploration sandbox).

Compatible with **Windows (Docker Desktop)**, **Linux (Docker & Podman)**, **macOS**, and home server environments (**Unraid, TrueNAS, Synology**).

---

## ⚡ Quick Start

Anyone on Windows, Linux, or macOS can start the server with a single command:

```bash
# 1. Clone the repository
git clone https://github.com/UberMetroid/Sunrise.git
cd Sunrise

# 2. Launch the emulation server
docker compose up -d

# 3. View live server logs
docker compose logs -f
```

---

## 🎮 Connecting Your Destiny 2 Client

### 1. Playing on the Same PC (Localhost)
* Launch the game via Steam with the Project Sunrise proxy hook (`steam_api64.dll`).
* The game client will automatically connect to `127.0.0.1:7777`.

### 2. Playing on a Windows PC with Server on LAN (Home Lab / Linux / NAS)
* Run the container on your server machine (e.g. IP `192.168.1.100`).
* Ensure port `7777` is allowed through your server's firewall.
* Windows clients on your home network can point to `192.168.1.100:7777`.

---

## 🛠️ Configuration & Environment Variables

| Variable | Default | Description |
|---|---|---|
| `SUNRISE_BIND_ADDRESS` | `0.0.0.0` | Listening interface (`0.0.0.0` listens across all LAN/WAN) |
| `SUNRISE_PORT` | `7777` | TCP port for BAP protocol telemetry |
| `RUST_LOG` | `info` | Log verbosity level (`info`, `debug`, `trace`) |

---

## 🛑 Stopping the Server

```bash
docker compose down
```
