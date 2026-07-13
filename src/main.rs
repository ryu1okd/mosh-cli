mod server;
mod ssh_config;
mod config;
mod registry;
mod cli;
mod mosh;

use clap::Parser;
use crate::server::Server;

fn main() -> anyhow::Result<()> {
    let cli = cli::Cli::parse();

    match cli {
        cli::Cli::List => {
            let entries = registry::all()?;
            for entry in &entries {
                let marker = match entry.source {
                    registry::Source::SshConfig => "[SSH]",
                    registry::Source::Custom => "[CLI]",
                };
                println!("  {marker} {} → {}", entry.server.name, entry.server.hostname);
            }
        }
        cli::Cli::Connect { name } => {
            let entries = registry::all()?;
            let entry = entries.iter()
                .find(|e| e.server.name == name)
                .ok_or_else(|| anyhow::anyhow!("サーバ '{name}' が見つかりません"))?;
            mosh::connect(&entry.server)?;
        }
        cli::Cli::Add { name, host, user, port } => {
            let _ = config::add(Server{ name, hostname: host, user, port});
            println!("追加しました");
        }
        cli::Cli::Remove { name } => {
            config::remove(&name)?;
            println!("削除しました");
        }
        cli::Cli::Init => {
            let ssh_path = dirs::home_dir()
                .unwrap()
                .join(".ssh")
                .join("config");
            let servers = ssh_config::parse(ssh_path.to_str().unwrap())?;
            for s in &servers {
                config::add(s.clone())?;
            }
            println!("{} 件のサーバをインポートしました", servers.len());
        }
    }
    Ok(())
}
