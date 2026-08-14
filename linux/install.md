# Sunrise Linux Installation & Usage Guide

This guide explains how to install, configure, and operate the Project Sunrise Linux runtime and local BAP emulation daemon.

---

## 1. Prerequisites

Ensure you have a working Rust toolchain (Rust 1.70 or newer):
```bash
cargo --version
rustc --version
```

---

## 2. Quick Installation

Run the automated installer from the `linux/` directory:

```bash
cd linux
cargo run -- install
```

### What the Installer Does Automatically:

1. **Destiny 2 Discovery:**
   - Scans default Steam library locations (`~/.local/share/Steam`, `~/.steam`, and Flatpak paths).
   - Parses `libraryfolders.vdf` to discover custom SSD or secondary drive Steam libraries.
   - Identifies the game root, asset packages (`packages/`), and binary directory (`bin/x64/`).

2. **XDG Configuration Directory Setup:**
   - Initializes `~/.config/sunrise/` adhering to the XDG Base Directory specification.
   - Creates default `config.json` configuration file.
   - Creates `profiles/` directory for offline character data.
   - Creates `cache/` directory for manifest index caches.

3. **Steam API Backup & Protection:**
   - Safely creates a backup copy of your original `steam_api64.dll` as `steam_api64_original.dll` in `bin/x64/`.
   - If a compiled `Sunrise.dll` client hook exists, installs it to `bin/x64/steam_api64.dll`.

4. **Desktop & systemd Integration:**
   - Installs a desktop launcher entry at `~/.local/share/applications/sunrise-server.desktop`.
   - Installs a systemd user service at `~/.config/systemd/user/sunrise.service`.

---

## 3. Operating the Server

### Method A: Run Directly in Terminal
```bash
# Start server on default port (127.0.0.1:7777)
cargo run -- server

# Start server on custom bind address and port
cargo run -- server 127.0.0.1 8888
```

### Method B: Manage via systemd User Service
```bash
# Start background server
systemctl --user start sunrise

# Check server status
systemctl --user status sunrise

# Stop background server
systemctl --user stop sunrise

# Enable automatic start on login (optional)
systemctl --user enable sunrise
```

---

## 4. Running Verification Diagnostics

To run the complete built-in self-test diagnostics:
```bash
cargo run -- test
```

To run the full unit and integration test suite:
```bash
cargo test
```

---

## 5. Configuration Reference

The server configuration file is stored at `~/.config/sunrise/config.json`:

```json
{
  "version": "0.2.4",
  "server": {
    "bind_address": "127.0.0.1",
    "port": 7777,
    "enable_queuez": true,
    "max_connections": 64
  },
  "auto_unlock_entitlements": true,
  "default_power_cap": 1000
}
```

| Setting | Default | Description |
| :--- | :--- | :--- |
| `bind_address` | `"127.0.0.1"` | IP address for the local BAP listener |
| `port` | `7777` | TCP port for BAP protocol connections |
| `enable_queuez` | `true` | Enables instant queue bypass response |
| `max_connections`| `64` | Maximum concurrent TCP client connections |
| `auto_unlock_entitlements` | `true` | Unlocks all seasons, expansions, and DLCs |
| `default_power_cap` | `1000` | Base power cap for character calculation |

---

## 6. Restoring Original Game Files

If you ever wish to restore the original Steam API DLL:
```bash
# In Destiny 2 bin/x64 directory:
cp bin/x64/steam_api64_original.dll bin/x64/steam_api64.dll
```
