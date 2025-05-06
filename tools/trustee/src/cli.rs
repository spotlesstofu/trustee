use std::path::{Path, PathBuf};

use anyhow::Result;
use clap::{error::Error, Parser};
use dirs::home_dir;
use log::info;

use crate::keys_certs::new_auth_key_pair;
use kbs::{ApiServer, KbsConfig};

fn trustee_keygen(trustee_home_dir: &Path) -> Result<(PathBuf, PathBuf)> {
    let (private, public) = new_auth_key_pair(trustee_home_dir)?;
    info!("Wrote new private key: {:?}", private);
    info!("Wrote new public key: {:?}", public);
    Ok((private, public))
}

async fn trustee_run(config_file: Option<PathBuf>, trustee_home_dir: &Path) -> Result<()> {
    let mut config = if let Some(path) = config_file {
        KbsConfig::try_from(path.as_path())?
    } else {
        KbsConfig::default()
    };

    if config.admin.auth_public_key.is_none() {
        let key_base_path = trustee_home_dir.join("auth_key");
        let private_path = key_base_path.with_extension("pem");

        let public_path = {
            if !private_path.exists() {
                trustee_keygen(&key_base_path)?.1
            } else {
                key_base_path.with_extension("pub")
            }
        };
        config.admin.auth_public_key = Some(public_path);
    }

    let api_server = ApiServer::new(config).await?;
    api_server.server()?.await?;
    Ok(())
}

#[derive(Debug, Parser)]
enum Commands {
    /// Generate a new key pair (Ed25519)
    Keygen {
        /// Output file for the private key
        #[arg(short = 'f')]
        output_file: Option<PathBuf>,
    },
    /// Launch Trustee. Uses a self-signed HTTPS certificate with a RSA 2048 key by default
    Run {
        /// Configuration file
        #[arg(long)]
        config_file: Option<PathBuf>,
    },
}

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
    /// Path to the directory for storing config and data
    #[arg(long, value_name = "PATH", env = "TRUSTEE_HOME")]
    home: Option<PathBuf>,
}

pub async fn cli_default() -> Result<(), Error> {
    let cli = Cli::try_parse()?;

    let trustee_home_dir = cli.home.unwrap_or_else(|| {
        if Uid::effective().is_root() {
            "/opt/confidential-containers".into()
        } else {
            home_dir().unwrap_or_default().join(".trustee")
        }
    });

    if !trustee_home_dir.exists() {
        std::fs::create_dir_all(&trustee_home_dir)?;
    }

    match cli.command {
        Commands::Keygen { output_file: out } => {
            let out = out.unwrap_or_else(|| trustee_home_dir.join("key"));
            trustee_keygen(&out).unwrap();
        }
        Commands::Run { config_file } => {
            trustee_run(config_file, &trustee_home_dir).await.unwrap();
        }
    };

    Ok(())
}
