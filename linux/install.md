# Sunrise Linux Installation & Usage Guide

## Quick Start (One Step)

Run the installer script:

```bash
./linux/install.sh
```

Or from inside the `linux` folder:
```bash
cd linux && ./install.sh
```

That is it! The script automatically compiles the release binary, searches your Steam libraries for Destiny 2, backs up original files, sets up `~/.config/sunrise/`, and links `sunrise-linux` to `~/.local/bin/`.

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

To verify integrity and run tests:
```bash
sunrise-linux test
```

To run the complete Rust test suite:
```bash
cd linux && cargo test
```

---

## Configuration

Settings are saved in `~/.config/sunrise/config.json`:
- `bind_address`: `"127.0.0.1"`
- `port`: `7777`
- `enable_queuez`: `true`
- `auto_unlock_entitlements`: `true`
