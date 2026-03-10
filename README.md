# Skysclawper

![Skysclawper v.0.01](docs/images/header.png)

> **The Autonomous Agent that actually sees the private web.**
> *v.0.01 - "The Phantom"*

**Skysclawper** is a divergent fork of [SkyClaw](https://github.com/zkfmms/skyclaw) engineered for **stealth**, **authentication**, and **efficiency**.

While the original SkyClaw aims to be a general-purpose AI agent using headless browsers, Skysclawper focuses on solving the hardest problem in web agents: **accessing the web as YOU.**

---

## ⚡️ Core Divergence: Why Skysclawper?

The fundamental difference lies in how they interact with the web.

| Feature | Original SkyClaw | 🦞 Skysclawper |
| :--- | :--- | :--- |
| **Browser Engine** | **Chromiumoxide** (Headless Chrome) | **Nab** (Native HTTP/3 Client) |
| **Authentication** | Manual login / Unstable | **Cookie Injection** (Brave/Chrome/Safari) |
| **Footprint** | Heavy (Requires Chrome) | **Ultra-Light** (Rust Binary) |
| **Detection** | Easily detected as Bot | **Stealth** (TLS/Fingerprint Spoofing) |
| **Philosophy** | "Automate the Browser" | "Extract the Data" |

### 1. The "Nab" Engine Integration
Instead of launching a heavy, detectable headless browser, Skysclawper integrates [Nab](https://github.com/MikkoParkkola/nab). This allows the agent to:
*   **Steal your cookies** (locally & safely) to access GitHub, Slack, Notion, and X (Twitter) *as you*.
*   **Bypass Cloudflare** using realistic TLS fingerprinting and HTTP/3.
*   **Read clean Markdown** directly, skipping the noise of raw HTML.

### 2. Workspace Separation
Skysclawper decouples the **Brain** (Configuration/Persona) from the **Body** (Runtime).
*   **Runtime**: This repository (`skysclawper`). The muscle.
*   **Workspace**: [skysclawper-workspace](https://github.com/rosenthal/skysclawper-workspace). The personality.
This allows you to deploy multiple agents (e.g., a private assistant and a public bot) using the same efficient binary.

---

## 🚀 Key Features

*   **Authenticated Browsing**: Access private repositories, internal docs, and social media feeds that block standard scrapers.
*   **Conversational Control**: Native integration with Discord (and Telegram). Chat with your agent to trigger research tasks.
*   **Session Persistence**: Remembers context across restarts using SQLite and Vector memory.
*   **Self-Healing**: Capable of restarting its own sub-processes and managing its health.

---

## 🛠 Tech Stack

*   **Language**: Rust 🦀
*   **Core**: SkyClaw Agentic Runtime
*   **Network**: Nab (Hyper/Reqwest/Quinn)
*   **Memory**: SQLite + Vector Embeddings

---

## ⚙️ Quick Start

### 1. Installation

```bash
# Clone the runtime
git clone https://github.com/rosenthal/skysclawper.git
cd skysclawper

# Build (Release mode recommended for speed)
cargo build --release
```

### 2. Configuration

You need to set up the environment variables. We recommend using the **Workspace** approach, but for a quick test:

```bash
export DISCORD_BOT_TOKEN="your_token"
export GEMINI_API_KEY="your_key"
export NAB_BIN="/path/to/nab"  # If not in PATH
```

### 3. Run

```bash
./target/release/skyclaw
```

---

## 📦 Deployment & Updates

### Syncing with Upstream
We respect the rapid development of the original SkyClaw. To keep Skysclawper updated with core improvements:

```bash
git remote add upstream https://github.com/zkfmms/skyclaw.git
git fetch upstream
git merge upstream/main
# Resolve conflicts in src/main.rs (Browser vs Nab logic)
```

### Deploying to VPS
Use the [skysclawper-workspace](https://github.com/rosenthal/skysclawper-workspace) deployment scripts to push your agent to `private` or `sns` environments.

---

## 📄 License

Inherits **MIT License** from SkyClaw and Nab.

> *Header art by User. Concept based on SkyClaw by zkfmms & Nab by MikkoParkkola.*
