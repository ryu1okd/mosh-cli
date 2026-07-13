use crate::server::Server;
use crate::{config, ssh_config};

/// サーバの出所を表す
#[derive(Debug, Clone, PartialEq)]
pub enum Source {
    SshConfig,
    Custom,
}
/// 出所付きサーバ情報
#[derive(Debug, Clone)]
pub struct ServerEntry {
    pub server: Server,
    pub source: Source,
}

/// 全サーバ一覧を取得(SSH config + 独自設定)
pub fn all() -> anyhow::Result<Vec<ServerEntry>> {
    let mut entries = Vec::new();

    // SSH config から
    let ssh_path = dirs::home_dir()
        .unwrap()
        .join(".ssh")
        .join("config");
    if ssh_path.exists() {
        if let Ok(servers) = ssh_config::parse(ssh_path.to_str().unwrap()) {
            for s in servers {
                entries.push(ServerEntry {
                    server: s,
                    source: Source::SshConfig,
                });
            }
        }
    }

    // 独自設定から
    let custom_servers = config::load()?;
    for s in custom_servers {
        // SSH config と同名のサーバは独自設定で上書き
        if !entries.iter().any(|e| e.server.name == s.name && e.source == Source::Custom) {
            entries.push(ServerEntry {
                server: s,
                source: Source::Custom,
            });
        }
    }
    Ok(entries)
}

