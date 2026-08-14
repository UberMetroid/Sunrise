#!/usr/bin/env bash
# File: Linux-Server/install.sh
# Title: Sunrise Linux One-Line Web & Local Installer Script
# Plain English: Professional Linux installer guided step-by-step by your Ghost.

set -e
trap 'rm -rf "${CLONE_DIR:-/tmp/empty}" 2>/dev/null; exit 1' ERR

# Parse --yes / -y and --with-manifest for non-interactive (curl | bash)
ASSUME_YES=0
WITH_MANIFEST=0
for arg in "$@"; do
    case "$arg" in
        -y|--yes) ASSUME_YES=1 ;;
        --with-manifest) WITH_MANIFEST=1 ;;
    esac
done

# ANSI Color Palette
CYAN="\033[1;36m"
YELLOW="\033[1;33m"
WHITE="\033[1;37m"
GREEN="\033[1;32m"
RED="\033[1;31m"
DIM="\033[38;5;240m"
RESET="\033[0m"

ghost_box() {
    local text="$1"
    echo -e "${YELLOW}╭─ Ghost ─────────────────────────────────────────────────────────────╮${RESET}"
    echo -e "$text" | fold -s -w 66 | while IFS= read -r line; do
        echo -e "  ${WHITE}${line}${RESET}"
    done
    echo -e "${YELLOW}╰─────────────────────────────────────────────────────────────────────╯${RESET}"
}

echo -e "${CYAN}"
cat << "EOF"
                 /\
                /  \
           /\  / /\ \  /\           PROJECT SUNRISE // LINUX FOUNDRY
          /  \/ /  \ \/  \          ================================
         / /\  / /\ \  /\ \         "Eyes up, Guardian. We found a signal."
        / /  \/ /  \ \/  \ \
       < <    | ( O ) |   > >       Offline BAP Emulation & Sandbox
        \ \  /\ \  / /\  / /
         \ \/  \ \/ /  \/ /
          \  /\ \  / /\  /
           \/  \ \/ /  \/
                \  /
                 \/
EOF
echo -e "${RESET}"

echo -e "${CYAN}[STEP 0/5]${RESET} ${WHITE}ENVIRONMENT & FOUNDRY PRE-FLIGHT${RESET}"
echo -e "${DIM}───────────────────────────────────────────────────────────────────────${RESET}"

# 1. Check for prerequisite core tools (git, curl, cargo)
for tool in git curl; do
    if ! command -v "$tool" &>/dev/null; then
        echo -e "  ${RED}[ FAIL ]${RESET} ${WHITE}Required utility '$tool' was not found.${RESET}"
        ghost_box "\"We need '$tool' to pull telemetry and game blueprints. Please install it with your package manager.\""
        exit 1
    fi
done
echo -e "  ${GREEN}[  OK  ]${RESET} ${WHITE}System Utilities:${RESET} git, curl verified"

if ! command -v cargo &>/dev/null; then
    echo -e "  ${RED}[ FAIL ]${RESET} ${WHITE}Rust toolchain ('cargo') was not found.${RESET}"
    ghost_box "\"Guardian down! The Rust weapon foundry ('cargo') is missing. Please install Rust via 'curl https://sh.rustup.rs -sSf | sh' to continue.\""
    exit 1
fi
echo -e "  ${GREEN}[  OK  ]${RESET} ${WHITE}Rust Foundry Detected:${RESET} $(cargo --version | cut -d' ' -f1,2)"

# 2. Determine execution context (local checkout vs remote curl pipe)
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" 2>/dev/null && pwd || echo "")"

if [ -n "$SCRIPT_DIR" ] && [ -f "$SCRIPT_DIR/Cargo.toml" ]; then
    BUILD_DIR="$SCRIPT_DIR"
    echo -e "  ${GREEN}[  OK  ]${RESET} ${WHITE}Local Workspace:${RESET} $BUILD_DIR"
elif [ -n "$SCRIPT_DIR" ] && [ -f "$SCRIPT_DIR/Linux-Server/Cargo.toml" ]; then
    BUILD_DIR="$SCRIPT_DIR/Linux-Server"
    echo -e "  ${GREEN}[  OK  ]${RESET} ${WHITE}Root Workspace:${RESET} $BUILD_DIR"
else
    SUNRISE_REPO="${SUNRISE_REPO:-https://github.com/UberMetroid/Sunrise.git}"
    SUNRISE_BRANCH="${SUNRISE_BRANCH:-master}"
    # Allow-list to prevent evil repo injection via SUNRISE_REPO env
    case "$SUNRISE_REPO" in
        https://github.com/UberMetroid/Sunrise.git|https://github.com/stanuwu/Sunrise.git) ;;
        *) echo -e "  ${RED}[ FAIL ]${RESET} SUNRISE_REPO not allow-listed: $SUNRISE_REPO"; exit 1 ;;
    esac
    CLONE_DIR="$HOME/.cache/sunrise-build"

    ghost_box "\"Transmitting beacon coordinates to $SUNRISE_REPO... Pulling down the latest Vanguard emulation blueprints.\""
    echo -e "  ${YELLOW}[ SYNC ]${RESET} ${WHITE}Downloading repository:${RESET} $SUNRISE_REPO ($SUNRISE_BRANCH)"
    rm -rf "$CLONE_DIR"
    mkdir -p "$CLONE_DIR"
    if ! git clone --depth 1 --branch "$SUNRISE_BRANCH" "$SUNRISE_REPO" \
        "$CLONE_DIR" --quiet; then
        echo -e "  ${RED}[ FAIL ]${RESET} git clone failed"
        exit 1
    fi
    if [ -d "$CLONE_DIR/Linux-Server" ]; then
        BUILD_DIR="$CLONE_DIR/Linux-Server"
    else
        echo -e "  ${RED}[ FAIL ]${RESET} Linux-Server not found in clone"
        exit 1
    fi
    echo -e "  ${GREEN}[  OK  ]${RESET} ${WHITE}Source Synchronized:${RESET} $BUILD_DIR"
