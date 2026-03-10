# Skysclawper (SkyClaw x Nab)

**The Autonomous Agent that actually sees the private web.**

This project is a specialized fork and fusion of [SkyClaw](https://github.com/zkfmms/skyclaw) (Autonomous Agent Runtime) and [Nab](https://github.com/MikkoParkkola/nab) (Authenticated Web Extraction). It solves the "auth wall" problem that plagues most AI agents by leveraging a lightweight, browser-based extraction engine capable of using your existing session cookies.

We deeply respect the original work of the SkyClaw and Nab teams. This fork aims to extend their capabilities into a unified, privacy-focused personal assistant runtime.

## 🚀 Key Features

*   **Authenticated Browsing**: Uses `nab` to access pages behind login screens (GitHub, Slack, Google Workspace) by leveraging cookies from your local browser (Brave, Chrome, Safari).
*   **Anti-Bot Evasion**: Employs advanced fingerprinting (HTTP/3, TLS impersonation) to bypass Cloudflare and other bot protections.
*   **Hyper-Lean Architecture**: Built on Rust, designed for low memory footprint and high performance on small VPS instances.
*   **Multi-Persona Runtime**: Designed to run multiple distinct agent personalities (e.g., `private` assistant, `sns` public bot) from the same core binary.

## 🛠 Tech Stack

*   **Core Runtime**: [SkyClaw](https://github.com/zkfmms/skyclaw) (Rust Agentic Runtime) - *The "Body"*
*   **Extraction Engine**: [Nab](https://github.com/MikkoParkkola/nab) (Markdown Transformer & Browser Engine) - *The "Eyes"*
*   **Configuration & Persona**: Managed separately in `skysclawper-workspace` - *The "Mind"*

## 📦 Architecture & Updates

### Repository Split Strategy
To maintain a clean separation of concerns and facilitate upstream updates:

1.  **`skysclawper` (This Repo)**: The **Core Runtime**. It contains the Rust binary, system-level dependencies, and the `nab` integration.
    *   *Update Strategy*: Merge changes from upstream `skyclaw` regularly. Keep custom modifications minimal (mostly in `src/main.rs` for channel integration and `Cargo.toml`).
2.  **`skysclawper-workspace`**: The **Configuration & Identity**. It contains the agent's personality (`persona.md`), skills (Python scripts), and deployment configurations (`envs/`).
    *   *Update Strategy*: This is your personal configuration. It rarely needs upstream merges unless the configuration schema changes significantly.

### Upstream Merge Strategy
When the original [SkyClaw](https://github.com/zkfmms/skyclaw) repository is updated:

```bash
# Add upstream remote
git remote add upstream https://github.com/zkfmms/skyclaw.git

# Fetch and merge
git fetch upstream
git merge upstream/main

# Resolve conflicts (usually in Cargo.toml or main.rs)
# Verify Discord/Nab integration remains intact
```

## ⚙️ Configuration

Configuration is handled via the `skysclawper-workspace` repository.
See `envs/private/config.toml` or `envs/sns/config.toml` for environment-specific settings.

### Key Environment Variables

| Variable | Description |
| :--- | :--- |
| `DISCORD_BOT_TOKEN` | Your Discord Bot Token (Required) |
| `GEMINI_API_KEY` | API Key for the LLM Provider |
| `NAB_BIN` | (Optional) Path to the `nab` binary if not in PATH |

## 🏃 Usage & Deployment

Please refer to the [SkyClaw Workspace](https://github.com/rosenthal/skysclawper-workspace) repository for detailed deployment instructions.

The deployment script (`deploy.sh`) in the workspace repo is designed to:
1.  Construct the agent's identity.
2.  Deploy to a specific VPS target (`ssh host alias`).
3.  Inject secrets securely.

```bash
# Example: Deploy to "sns" environment
./scripts/deploy.sh sns
```

## 📄 License

This project inherits the **MIT License** from SkyClaw and Nab.
