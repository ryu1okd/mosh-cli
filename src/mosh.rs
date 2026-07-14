use crate::server::Server;
use std::process::Command;


/// 指定されたサーバに mosh で接続する
pub fn connect(server: &Server) -> anyhow::Result<()> {
    let user = server.user.as_deref().unwrap_or("root");
    let addr = format!("{user}@{}", server.hostname);

    let server_cmd = "/opt/homebrew/bin/mosh-server || /usr/local/bin/mosh-server || /usr/bin/mosh-server || mosh-server";

    let status = Command::new("mosh")
        .arg("--server")
        .arg(server_cmd)
        .arg(&addr)
        .status()?;

    if !status.success() {
        anyhow::bail!("mosh 接続に失敗しました (status: {status})");
    }
    Ok(())
}
