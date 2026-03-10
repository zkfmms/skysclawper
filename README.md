# Skysclawper

![Skysclawper v.0.01](assets/banner.png)

> **The Autonomous Agent that actually sees the private web.**
> *v.0.01 - "The Phantom"*

**Skysclawper** は、[SkyClaw](https://github.com/nagisanzenin/skyclaw) をベースに、**VPSでの運用**、**認証サイトの攻略**、そして**徹底的な軽量化**に特化した派生エンジンです。

## ⚡️ ZeroClaw の反省と進化

かつて我々は **ZeroClaw** というエージェントを開発していました。しかし、それは「UNIX思想」に固執しすぎた結果、セキュリティが強固すぎて外部との連携が難しく、実用性に欠けるというジレンマを抱えていました。

Skysclawper はその反省から生まれました。
**「実用的な緩さ」と「圧倒的な突破力」**。これが新しい哲学です。

### なぜ Skysclawper なのか？

本家 SkyClaw との決定的な違いは、ウェブとの関わり方にあります。

| 特徴 | 本家 SkyClaw | 🦞 Skysclawper |
| :--- | :--- | :--- |
| **ブラウザエンジン** | **Chromiumoxide** (Headless Chrome) | **Nab** (Native HTTP/3 Client) |
| **認証** | 手動ログイン / 不安定 | **Cookie Injection** (Brave/Chrome/Safari) |
| **リソース消費** | 重厚 (Chrome必須) | **超軽量** (Rust バイナリのみ) |
| **検知リスク** | ボットとして検知されやすい | **ステルス** (TLS/Fingerprint 偽装) |
| **思想** | 「ブラウザを操作する」 | 「データを抽出する」 |

### 1. Nab エンジンの統合
重たく検知されやすいヘッドレスブラウザの代わりに、[Nab](https://github.com/MikkoParkkola/nab) を統合しました。
*   **Cookie を拝借**: ローカルブラウザの Cookie を安全に利用し、GitHub, Slack, X (Twitter) に「あなた」としてアクセスします。
*   **Cloudflare 突破**: リアルな TLS 指紋と HTTP/3 により、ボット対策を回避します。
*   **Markdown 直読**: ノイズの多い HTML ではなく、整形された Markdown を直接読み込みます。

### 2. VPS デプロイ特化 (Ubuntu 22.04 LTS)
Skysclawper は、安価な VPS (Ubuntu 22.04 等) での稼働を前提に設計されています。
*   **シングルバイナリ**: 複雑な依存関係（X11, Chrome等）は不要です。
*   **ワークスペース分離**: 設定や人格（Persona）をランタイムから切り離し、`skysclawper-workspace` で管理することで、1つのバイナリで複数のエージェント（Private用, SNS用など）を使い分けることができます。

---

## 🚀 主な機能

*   **Authenticated Browsing**: 通常のスクレイパーではアクセスできないプライベートリポジトリや社内ドキュメントを閲覧可能。
*   **Conversational Control**: Discord (Telegram) とネイティブ統合。チャットで調査を依頼し、結果を受け取る自然な対話。
*   **Session Persistence**: SQLite と Vector メモリにより、再起動しても文脈を失いません。
*   **Self-Healing**: サブプロセスの管理と自己修復機能を搭載。

---

## 🛠 技術スタック

*   **Language**: Rust 🦀
*   **Core**: SkyClaw Agentic Runtime
*   **Network**: Nab (Hyper/Reqwest/Quinn)
*   **Memory**: SQLite + Vector Embeddings

---

## ⚙️ クイックスタート

### 1. インストール

```bash
# ランタイムのクローン
git clone https://github.com/rosenthal/skysclawper.git
cd skysclawper

# ビルド (Release モード推奨)
cargo build --release
```

### 2. 設定

環境変数を設定します。本格的な運用には **Workspace** の利用を推奨しますが、手軽なテストなら以下で十分です。

```bash
export DISCORD_BOT_TOKEN="your_token"
export GEMINI_API_KEY="your_key"
export NAB_BIN="/path/to/nab"  # PATHに含まれていない場合
```

### 3. 実行

```bash
./target/release/skyclaw
```

---

## 📦 デプロイと更新

### 本家との同期
本家 SkyClaw の急速な進化を尊重し、追従します。

```bash
git remote add upstream https://github.com/nagisanzenin/skyclaw.git
git fetch upstream
git merge upstream/main
# src/main.rs (Browser vs Nab) の競合解決が必要になる場合があります
```

### VPS へのデプロイ
[skysclawper-workspace](https://github.com/rosenthal/skysclawper-workspace) のデプロイスクリプトを使用してください。

---

## 📄 ライセンス

SkyClaw および Nab の **MIT License** を継承します。

> *Header art by User. Concept based on SkyClaw by nagisanzenin & Nab by MikkoParkkola.*
