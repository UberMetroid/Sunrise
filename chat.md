# 🌅 Project Sunrise // Handover & Briefing for OpenCode

Welcome, **OpenCode**! Here is the complete architectural context, user preferences, operational status, and immediate starting points for **Project Sunrise**.

---

## 📌 1. Project Overview & Current State

* **Repository:** `https://github.com/UberMetroid/Sunrise` (Fork of `stanuwu/Sunrise`)
* **Local Workspace:** `~/Projects/UberMetroid/Sunrise`
* **Current Version:** `v0.6.3` (Git branch `master`, fully synchronized with GitHub)
* **Active Status:**
  - **Emulation Server:** 100% operational in Rust (`Linux/`) and Docker container (`Docker/`).
  - **Unit & Integration Tests:** 56/56 passing with 0 warnings (39 prior + 17 new for UDP transport).
  - **Multiplayer Foundation Complete (Phase 1 + Phase 2):**
    - Multi-client TCP registry with Steam-linked identity, fireteam broadcast fanout, ephemeral fallback for steamless clients.
    - UDP transport (`UdpSocket`, port 7778 default, env `SUNRISE_UDP_PORT`) with `SUNU` magic + `BindUdp (0x0701)` handshake + `WorldState` snapshot echo per `PlayerPosition` packet.
  - **Next: Full D2 Mirror Roadmap** — entities, AI, combat, patrols, raids, vendors, quests, loot tables.
  - **Vault Download Complete:** The full **Season of Arrivals (v2.9.2.2 / Build 1085660)** package vault (~258 GB uncompressed across 4,443 `.pkg` archives) is 100% downloaded and indexed.
  - **Full Vault Backup Created:** Safely mirrored to `~/Downloads/Destiny 2`.
  - **Doctor Diagnostics:** `[✓] VANGUARD SYSTEM DIAGNOSTICS: ALL SYSTEMS OPERATIONAL`.

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
├── chat.md                  # Handover document
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
│       ├── protocol/        # BAP binary wire framing (0x42415000), Opcode registry,
│       │                    # UDP packet codecs & magic (`SUNU`)
│       ├── server/          # TcpServer (multi-threaded listener), UdpServer (combat sync),
│       │                    # SessionHandler, ClientRegistry, OutboundQueue, Fireteam,
│       │                    # WorldState, handlers/{signon, account, inventory, activity, misc}
│       ├── settings/        # Configuration loader & defaults
│       ├── state/           # ProfileStore, StarterLoadout, ActivityDirector,
│       │                    # LightCalculator, Inventory, Account, PackageScanner
│       └── tui/             # Terminal UI views & Ghost art
└── Sunrise/                 # Upstream Windows C++ client codebase (Visual Studio)
```

---

## 🎯 4. Launching the Game & Emulation Server

1. **Launch Server:**
   - Local: `sunrise-linux server 0.0.0.0 7777`
   - Docker: `cd Docker && docker compose up -d`
2. **Launch Destiny 2 via Steam:**
   - Ensure Steam Proton launch options are set to:
     ```bash
     WINEDLLOVERRIDES="steam_api64=n,b" %command%
     ```
   - Hit **Play** in Steam!

---

*Good luck, OpenCode! Eyes up, Guardian.* 🚀