fi

# 3. Build release binary (with error visibility + disk check)
echo -e "\n${CYAN}[FOUNDRY]${RESET} ${WHITE}COMPILING NATIVE LINUX RELEASE BINARY...${RESET}"
echo -e "${DIM}───────────────────────────────────────────────────────────────────────${RESET}"
cd "$BUILD_DIR"
# Require ~500MB free for release build
if command -v df &>/dev/null; then
    avail_kb=$(df -k "$BUILD_DIR" | awk 'NR==2 {print $4}')
    if [ -n "$avail_kb" ] && [ "$avail_kb" -lt 500000 ]; then
        echo -e "  ${YELLOW}[ WARN ]${RESET} Low disk space: ${avail_kb}KB free, 500MB recommended"
    fi
fi
if ! cargo build --release 2>&1 | tee /tmp/sunrise-build.log; then
    echo -e "  ${RED}[ FAIL ]${RESET} cargo build failed — see /tmp/sunrise-build.log"
    ghost_box "\"Foundry failure. Check Rust logs and free disk space.\""
    exit 1
fi
echo -e "  ${GREEN}[  OK  ]${RESET} ${WHITE}Sunrise Linux Foundry Build Complete${RESET}"

# Verify binary exists
if [ ! -x "$BUILD_DIR/target/release/sunrise-linux" ]; then
    echo -e "  ${RED}[ FAIL ]${RESET} Binary not found after build"
    exit 1
fi

# 4. Link executable to ~/.local/bin and configure PATH
LOCAL_BIN="$HOME/.local/bin"
mkdir -p "$LOCAL_BIN"
ln -sf "$BUILD_DIR/target/release/sunrise-linux" "$LOCAL_BIN/sunrise-linux"
echo -e "  ${GREEN}[  OK  ]${RESET} ${WHITE}Global Executable Linked:${RESET} $LOCAL_BIN/sunrise-linux"

if [[ ":$PATH:" != *":$LOCAL_BIN:"* ]]; then
    # Detect shell rc
    SHELL_RC="$HOME/.bashrc"
    if [ -n "${ZSH_VERSION:-}" ] || [ "$(basename "$SHELL")" = "zsh" ]; then
        SHELL_RC="$HOME/.zshrc"
    elif [ "$(basename "$SHELL")" = "fish" ]; then
        SHELL_RC="$HOME/.config/fish/config.fish"
        if [ -f "$SHELL_RC" ] && ! grep -q "$LOCAL_BIN" "$SHELL_RC"; then
            echo "fish_add_path $LOCAL_BIN" >> "$SHELL_RC"
            echo -e "  ${GREEN}[  OK  ]${RESET} ${WHITE}Added $LOCAL_BIN to $SHELL_RC${RESET}"
        fi
    fi
    if [ -f "$SHELL_RC" ] && ! grep -q 'export PATH="$HOME/.local/bin:$PATH"' "$SHELL_RC"; then
        if [ "$SHELL_RC" != "$HOME/.config/fish/config.fish" ]; then
            echo 'export PATH="$HOME/.local/bin:$PATH"' >> "$SHELL_RC"
            echo -e "  ${GREEN}[  OK  ]${RESET} ${WHITE}Added ~/.local/bin to $SHELL_RC PATH${RESET}"
        fi
    fi
    export PATH="$LOCAL_BIN:$PATH"
fi

# 5. Run automated step-by-step installation with Ghost companion
if [ "$ASSUME_YES" = 1 ]; then
    "$BUILD_DIR/target/release/sunrise-linux" install --yes
else
    "$BUILD_DIR/target/release/sunrise-linux" install
fi

# 6. Optional: fetch public Bungie manifest (anonymous, no account, opt-in only)
if [ "$WITH_MANIFEST" = 1 ]; then
    echo -e "\n${CYAN}[MANIFEST]${RESET} ${WHITE}Syncing public Bungie manifest (anonymous, no account)...${RESET}"
    if "$BUILD_DIR/target/release/sunrise-linux" sync-manifest 2>&1 | tee /tmp/sunrise-manifest.log; then
        echo -e "  ${GREEN}[  OK  ]${RESET} Manifest sync complete"
    else
        echo -e "  ${YELLOW}[ WARN ]${RESET} Manifest sync failed — server still boots from local vault"
    fi
fi

echo -e "${GREEN}=======================================================================${RESET}"
echo -e "${GREEN}  TRANSMAT STATUS: READY TO LAUNCH                                     ${RESET}"
echo -e "${GREEN}=======================================================================${RESET}"
echo -e "  Launch Terminal Server:   ${CYAN}sunrise-linux server${RESET}"
echo -e "  Launch Background Daemon: ${CYAN}systemctl --user start sunrise${RESET}"
echo -e "  Launch Destiny 2 Wrapper: ${CYAN}sunrise-game${RESET}"
echo -e "  Inspect Live Logs:        ${CYAN}journalctl --user -u sunrise -f${RESET}"
echo -e "${GREEN}=======================================================================${RESET}\n"
