#!/usr/bin/env bash
# File: linux/install.sh
# Title: Sunrise Linux One-Line Web & Local Installer Script
# Plain English: Installs Sunrise on Linux from a local clone or via curl | bash.

set -e

echo "============================================"
echo "  Project Sunrise - Linux Installer         "
echo "============================================"

# 1. Check for Rust toolchain
if ! command -v cargo &>/dev/null; then
    echo "[-] Error: 'cargo' not found."
    echo "    Please install Rust first: https://rustup.rs"
    exit 1
fi

# 2. Determine execution context (local checkout vs remote curl pipe)
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" 2>/dev/null && pwd || echo "")"

if [ -n "$SCRIPT_DIR" ] && [ -f "$SCRIPT_DIR/Cargo.toml" ]; then
    # Running inside a local linux/ folder
    BUILD_DIR="$SCRIPT_DIR"
elif [ -n "$SCRIPT_DIR" ] && [ -f "$SCRIPT_DIR/linux/Cargo.toml" ]; then
    # Running from the repository root
    BUILD_DIR="$SCRIPT_DIR/linux"
else
    # Running via curl | bash (remote pipeline)
    SUNRISE_REPO="${SUNRISE_REPO:-https://github.com/UberMetroid/Sunrise.git}"
    SUNRISE_BRANCH="${SUNRISE_BRANCH:-master}"
    BUILD_DIR="$HOME/.cache/sunrise-build/linux"

    echo "[*] Downloading Sunrise from $SUNRISE_REPO ($SUNRISE_BRANCH)..."
    rm -rf "$HOME/.cache/sunrise-build"
    mkdir -p "$HOME/.cache/sunrise-build"
    git clone --depth 1 --branch "$SUNRISE_BRANCH" "$SUNRISE_REPO" \
        "$HOME/.cache/sunrise-build"
fi

# 3. Build release binary
echo "[*] Compiling Sunrise Linux release binary..."
cd "$BUILD_DIR"
cargo build --release --quiet

# 4. Run automated installation (Steam detection & .config setup)
echo "[*] Detecting Steam, Destiny 2, and setting up ~/.config/sunrise..."
"$BUILD_DIR/target/release/sunrise-linux" install

# 5. Link executable to ~/.local/bin
LOCAL_BIN="$HOME/.local/bin"
mkdir -p "$LOCAL_BIN"
ln -sf "$BUILD_DIR/target/release/sunrise-linux" "$LOCAL_BIN/sunrise-linux"
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
