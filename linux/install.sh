#!/usr/bin/env bash
# File: linux/install.sh
# Title: Sunrise Linux One-Line Web & Local Installer Script
# Plain English: Professional Linux installer guided step-by-step by your Ghost.

set -e

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
    echo -e "$text" | fold -s -w 64 | while IFS= read -r line; do
        printf "${YELLOW}│${RESET}  ${WHITE}%-64s${RESET} ${YELLOW}│${RESET}\n" "$line"
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
elif [ -n "$SCRIPT_DIR" ] && [ -f "$SCRIPT_DIR/linux/Cargo.toml" ]; then
    BUILD_DIR="$SCRIPT_DIR/linux"
    echo -e "  ${GREEN}[  OK  ]${RESET} ${WHITE}Root Workspace:${RESET} $BUILD_DIR"
else
    SUNRISE_REPO="${SUNRISE_REPO:-https://github.com/UberMetroid/Sunrise.git}"
    SUNRISE_BRANCH="${SUNRISE_BRANCH:-master}"
    BUILD_DIR="$HOME/.cache/sunrise-build/linux"

    ghost_box "\"Transmitting beacon coordinates to $SUNRISE_REPO... Pulling down the latest Vanguard emulation blueprints.\""
    echo -e "  ${YELLOW}[ SYNC ]${RESET} ${WHITE}Downloading repository:${RESET} $SUNRISE_REPO ($SUNRISE_BRANCH)"
    rm -rf "$HOME/.cache/sunrise-build"
    mkdir -p "$HOME/.cache/sunrise-build"
    git clone --depth 1 --branch "$SUNRISE_BRANCH" "$SUNRISE_REPO" \
        "$HOME/.cache/sunrise-build" --quiet
    echo -e "  ${GREEN}[  OK  ]${RESET} ${WHITE}Source Synchronized:${RESET} ~/.cache/sunrise-build"
fi

# 3. Build release binary
echo -e "\n${CYAN}[FOUNDRY]${RESET} ${WHITE}COMPILING NATIVE LINUX RELEASE BINARY...${RESET}"
echo -e "${DIM}───────────────────────────────────────────────────────────────────────${RESET}"
cd "$BUILD_DIR"
cargo build --release --quiet
echo -e "  ${GREEN}[  OK  ]${RESET} ${WHITE}Sunrise Linux Foundry Build Complete${RESET}"

# 4. Link executable to ~/.local/bin and configure PATH
LOCAL_BIN="$HOME/.local/bin"
mkdir -p "$LOCAL_BIN"
ln -sf "$BUILD_DIR/target/release/sunrise-linux" "$LOCAL_BIN/sunrise-linux"
echo -e "  ${GREEN}[  OK  ]${RESET} ${WHITE}Global Executable Linked:${RESET} $LOCAL_BIN/sunrise-linux"

if [[ ":$PATH:" != *":$LOCAL_BIN:"* ]]; then
    if [ -f "$HOME/.bashrc" ] && ! grep -q 'export PATH="$HOME/.local/bin:$PATH"' "$HOME/.bashrc"; then
        echo 'export PATH="$HOME/.local/bin:$PATH"' >> "$HOME/.bashrc"
        echo -e "  ${GREEN}[  OK  ]${RESET} ${WHITE}Added ~/.local/bin to ~/.bashrc PATH${RESET}"
    fi
    export PATH="$LOCAL_BIN:$PATH"
fi

# 5. Run automated step-by-step installation with Ghost companion
"$BUILD_DIR/target/release/sunrise-linux" install

echo -e "${GREEN}=======================================================================${RESET}"
echo -e "${GREEN}  TRANSMAT STATUS: READY TO LAUNCH                                     ${RESET}"
echo -e "${GREEN}=======================================================================${RESET}"
echo -e "  Launch Terminal Server:   ${CYAN}sunrise-linux server${RESET}"
echo -e "  Launch Background Daemon: ${CYAN}systemctl --user start sunrise${RESET}"
echo -e "  Launch Destiny 2 Wrapper: ${CYAN}sunrise-game${RESET}"
echo -e "  Inspect Live Logs:        ${CYAN}journalctl --user -u sunrise -f${RESET}"
echo -e "${GREEN}=======================================================================${RESET}\n"
