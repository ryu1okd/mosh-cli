use crate::server::Server;
use anyhow::Context;
use std::process::Command;


/// 指定されたサーバに mosh で接続する
pub fn connect(server: &Server) -> anyhow::Result<()> {
    let user = server.user.as_deref().unwrap_or("root");
    let addr = format!("{user}@{}", server.hostname);

    // 1. 絶対パスで mosh-server を起動（PATH に依存しない）
    let output = Command::new("ssh")
        .arg(&addr)
        .arg("/opt/homebrew/bin/mosh-server new || /usr/local/bin/mosh-server new || /usr/bin/mosh-server new || mosh-server new")
        .output()?;

    let stderr = String::from_utf8_lossy(&output.stderr);
    if !output.status.success() || stderr.contains("not found") || stderr.contains("command not found") {
        anyhow::bail!(
            "接続先 '{}' で mosh-server の起動に失敗\n\
             確認: ssh {} 'which mosh-server || echo not found'",
            server.hostname,
            &addr,
        );
    }

    // 2. キーとポート番号をパース
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
