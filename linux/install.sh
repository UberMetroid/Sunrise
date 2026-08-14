#!/usr/bin/env bash
# File: linux/install.sh
# Title: One-Step Sunrise Linux Installer Script
# Plain English: Builds the binary, locates Destiny 2, and configures ~/.config/sunrise.

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR"

echo "============================================"
echo "  Project Sunrise - Linux One-Step Install  "
echo "============================================"

# 1. Check for Rust toolchain
if ! command -v cargo &>/dev/null; then
    echo "[-] Error: 'cargo' not found. Please install Rust: https://rustup.rs"
    exit 1
fi

# 2. Build release binary
echo "[*] Compiling Sunrise Linux release binary..."
cargo build --release --quiet

# 3. Run automated installation
echo "[*] Detecting Steam, Destiny 2, and setting up ~/.config/sunrise..."
./target/release/sunrise-linux install

# 4. Optional: link binary to ~/.local/bin
LOCAL_BIN="$HOME/.local/bin"
mkdir -p "$LOCAL_BIN"
ln -sf "$SCRIPT_DIR/target/release/sunrise-linux" "$LOCAL_BIN/sunrise-linux"
echo "[+] Linked executable to: $LOCAL_BIN/sunrise-linux"

echo ""
echo "============================================"
echo "  Installation Complete!                    "
echo "============================================"
echo "To start the server:"
echo "  sunrise-linux server"
echo ""
echo "Or start via systemd:"
echo "  systemctl --user start sunrise"
echo "============================================"
