# SkyClaw x Nab (Skysclawper)
### The Autonomous Agent that actually sees the private web.

This project is a fusion of **SkyClaw** (Autonomous Agent Runtime) and **Nab** (Authenticated Web Extraction). It solves the "auth wall" problem that plagues most AI agents by leveraging a lightweight, browser-based extraction engine capable of using your existing session cookies.

## 🚀 Key Features

*   **Authenticated Browsing**: Uses `nab` to access pages behind login screens (GitHub, Slack, Google Workspace) by leveraging cookies from your local browser (Brave, Chrome, Safari).
*   **Anti-Bot Evasion**: Employs advanced fingerprinting (HTTP/3, TLS impersonation) to bypass Cloudflare and other bot protections.
*   **Hyper-Lean Architecture**: Built on Rust, designed for low memory footprint and high performance.
*   **Conversational Control**: Native Discord integration allows you to control the agent, send files, and receive reports via natural language.

## 🛠 Tech Stack

*   **Core**: [SkyClaw](https://github.com/nagisanzenin/skyclaw) (Rust Agentic Runtime)
*   **Extraction Engine**: [Nab](https://github.com/nagisanzenin/nab) (Markdown Transformer & Browser Engine)
*   **Memory**: Session-persistent memory (SQLite/Vector)

## 📦 Prerequisites

1.  **Rust**: Stable toolchain (`cargo`).
2.  **Nab Binary**: You must have the `nab` binary installed and accessible.
    *   Download from [Nab Releases](https://github.com/nagisanzenin/nab/releases) or build from source.
    *   Ensure `nab` is in your `$PATH` or set `NAB_BIN=/path/to/nab`.
3.  **Discord Bot**: A Discord Application with "Message Content Intent" enabled.

## ⚙️ Configuration

### Environment Variables

| Variable | Description |
| :--- | :--- |
| `DISCORD_BOT_TOKEN` | Your Discord Bot Token (Required) |
| `GEMINI_API_KEY` | API Key for the LLM Provider |
| `SKYSCLAWPER_USE_NAB` | Set to `1` or `true` to enable Nab for web fetching by default |
| `NAB_BIN` | (Optional) Path to the `nab` binary if not in PATH |

### Config File (`config/default.toml`)

SkyClaw is configured to use **Discord** by default in this fork.

```toml
[channel.discord]
enabled = true
token = "${DISCORD_BOT_TOKEN}"
allowlist = [] # Add your User ID here to restrict access
```

## 🏃 Usage

### 1. Setup Environment
```bash
# Clone the repo
git clone https://github.com/rosenthal/skysclawper.git
cd skysclawper

# Install dependencies (ensure nab is in path)
export NAB_BIN=$(which nab)
```

### 2. Run the Agent
```bash
# Export secrets
export DISCORD_BOT_TOKEN="your_token_here"
export GEMINI_API_KEY="your_key_here"

# Run
cargo run --release
```

### 3. Deployment
Use the deployment script in the `skysclawper-workspace` repository to sync to your VPS (`sns` or `private`):

```bash
# Clone the workspace repo
git clone https://github.com/rosenthal/skysclawper-workspace.git
cd skysclawper-workspace

# Deploy to Private VPS
./scripts/deploy.sh private

# Deploy to SNS VPS
./scripts/deploy.sh sns
```

## 📄 License

This project inherits the **MIT License** from SkyClaw and Nab.
