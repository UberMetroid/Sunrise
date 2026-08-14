# Sunrise Linux Installation & Usage Guide

## One-Line HTTPS Web Install

To install Project Sunrise on any Linux machine with a single terminal command:

```bash
curl -fsSL https://raw.githubusercontent.com/UberMetroid/Sunrise/master/linux/install.sh | bash
```

---

## Local Installation (From Cloned Repo)

If you already have this repository cloned locally:

```bash
./linux/install.sh
```

Or from inside the `linux` folder:
```bash
cd linux && ./install.sh
```

---

## What the Installer Does Automatically

1. **Rust Toolchain Verification:** Checks for `cargo` and `rustc`.
2. **Destiny 2 & Steam Scanning:** Detects game packages and binary directories in Steam libraries.
3. **Configuration Setup:** Initializes `~/.config/sunrise/` adhering to the XDG Base Directory specification.
4. **Safety Backup:** Backs up your original `steam_api64.dll` to `steam_api64_original.dll`.
5. **Path Linking:** Creates a symlink at `~/.local/bin/sunrise-linux` so the binary is available globally.
6. **Desktop & systemd Services:** Installs a desktop shortcut and user service for background execution.

---

## Starting the Server

### Option A: Run Directly in Terminal
```bash
sunrise-linux server
```

### Option B: Run as a Background Service
```bash
# Start background service
systemctl --user start sunrise

# Check service status
systemctl --user status sunrise

# Stop background service
systemctl --user stop sunrise
```

---

## Diagnostics & Testing

To verify integrity and run self-tests:
```bash
sunrise-linux test
```

To run the complete Rust test suite:
```bash
cd linux && cargo test
```

---

## Configuration Reference

Settings are stored at `~/.config/sunrise/config.json`:

```json
{
  "version": "0.2.6",
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
