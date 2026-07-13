use crate::server::Server;
use std::fs;

/// SSH config ファイルを読み込み、Serverのリストを返す
pub fn parse(path: &str) -> anyhow::Result<Vec<Server>> {
    let content = fs::read_to_string(path)?;
    let mut servers = Vec::new();
    let mut current: Option<Server> = None;

    for line in content.lines() {
        let line = line.trim();

        // skip empty line or comment line 
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        // Hot ディレクティブ→新しいサーバブロックの開始
        if let Some(host_name) = parse_host_directive(line) {
            // 前のサーバがあれば確定して追加
            if let Some(server) = current.take() {
                servers.push(server);
            }
            // ワイルドカード(* や ?) を含むホストはスキップ
            if !host_name.contains('*') && !host_name.contains('?') {
                current = Some(Server {
                    name: host_name,
                    hostname: String::new(),
                    user: None,
                    port: None,
                });
            }
            continue;
        }
        // 現在アクティブなサーバブロックがあれば、設定値を更新
        if let Some(ref mut server) = current {
            apply_directive(server, line);
        }
    }
    // 最後のサーバを追加
    if let Some(server) = current {
        servers.push(server);
    }

    Ok(servers)

}

/// Host xxx の行から xxx 部分を抽出。 Host で始まらなければNone
fn parse_host_directive(line: &str) -> Option<String> {
    let lower = line.to_lowercase();
    if lower.starts_with("host ") || lower.starts_with("host\t") {
        let name = line[4..].trim().to_string();
        if !name.is_empty() {
            return Some(name);
        }
    }
    None
}

/// ここの設定行(Host, User, Port など) を Server に反映
fn apply_directive(server: &mut Server, line: &str) {
    // 空白で分割(例: "HostName 192.168.1.100" → ["HostName", "192.168.1.100"])
    let parts: Vec<&str> = line.splitn(2, char::is_whitespace).collect();
    if parts.len() < 2 {
        return;
    }

    let key = parts[0];
    let value = parts[1].trim();

    match key.to_lowercase().as_str() {
        "hostname" => server.hostname = value.to_string(),
        "user" => server.user = Some(value.to_string()),
        "port" => {
            if let Ok(p) = value.parse::<u16>() {
                server.port = Some(p);
            }
        }
        _ => {} // その他のディレクティブは無視
    }
}
