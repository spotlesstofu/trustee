use log::debug;

mod cli;
mod keys_certs;

#[tokio::main]
async fn main() {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    if let Err(e) = cli::cli_default().await {
        eprintln!("Error: {:#}", e);
        std::process::exit(1);
    }
}
