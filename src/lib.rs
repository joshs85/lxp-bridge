pub mod channels;
pub mod command;
pub mod config;
pub mod coordinator;
pub mod home_assistant;
pub mod lxp;
pub mod mqtt;
pub mod options;
pub mod prelude;
pub mod scheduler;
pub mod unixtime;
pub mod utils;

pub const CARGO_PKG_VERSION: &str = env!("CARGO_PKG_VERSION");

use crate::prelude::*;

pub async fn app() -> Result<()> {
    let options = Options::new();

    let config = ConfigWrapper::new(options.config_file).unwrap_or_else(|err| {
        // no logging available yet, so eprintln! will have to do
        eprintln!("Error: {err:?}");
        std::process::exit(255);
    });

    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or(config.loglevel()))
        .format(|buf, record| {
            writeln!(
                buf,
                "[{} {} {}] {}",
                chrono::Local::now().format("%Y-%m-%dT%H:%M:%S%.3f"),
                record.level(),
                record.module_path().unwrap_or(""),
                record.args()
            )
        })
        .write_style(env_logger::WriteStyle::Never)
        .init();

    info!("lxp-bridge {} starting", CARGO_PKG_VERSION);

    let channels = Channels::new();

    let scheduler = Scheduler::new(config.clone(), channels.clone());
    let mqtt = Mqtt::new(config.clone(), channels.clone());
    let coordinator = Coordinator::new(config.clone(), channels.clone());

    let inverters = config
        .enabled_inverters()
        .into_iter()
        .map(|inverter| Inverter::new(config.clone(), &inverter, channels.clone()))
        .collect();

    futures::try_join!(
        start_inverters(inverters),
        scheduler.start(),
        mqtt.start(),
        coordinator.start(),
        health_monitor(config.clone(), channels.clone())
    )?;

    Ok(())
}



async fn start_inverters(inverters: Vec<Inverter>) -> Result<()> {
    let futures = inverters.iter().map(|i| i.start());

    futures::future::join_all(futures).await;

    Ok(())
}

async fn health_monitor(config: ConfigWrapper, channels: Channels) -> Result<()> {
    let mut interval = tokio::time::interval(std::time::Duration::from_secs(60)); // Check every minute
    
    loop {
        interval.tick().await;
        
        // Check channel health
        channels.check_channel_health();
        
        // Log system status
        info!("Health check: {} enabled inverters, MQTT: {}",
            config.enabled_inverters().len(),
            if config.mqtt().enabled() { "enabled" } else { "disabled" }
        );
    }
}
