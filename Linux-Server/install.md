# Sunrise Linux Installation & Usage Guide

## One-Line HTTPS Web Install

To install Project Sunrise on any Linux machine with a single terminal command:

```bash
curl -fsSL https://raw.githubusercontent.com/UberMetroid/Sunrise/master/Linux-Server/install.sh | bash
```

---

## Local Installation (From Cloned Repo)

If you already have this repository cloned locally:

```bash
./Linux-Server/install.sh
```

Or from inside the `Linux-Server` folder:
```bash
cd Linux-Server && ./install.sh
```

Non-interactive (for `curl | bash`):
```bash
curl -fsSL https://raw.githubusercontent.com/UberMetroid/Sunrise/master/Linux-Server/install.sh | bash -s -- --yes
```

---

## Steam Launch Parameter (Required for Proton / Wine)

In Steam, right-click **Destiny 2** -> **Properties** -> **Launch Options**, and paste:

```bash
WINEDLLOVERRIDES="steam_api64=n,b" %command%
```

This instructs Proton to load Sunrise's proxy `steam_api64.dll` rather than the default Wine stub.

---

## What the Installer Does Automatically

1. **Rust Toolchain Verification:** Checks for `cargo` and `rustc` + verifies `git`/`curl`.
2. **Destiny 2 & Steam Scanning:** Detects game packages and binary directories in Steam libraries (native, Flatpak `~/.var/app/com.valvesoftware.Steam/`, Snap `~/snap/steam/`, custom `libraryfolders.vdf`).
3. **Configuration Setup:** Initializes `~/.config/sunrise/` (XDG) and `~/Downloads/Destiny 2/Sunrise-manifest/` for cached manifest (local-only, git-ignored).
4. **Safety Backup:** Backs up your original `steam_api64.dll` to `steam_api64_original.dll` (validates non-empty).
5. **Path Linking:** Creates symlink at `~/.local/bin/sunrise-linux`, auto-adds to `~/.bashrc`/`~/.zshrc`/`fish` as needed.
6. **Build Verification:** Runs `cargo build --release` with log to `/tmp/sunrise-build.log` and checks disk space.
7. **Desktop & systemd Services:** Installed via `sunrise-linux install` (user service `systemctl --user`).

---

## Operating the Emulation Server

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

## Indexing Game Packages (Optional Ahead-of-Time Caching)

To scan and index all Destiny 2 package archives into `~/.config/sunrise/cache/`:
```bash
sunrise-linux index
```

---

## Diagnostics & Testing

To verify integrity and run self-tests:
```bash
sunrise-linux test
```

To run the complete Rust test suite:
```bash
cd Linux-Server && cargo test
```

---

## Manifest (Optional, Anonymous)

By default the server boots from your local vault (`~/Downloads/Destiny 2/packages/` + `~/Downloads/Destiny 2/Sunrise-manifest/`). No Bungie account is needed. To enrich the local item cache with the public Bungie manifest (anonymous, no `X-API-Key`, no account link):

```bash
sunrise-linux sync-manifest
# or during install
curl -fsSL https://raw.githubusercontent.com/UberMetroid/Sunrise/master/Linux-Server/install.sh | bash -s -- --with-manifest
```

This fetches `https://www.bungie.net/Platform/Destiny2/Manifest/` anonymously and caches to `~/.config/sunrise/manifest_cache.json` (git-ignored). Explore parity with `stanuwu` needs only the vault — manifest is enrichment only.

---

## Uninstalling & Restoring Original Files

To restore your original Steam API binaries and remove desktop shortcuts:
```bash
sunrise-linux uninstall
```

---

## Configuration Reference

Settings are stored at `~/.config/sunrise/config.json` and manifest at `~/.config/sunrise/manifest_cache.json` (or `~/Downloads/Destiny 2/Sunrise-manifest/bootstrap_manifest.json`):

```json
{
  "version": "0.6.4",
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
