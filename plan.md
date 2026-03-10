# Plan for Migrating to Skysclawper (SkyClaw x Nab)

This plan addresses the migration from `zeroclaw-workspace` to `skysclawper`, incorporating the `nab` integration, Discord configuration, and environment management scripts.

## 1. Documentation (Task A)
- **Action**: Update `skysclawper/README.md`.
- **Content**:
    - Title: **SkyClaw x Nab (Skysclawper)**
    - Subtitle: *The Autonomous Agent that actually sees the private web.*
    - Features: Authenticated Browsing (via `nab`), Anti-Bot Evasion, Hyper-Lean Architecture.
    - Setup: Mention `nab` binary requirement and environment variables (`SKYSCLAWPER_USE_NAB`).
    - Tech Stack: SkyClaw (Core) + Nab (Extraction).

## 2. Configuration & Dependencies (Task C)
- **Action**: Modify `skysclawper/Cargo.toml`.
    - Change default features from `["telegram", "browser"]` to `["discord", "browser"]` to ensure Discord support is built-in by default.
- **Action**: Modify `skysclawper/config/default.toml`.
    - Disable `[channel.telegram]`.
    - Enable `[channel.discord]` with `${DISCORD_BOT_TOKEN}`.

## 3. Environment & Deployment Scripts (Task B & D)
- **Action**: Create `skysclawper/scripts/` directory.
- **Action**: Port and Adapt `zcredeploy.sh`.
    - Rename/Update to `deploy.sh` (or keep `zcredeploy.sh` if preferred).
    - Update paths to match `skysclawper` structure.
    - Ensure it deploys the `skyclaw` binary (or restarts the service).
    - Maintain the `sns` vs `private` argument logic for environment separation.
- **Action**: Port `Identity_Constructor.py`.
    - Ensure it works with the new directory structure.
- **Action**: Create Environment Directory Structure.
    - Create `envs/private/workspace` and `envs/sns/workspace`.
    - Create `instructions_library/skills`.

## 4. Skills Migration (Task D)
- **Action**: Copy Skills from `zeroclaw-workspace`.
    - Copy `x-intel`, `x-post`, `searxng-search-api` to `skysclawper/instructions_library/skills/`.
    - Ensure `SKILL.md` and Python scripts are preserved.

## 5. Verification
- **Action**: Verify file placement and configuration syntax.
- **Note**: Actual deployment requires the VPS and secrets, which are out of scope for this IDE session, but the scripts will be ready.

## Execution Order
1.  Create directories.
2.  Update `Cargo.toml` and `config/default.toml`.
3.  Port scripts (`zcredeploy.sh`, `Identity_Constructor.py`).
4.  Copy skills.
5.  Update `README.md`.
