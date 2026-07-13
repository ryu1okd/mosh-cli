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

    // mosh は子プロセスとして実行され、終了するまで待つ
    let status = cmd.status()?;
    if !status.success() {
        anyhow::bail!("mosh が異常終了しました: {status}");
    }
    Ok(())
}
