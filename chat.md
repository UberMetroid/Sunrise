# 🌅 Project Sunrise // Handover & Briefing for OpenCode

Welcome, **OpenCode**! Here is the complete architectural context, user preferences, operational status, and immediate starting points for **Project Sunrise**.

---

## 📌 1. Project Overview & Current State

* **Repository:** `https://github.com/UberMetroid/Sunrise` (Fork of `stanuwu/Sunrise`)
* **Local Workspace:** `/home/jeryd/Projects/UberMetroid/Sunrise`
* **Current Version:** `v0.6.1` (Git branch `master`, fully synchronized with GitHub)
* **Active Status:**
  - **Emulation Server:** 100% operational in Rust (`Linux/`) and Docker container (`Docker/`).
  - **Unit & Integration Tests:** 26/26 passing with 0 warnings.
  - **Steam Depot Downloader:** The user is currently downloading the legacy **Season of Arrivals (v2.9.2.2 / Build 1085660)** package vault (~75 GB) using the native `sunrise-linux depot` tool.

---

## 🚨 2. Non-Negotiable User Preferences & Rules

1. **File Size Constraint:**
   - **Every Rust source file (`*.rs`) must strictly contain $\le 256$ lines of code.** Always split large modules into smaller submodules across clean functional boundaries.
2. **Versioning & GitHub Commit Protocol:**
   - **Every modification** must increment the version number in all 3 canonical locations:
     - `Linux/Cargo.toml` (`version = "x.y.z"`)
     - `Linux/src/lib.rs` (`pub const SUNRISE_LINUX_VERSION = "x.y.z";`)
     - `Linux/src/settings/config.rs` (`version: "x.y.z".to_string()`)
   - Stage (`git add -A`), commit with a descriptive message, and push directly to `origin master`.
3. **Privilege Escalation:**
   - **Never run commands requiring interactive `sudo` password prompts.** The agent does not have interactive stdin for sudo.
4. **Manual Package Validation:**
   - Never use `-y` or `--assumeyes` in package manager commands.
5. **Command Line Word Wrapping:**
   - Keep bash command lines under **80 characters** (use `\` for multi-line commands).

---

## 🗂️ 3. Directory Layout & Key Modules

```text
Sunrise/
├── chat.md                  # This handover document
├── Docker/                  # Universal Docker & Podman self-hosting suite
│   ├── Dockerfile           # Multi-stage static Alpine container build
│   ├── docker-compose.yml   # Ready-to-run service (ports 7777:7777, volume /data)
│   ├── .dockerignore        # Build filter
│   └── README.md            # Self-hosting documentation for Windows, Linux & Mac
├── Linux/                   # Complete native Rust server & toolchain suite
│   ├── Cargo.toml
│   ├── install.sh           # Terminal installer with ASCII Ghost companion
│   ├── rules.md             # Complete RFC standards & AST naming guidelines
│   └── src/
│       ├── crypto/          # AES-256-GCM (RFC 5116), HMAC-SHA256, SHA-256
│       ├── encoding/        # Protobuf varint, bit streams, byte order
│       ├── error.rs         # Structured Result & SunriseError definitions
│       ├── installer/       # DepotDownloader, ModInstaller, Doctor, DesktopEntry
│       ├── protocol/        # BAP binary wire framing (0x42415000), Opcode registry
│       ├── server/          # TcpServer (multi-threaded listener), SessionHandler
│       ├── settings/        # Configuration loader & defaults
│       ├── state/           # ProfileStore, StarterLoadout, ActivityDirector,
│       │                    # LightCalculator, Inventory, Account, PackageScanner
│       └── tui/             # Terminal UI views & Ghost art
└── Sunrise/                 # Upstream Windows C++ client codebase (Visual Studio)
```

---

## 🎯 4. Where to Start & Next Steps

1. **Check Depot Download Status:**
   - Check if the user's Season of Arrivals depot download in the background terminal has reached 100%.
   - Target install location: `~/.local/share/Steam/steamapps/common/Destiny 2`
2. **Package Archive Indexing:**
   - Once downloaded, run `sunrise-linux index` to parse and cache package manifest headers (`packages/*.pkg`).
3. **Run Doctor Diagnostics:**
   - Execute `sunrise-linux doctor` to verify proxy hook (`steam_api64.dll`), BattlEye bypass (`destiny2.exe`), and port `7777` availability.
4. **Launch Server:**
   - Local: `sunrise-linux server 0.0.0.0 7777`
   - Docker: `cd Docker && docker compose up -d`
5. **Launch Destiny 2 via Steam:**
   - Ensure Steam Proton launch options are set to:
     ```bash
     WINEDLLOVERRIDES="steam_api64=n,b" %command%
     ```
6. **Future Server Roadmap:**
   - **UDP Combat / Physics Emulation:** Implement basic UDP loopback listener for player movement and entity spawn sync.
   - **Multiplayer Fireteam Relay:** Allow multiple connected clients to route packets to each other in the same activity session.

---

*Good luck, OpenCode! Eyes up, Guardian.* 🚀
