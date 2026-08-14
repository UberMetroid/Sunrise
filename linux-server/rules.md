# Project Sunrise // Linux Foundry Rules & Architecture Standards

This document establishes the official engineering guidelines, RFC compliance mandates, AST naming conventions, user preferences, and coding standards for the Linux emulation suite of **Project Sunrise** (`linux-server/`).

---

## 1. RFC Compliance & Open Standards

All network protocols, cryptographic operations, serialization formats, and operating system integrations must strictly adhere to the relevant IETF Request for Comments (RFC) standards and open specifications:

1. **Cryptographic Standards:**
   - **RFC 5116:** Authenticated Encryption with Associated Data (AEAD) using AES-128-GCM for secure packet payload encryption.
   - **RFC 2104:** HMAC: Keyed-Hashing for Message Authentication (HMAC-SHA256).
   - **RFC 6234:** US Secure Hash Algorithms (SHA and SHA-based HMAC) for SHA-256 digests.

2. **Network & Transport Protocols:**
   - **RFC 9293 (RFC 793):** Transmission Control Protocol (TCP) non-blocking socket handling, framing, and clean teardown.
   - **RFC 8259:** The JavaScript Object Notation (JSON) Data Interchange Format for configuration and cache persistence.

3. **Linux Desktop & System Specifications:**
   - **XDG Base Directory Specification:** Proper placement in `~/.config/sunrise/`, `~/.local/share/applications/`, and `~/.local/share/icons/`.
   - **XDG Desktop Entry Specification:** Standardized `.desktop` launcher keys, categories, and icon associations.

---

## 2. Simple AST Plain English Engineering Standards

1. **Descriptive Syntax Trees:**
   - All Abstract Syntax Tree (AST) definitions—including structs, enums, functions, traits, methods, fields, and variables—must use clear, simple, descriptive plain English.
   - Avoid cryptic abbreviations, acronyms, or obfuscated type names (e.g., use `BapFrameEnvelope` rather than `BFE`, `calculate_base_light` rather than `calc_bl`).

2. **Plain English Header Convention:**
   - Every `.rs` source file must begin with a standardized 3-line header:
     ```rust
     // File: <file_path>
     // Title: <Descriptive Component Title>
     // Plain English: <Clear 1-sentence explanation of what this file does>
     ```

3. **File Size Constraint:**
   - Every Rust source file (`*.rs`) must strictly contain **$\le 256$ lines of code**. Split large modules into submodules across clean functional boundaries.

4. **Zero Compiler Warnings:**
   - All builds (`cargo check`, `cargo build`, `cargo test`, `cargo build --release`) must compile with **0 warnings** and 0 errors.

---

## 3. Architectural Isolation & Upstream Purity

1. **Strict Subdirectory Confinement:**
   - All Linux-specific binaries, Rust crates, scripts, assets, Docker configs (`linux-server/Dockerfile`, `linux-server/docker-compose.yml`), and documentation must reside **100% inside the `linux-server/` directory**.
   - The repository root (`Sunrise/`, `Sunrise.sln`, `Sunrise.vcxproj`, etc.) must remain identical to upstream `stanuwu/Sunrise` to ensure clean upstream tracking and merging.

2. **Artifact Locations:**
   - Configuration & Runtime Cache: `~/.config/sunrise/`
   - Global Binary: `~/.local/bin/sunrise-linux`
   - Application Menu Launchers: `~/.local/share/applications/`
   - Scalable Vector Icons: `~/.local/share/icons/hicolor/scalable/apps/sunrise.svg`

---

## 4. Privilege Escalation & Terminal Formatting

1. **No Interactive Sudo Prompts:**
   - Never execute commands requiring interactive `sudo` authentication. If root privileges are required, provide the command for the user to run in their own terminal or offer non-sudo alternatives.

2. **Manual Package Validation:**
   - Never use `-y` or `--assumeyes` flags in DNF or package manager commands. Allow the user to review transaction summaries before confirming.

3. **Word Wrapping Constraint:**
   - Keep shell command lines under **80 characters** to prevent awkward word wrapping in terminal code blocks. Use backslashes (`\`) for multi-line commands when needed.

---

## 5. Git Versioning & Commit Protocol

1. **Continuous Version Bumps:**
   - Every modification, new feature, or bugfix must increment the version number across all three canonical locations:
     - `linux-server/Cargo.toml` (`version = "x.y.z"`)
     - `linux-server/src/lib.rs` (`pub const SUNRISE_LINUX_VERSION = "x.y.z";`)
     - `linux-server/src/settings/config.rs` (`version: "x.y.z".to_string()`)

2. **GitHub Synchronization:**
   - Every update must be staged (`git add linux-server/`), committed with a clear descriptive message, and pushed directly to `origin master`.

---

## 6. System Integration & Desktop Standards

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
