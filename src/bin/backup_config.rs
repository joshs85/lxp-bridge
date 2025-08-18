use anyhow::Result;
use clap::Parser;
use log::info;
use lxp_bridge::config::ConfigWrapper;
use lxp_bridge::coordinator::commands::backup_config::BackupConfigCommand;

#[derive(Parser)]
#[clap(author, version)]
struct BackupOptions {
    /// Config file to read
    #[clap(short = 'c', long = "config", default_value = "config.yaml")]
    config_file: String,
    
    /// Inverter serial number to backup
    #[clap(short = 'i', long = "inverter")]
    inverter_serial: String,
    
    /// Output file path for backup
    #[clap(short = 'o', long = "output")]
    output_path: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    env_logger::init();
    
    let options = BackupOptions::parse();
    
    info!("Starting LXP Bridge Configuration Backup");
    info!("Config file: {}", options.config_file);
    info!("Inverter serial: {}", options.inverter_serial);
    info!("Output path: {}", options.output_path);
    
    // Load configuration
    let config = ConfigWrapper::new(options.config_file)?;
    
    // Find the specified inverter
    let inverter = config
        .enabled_inverters()
        .into_iter()
        .find(|inv| inv.serial().to_string() == options.inverter_serial)
        .ok_or_else(|| anyhow::anyhow!("Inverter {} not found in configuration", options.inverter_serial))?;
    
    info!("Found inverter: {} ({})", inverter.serial(), inverter.host());
    
    // Create and execute backup command
    let backup_cmd = BackupConfigCommand::new(inverter, options.output_path);
    backup_cmd.execute(&mut lxp_bridge::coordinator::Coordinator::new(config, lxp_bridge::channels::Channels::new())).await?;
    
    info!("Backup completed successfully!");
    Ok(())
}
