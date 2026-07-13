use crate::server::Server;
use std::fs;
use std::path::PathBuf;

/// 設定ファイルのパスを返す(~/.config/mosh-cli/servers.json)
pub fn config_path() -> PathBuf {
    let base = dirs::config_dir().expect("config directory not found");
    base.join("mosh-cli").join("servers.json")
}
/// JSONファイルからサーバ一覧を読み込む(ファイルがなければ空のリスト)
pub fn load() -> anyhow::Result<Vec<Server>> {
    let path = config_path();
    if !path.exists() {
        return Ok(Vec::new());
    }
    let content = fs::read_to_string(&path)?;
    let servers: Vec<Server> = serde_json::from_str(&content)?;
    Ok(servers)
}
/// サーバ一覧をJSONファイルに書き込む
pub fn save(servers:&[Server]) -> anyhow::Result<()> {
    let path = config_path();
    //親ディレクトリがなければ作成
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let content = serde_json::to_string_pretty(servers)?;
    fs::write(&path, content)?;
    Ok(())
}
/// サーバを追加して保存
pub fn add(server: Server) -> anyhow::Result<()> {
    let mut servers = load()?;
    servers.push(server);
    save(&servers)
}
/// 名前でサーバを削除して保存
pub fn remove(name: &str) -> anyhow::Result<()> {
    let mut servers = load()?;
    servers.retain(|s| s.name != name);
    save(&servers)
}
