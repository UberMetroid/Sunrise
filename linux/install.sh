#!/usr/bin/env bash
# File: linux/install.sh
# Title: Sunrise Linux One-Line Web & Local Installer Script
# Plain English: Installs Sunrise on Linux with Destiny 2 storytelling and Ghost dialogue.

set -e

# ANSI Color Codes
CYAN="\033[1;36m"
YELLOW="\033[1;33m"
WHITE="\033[1;37m"
GREEN="\033[1;32m"
RED="\033[1;31m"
RESET="\033[0m"

ghost_speak() {
    echo -e "${YELLOW}[Ghost]${RESET} ${WHITE}\"$1\"${RESET}"
}

ghost_alert() {
    echo -e "${RED}[Ghost]${RESET} ${WHITE}\"$1\"${RESET}"
}

echo -e "${CYAN}"
cat << "EOF"
            /\
           /  \
     /\   / /\ \   /\        PROJECT SUNRISE // LINUX FOUNDRY
    /  \ / /  \ \ /  \       ================================
   <    V | (o) | V   >      "Eyes up, Guardian."
    \  / \ \  / / \  /
     \/   \ \/ /   \/
           \  /
            \/
EOF
echo -e "${RESET}"

ghost_speak "Eyes up, Guardian. I'm reconstructing our offline link."

# 1. Check for Rust toolchain
if ! command -v cargo &>/dev/null; then
    ghost_alert "Guardian down! The Rust weapon foundry ('cargo') is missing."
    echo -e "    ${WHITE}Let me patch the link: please install Rust at https://rustup.rs${RESET}"
    exit 1
fi

# 2. Determine execution context (local checkout vs remote curl pipe)
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" 2>/dev/null && pwd || echo "")"

if [ -n "$SCRIPT_DIR" ] && [ -f "$SCRIPT_DIR/Cargo.toml" ]; then
    # Running inside local linux/ folder
    BUILD_DIR="$SCRIPT_DIR"
elif [ -n "$SCRIPT_DIR" ] && [ -f "$SCRIPT_DIR/linux/Cargo.toml" ]; then
    # Running from repository root
    BUILD_DIR="$SCRIPT_DIR/linux"
else
    # Running via curl | bash (remote pipeline)
    SUNRISE_REPO="${SUNRISE_REPO:-https://github.com/UberMetroid/Sunrise.git}"
    SUNRISE_BRANCH="${SUNRISE_BRANCH:-master}"
    BUILD_DIR="$HOME/.cache/sunrise-build/linux"

    ghost_speak "Transmitting beacon coordinates to $SUNRISE_REPO ($SUNRISE_BRANCH)..."
    rm -rf "$HOME/.cache/sunrise-build"
    mkdir -p "$HOME/.cache/sunrise-build"
    git clone --depth 1 --branch "$SUNRISE_BRANCH" "$SUNRISE_REPO" \
        "$HOME/.cache/sunrise-build" --quiet
fi

# 3. Build release binary
ghost_speak "Igniting the local foundry. Forging the Sunrise Linux binary..."
cd "$BUILD_DIR"
cargo build --release --quiet

# 4. Run automated installation (Steam detection & .config setup)
"$BUILD_DIR/target/release/sunrise-linux" install

# 5. Link executable to ~/.local/bin
LOCAL_BIN="$HOME/.local/bin"
mkdir -p "$LOCAL_BIN"
ln -sf "$BUILD_DIR/target/release/sunrise-linux" "$LOCAL_BIN/sunrise-linux"
echo -e "  ${CYAN}✦${RESET} ${WHITE}FOUNDRY LINK${RESET}: $LOCAL_BIN/sunrise-linux"

# 6. Ensure ~/Desktop shortcut is updated if ~/Desktop exists
if [ -d "$HOME/Desktop" ]; then
    cp -f "$HOME/.local/share/applications/sunrise-server.desktop" "$HOME/Desktop/sunrise-server.desktop" 2>/dev/null || true
    chmod +x "$HOME/Desktop/sunrise-server.desktop" 2>/dev/null || true
    echo -e "  ${CYAN}✦${RESET} ${WHITE}DESKTOP ICON${RESET}: $HOME/Desktop/sunrise-server.desktop"
fi

echo ""
echo -e "${GREEN}============================================${RESET}"
echo -e "${GREEN}  TRANSMAT STATUS: READY                    ${RESET}"
echo -e "${GREEN}============================================${RESET}"
echo -e "Launch server command: ${CYAN}sunrise-linux server${RESET}"
echo -e "Or systemd service:    ${CYAN}systemctl --user start sunrise${RESET}"
echo -e "${GREEN}============================================${RESET}"
