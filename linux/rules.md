# Project Sunrise // Linux Foundry Rules & Architecture Standards

This document establishes the official engineering guidelines, user preferences, architectural boundaries, and coding standards for the Linux emulation suite of **Project Sunrise** (`linux/`).

---

## 1. Architectural Isolation & Upstream Purity

1. **Strict Subdirectory Confinement:**
   - All Linux-specific binaries, Rust crates, scripts, assets, and documentation must reside **100% inside the `linux/` directory**.
   - The repository root (`Sunrise/`, `Sunrise.sln`, `Sunrise.vcxproj`, etc.) must remain identical to upstream `stanuwu/Sunrise` to ensure clean upstream tracking and merging.

2. **Artifact Locations:**
   - Configuration & Runtime Cache: `~/.config/sunrise/`
   - Global Binary: `~/.local/bin/sunrise-linux`
   - Application Menu Launchers: `~/.local/share/applications/`
   - Scalable Vector Icons: `~/.local/share/icons/hicolor/scalable/apps/sunrise.svg`

---

## 2. Code Quality & File Size Constraints

1. **Max File Size Limit:**
   - Every Rust source file (`*.rs`) must strictly contain **$\le 256$ lines of code**.
   - When a module approaches this limit, it must be split across clean functional boundaries into submodules.

2. **Zero Compiler Warnings:**
   - All builds (`cargo check`, `cargo build`, `cargo test`, `cargo build --release`) must compile with **0 warnings** and 0 errors.

3. **Documentation & Naming:**
   - AST identifiers and comments must be written in clear, plain English.
   - Every source file begins with a standardized 3-line header:
     - `// File: <path>`
     - `// Title: <title>`
     - `// Plain English: <summary>`

---

## 3. Privilege Escalation & Terminal Formatting

1. **No Interactive Sudo Prompts:**
   - Never execute commands requiring interactive `sudo` authentication. If root privileges are required, provide the command for the user to run in their own terminal or offer non-sudo alternatives.

2. **Manual Package Validation:**
   - Never use `-y` or `--assumeyes` flags in DNF or package manager commands. Allow the user to review transaction summaries before confirming.

3. **Word Wrapping Constraint:**
   - Keep shell command lines under **80 characters** to prevent awkward word wrapping in terminal code blocks. Use backslashes (`\`) for multi-line commands when needed.

---

## 4. Git Versioning & Commit Protocol

1. **Continuous Version Bumps:**
   - Every modification, new feature, or bugfix must increment the version number across all three canonical locations:
     - `linux/Cargo.toml` (`version = "x.y.z"`)
     - `linux/src/lib.rs` (`pub const SUNRISE_LINUX_VERSION = "x.y.z";`)
     - `linux/src/settings/config.rs` (`version: "x.y.z".to_string()`)

2. **GitHub Synchronization:**
   - Every update must be staged (`git add linux/`), committed with a clear descriptive message, and pushed directly to `origin master`.

---

## 5. System Integration & Desktop Standards

1. **Start Menu Exclusivity:**
   - Application shortcuts must be installed directly into `~/.local/share/applications/` (`destiny2-sunrise.desktop` and `sunrise-server.desktop`).
   - Desktop directory (`~/Desktop`) must remain clean unless explicitly requested.

2. **Client Proxy Automation:**
   - The installer must automatically handle backing up original game DLLs (`steam_api64_original.dll`) and translocating the Project Sunrise proxy core (`steam_api64.dll`) into `bin/x64/`.

3. **Proton / Wine Launch Parameter:**
   - The required Steam launch override is:
     ```bash
     WINEDLLOVERRIDES="steam_api64=n,b" %command%
     ```
