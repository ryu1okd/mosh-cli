use crate::server::Server;
use std::process::Command;


/// 指定されたサーバに mosh で接続する
pub fn connect(server: &Server) -> anyhow::Result<()> {
    let user = server.user.as_deref().unwrap_or("root");
    let addr = format!("{user}@{}", server.hostname);

    let mut cmd = Command::new("mosh");
    cmd.arg(&addr);

    if let Some(port) = server.port {
        cmd.arg("--port").arg(port.to_string());
    }

    let status = cmd.status()?;
    if !status.success() {
        anyhow::bail!(
            "mosh 接続に失敗しました (status: {status})\n\
             接続先で mosh がインストールされているか確認:\n\
             ssh {} {} 'which mosh-server || echo not found'",
            if let Some(u) = &server.user { format!("-l {u}") } else { String::new() },
            server.hostname,
        );
    }
    Ok(())
}
