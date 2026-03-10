# Skysclawper (SkyClaw x Nab)

**The Autonomous Agent that actually sees the private web.**

## 🌟 導入 (Intro)

**SkyClaw** の自己学習型エージェント機能と、**Nab** の超軽量・認証対応ブラウザエンジンを融合。

既存の AI エージェントは、ログインが必要なサイト（GitHub, Slack, Google Workspace など）や Cloudflare 等の高度なボット対策に阻まれ、本来必要な情報にアクセスできないことが多々あります。

**Skysclawper** は、あなたのブラウザセッション（Cookie）を安全に利用する Nab の抽出エンジンを搭載。通常の AI ではアクセスできない「壁の向こう側」から直接情報を取得・学習し、複雑なウェブ調査や操作タスクを完遂します。

## 🚀 主な特徴 (Key Features)

*   **Authenticated Browsing**: Nab エンジンにより、認証が必要なプライベートなページをクリーンな Markdown として直接読み取り。
*   **Anti-Bot Evasion**: HTTP/3, TLS Impersonation 等の独自指紋（Fingerprint）偽装により、AI 拒否設定のあるサイトも回避。
*   **Hyper-Lean Architecture**: 両プロジェクトの Rust ベースの設計を継承。低メモリフットプリントで、安価な VPS 上でも高速に動作。
*   **Conversational Control**: Discord や Telegram を通じて、自然言語でエージェントに指示。レポートの受け取りやファイル操作もスムーズ。

## 🛠 技術スタックの融合 (The Hybrid Tech)

*   **Core**: [SkyClaw](https://github.com/zkfmms/skyclaw) (Rust / Agentic Runtime) - *The "Body"*
*   **Extraction Engine**: [Nab](https://github.com/MikkoParkkola/nab) (HTTP/3, Cookie Injection, Markdown Transformer) - *The "Eyes"*
*   **Memory/Learning**: SkyClaw’s session-persistent memory (SQLite/Vector) - *The "Brain"*

## ⚙️ 設定 (Environment Variables)

認証情報やパスの設定は以下の環境変数を利用します。

| 変数名 | 内容 |
| :--- | :--- |
| `DISCORD_BOT_TOKEN` | Discord Bot のトークン（必須） |
| `GEMINI_API_KEY` | LLM プロバイダー (Google) の API キー |
| `NAB_BIN` | `nab` バイナリのパス（PATH にない場合） |

※ 詳細な設定（Persona, Skills 等）は `skysclawper-workspace` リポジトリで管理します。

## 📦 運用とアップデート

### アップストリームとの同期
本プロジェクトは本家 SkyClaw の更新を適宜取り込めるよう、最小限の変更に留めています。

```bash
git remote add upstream https://github.com/zkfmms/skyclaw.git
git fetch upstream
git merge upstream/main
```

### デプロイ
マルチ VPS 環境へのデプロイは、[skysclawper-workspace](https://github.com/rosenthal/skysclawper-workspace) のスクリプトを使用してください。

## 📄 ライセンス (License)

このプロジェクトは、SkyClaw および Nab の **MIT License** を継承しています。

---
*Developed with respect for the original creators of SkyClaw and Nab.*
