# mosh-cli

SSH config と独自設定ファイルの両方に対応した mosh 接続CLIツール。

## Features

- SSH config の自動読み込み
- 独自サーバ設定の追加・削除
- mosh によるワンコマンド接続
- SSH config からの一括インポート

## Installation

### Download binary

GitHub Releases からバイナリをダウンロード:

```bash
curl -L https://github.com/ryu1okd/mosh-cli/releases/latest/download/mosh-cli-darwin -o /usr/local/bin/mosh-cli
chmod +x /usr/local/bin/mosh-cli
```

### Build from source

```bash
git clone https://github.com/ryu1okd/mosh-cli.git
cd mosh-cli
cargo build --release
cp target/release/mosh-cli /usr/local/bin
```

## Usage

```bash
# サーバ一覧
mosh-cli list

# 接続
mosh-cli connect myserver

# サーバ追加
mosh-cli add myserver 192.168.1.100 --user r1 --port 2222

# サーバ削除
mosh-cli remove myserver

# SSH config からインポート
mosh-cli init
```

## Configuration

設定ファイル: `~/.config/mosh-cli/servers.json`

自動的に `~/.ssh/config` の内容も読み込みます（読み取り専用）。

## License

MIT
