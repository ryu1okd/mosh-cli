use crate::server::Server;
use anyhow::Context;
use std::process::Command;


/// 指定されたサーバに mosh で接続する
pub fn connect(server: &Server) -> anyhow::Result<()> {
    let user = server.user.as_deref().unwrap_or("root");
    let addr = format!("{user}@{}", server.hostname);

    // 1. SSH で接続し mosh-server の有無を確認
    let check = Command::new("ssh")
        .arg(&addr)
        .arg("which mosh-server >/dev/null 2>&1")
        .status()?;
    if !check.success() {
        anyhow::bail!(
            "接続先 '{}' に mosh-server が見つかりません",
            server.hostname,
        );
    }

    // 2. mosh-server を起動
    let output = Command::new("ssh")
        .arg(&addr)
        .arg("mosh-server new")
        .output()?;

    // 3. キーとポート番号をパース
    let stdout = String::from_utf8_lossy(&output.stdout);
    let first_line = stdout.lines().next().context("mosh-server の出力が空です")?;
    let parts: Vec<&str> = first_line.splitn(4, char::is_whitespace).collect();
    if parts.len() < 4 || parts[0] != "MOSH" || parts[1] != "CONNECT" {
        anyhow::bail!("mosh-server の出力パースに失敗:\n{}", first_line);
    }
    let port = parts[2];
    let key = parts[3];

    // 4. SSH 切断（スコープを抜ければ自動終了）
    // 5. mosh 接続
    let mut cmd = Command::new("mosh");
    cmd.env("MOSH_KEY", key)
        .arg("--no-init")
        .arg(&addr)
        .arg(port);

    if let Some(p) = server.port {
        cmd.arg("--port").arg(p.to_string());
    }

    let status = cmd.status()?;
    if !status.success() {
        anyhow::bail!("mosh 接続に失敗しました (status: {status})");
    }
    Ok(())
}
