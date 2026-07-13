use clap::Parser;

#[derive(Debug, Parser)]
#[command(name = "mosh-cli", version)]
pub enum Cli {
    /// サーバ一覧を表示
    List,
    /// サーバに mosh で接続
    Connect {
        name :String,
    },
    /// サーバを追加
    Add {
        name: String,
        host: String,
        #[arg(long)]
        user: Option<String>,
        #[arg(long)]
        port: Option<u16>,
    },
    /// サーバを削除
    Remove {
        name: String,
    },
    /// SSH config からサーバをインポート
    Init,
}
