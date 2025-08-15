use crate::prelude::*;

use serde::Deserialize;
use serde_with::serde_as; //, OneOrMany;

#[serde_as]
#[derive(Clone, Debug, Deserialize)]
pub struct Config {
    pub inverters: Vec<Inverter>,
    //#[serde_as(deserialize_as = "OneOrMany<_>")]
    pub mqtt: Mqtt,
    pub scheduler: Option<Scheduler>,

    #[serde(default = "Config::default_loglevel")]
    pub loglevel: String,
}

// Inverter {{{
#[derive(Clone, Debug, Deserialize)]
pub struct Inverter {
    #[serde(default = "Config::default_enabled")]
    pub enabled: bool,

    pub host: String,
    pub port: u16,
    #[serde(deserialize_with = "de_serial")]
    pub serial: Serial,
    #[serde(deserialize_with = "de_serial")]
    pub datalog: Serial,

    pub heartbeats: Option<bool>,
    pub publish_holdings_on_connect: Option<bool>,
    pub read_timeout: Option<u64>,
}
impl Inverter {
    pub fn enabled(&self) -> bool {
        self.enabled
    }

    pub fn host(&self) -> &str {
        &self.host
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub fn serial(&self) -> Serial {
        self.serial
    }

    pub fn datalog(&self) -> Serial {
        self.datalog
    }

    pub fn heartbeats(&self) -> bool {
        self.heartbeats == Some(true)
    }

    pub fn publish_holdings_on_connect(&self) -> bool {
        self.publish_holdings_on_connect == Some(true)
    }

    pub fn read_timeout(&self) -> u64 {
        self.read_timeout.unwrap_or(900) // 15 minutes
    }
} // }}}



// Mqtt {{{
#[derive(Clone, Debug, Deserialize)]
pub struct Mqtt {
    #[serde(default = "Config::default_enabled")]
    pub enabled: bool,

    #[serde(default = "Config::default_mqtt_host")]
    pub host: String,

    #[serde(default = "Config::default_mqtt_port")]
    pub port: u16,
    pub username: Option<String>,
    pub password: Option<String>,

    #[serde(default = "Config::default_mqtt_namespace")]
    pub namespace: String,

    #[serde(default = "Config::default_mqtt_homeassistant_enabled")]
    pub homeassistant_enabled: bool,

    #[serde(default = "Config::default_mqtt_homeassistant_prefix")]
    pub homeassistant_prefix: String,

    pub publish_individual_input: Option<bool>,

    // Reliability settings
    #[serde(default = "Config::default_mqtt_max_retries")]
    pub max_retries: u32,
    
    #[serde(default = "Config::default_mqtt_circuit_breaker_threshold")]
    pub circuit_breaker_threshold: u32,
    
    #[serde(default = "Config::default_mqtt_reconnect_delay")]
    pub reconnect_delay_secs: u64,
    
    #[serde(default = "Config::default_mqtt_max_reconnect_delay")]
    pub max_reconnect_delay_secs: u64,
}
impl Mqtt {
    pub fn enabled(&self) -> bool {
        self.enabled
    }

    pub fn host(&self) -> &str {
        &self.host
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub fn username(&self) -> &Option<String> {
        &self.username
    }

    pub fn password(&self) -> &Option<String> {
        &self.password
    }

    pub fn namespace(&self) -> &str {
        &self.namespace
    }

    pub fn homeassistant_enabled(&self) -> bool {
        self.homeassistant_enabled
    }

    pub fn homeassistant_prefix(&self) -> &str {
        &self.homeassistant_prefix
    }

    pub fn publish_individual_input(&self) -> bool {
        self.publish_individual_input == Some(true)
    }

    pub fn max_retries(&self) -> u32 {
        self.max_retries
    }

    pub fn circuit_breaker_threshold(&self) -> u32 {
        self.circuit_breaker_threshold
    }

    pub fn reconnect_delay_secs(&self) -> u64 {
        self.reconnect_delay_secs
    }

    pub fn max_reconnect_delay_secs(&self) -> u64 {
        self.max_reconnect_delay_secs
    }
} // }}}



// Scheduler {{{
#[derive(Clone, Debug, Deserialize)]
pub struct Scheduler {
    #[serde(default = "Config::default_enabled")]
    pub enabled: bool,

    #[serde(default = "Config::default_timesync_cron")]
    pub timesync_cron: Option<String>,
}
impl Scheduler {
    pub fn enabled(&self) -> bool {
        self.enabled
    }

    pub fn timesync_cron(&self) -> &Option<String> {
        &self.timesync_cron
    }
} // }}}

#[derive(Debug)]
pub struct ConfigWrapper {
    config: Rc<RefCell<Config>>,
}

impl Clone for ConfigWrapper {
    fn clone(&self) -> Self {
        Self {
            config: Rc::clone(&self.config),
        }
    }
}

impl ConfigWrapper {
    pub fn new(file: String) -> Result<Self> {
        let config = Rc::new(RefCell::new(Config::new(file)?));

        Ok(Self { config })
    }

    pub fn new_from_config(config: Config) -> Self {
        Self {
            config: Rc::new(RefCell::new(config)),
        }
    }

    pub fn inverters(&self) -> Ref<'_, Vec<Inverter>> {
        Ref::map(self.config.borrow(), |b| &b.inverters)
    }

    pub fn set_inverters(&self, new: Vec<Inverter>) {
        let mut c = self.config.borrow_mut();
        c.inverters = new;
    }

    pub fn enabled_inverters(&self) -> Vec<Inverter> {
        self.inverters()
            .iter()
            .filter(|inverter| inverter.enabled)
            .cloned()
            .collect()
    }

    pub fn inverter_with_host(&self, host: &str) -> Option<Inverter> {
        self.inverters()
            .iter()
            .find(|inverter| inverter.host == host)
            .cloned()
    }

    pub fn enabled_inverter_with_datalog(&self, datalog: Serial) -> Option<Inverter> {
        self.enabled_inverters()
            .iter()
            .find(|inverter| inverter.datalog == datalog)
            .cloned()
    }

    pub fn inverters_for_message(&self, message: &mqtt::Message) -> Result<Vec<Inverter>> {
        use mqtt::TargetInverter::*;

        let (target_inverter, _) = message.split_cmd_topic()?;
        let inverters = self.enabled_inverters();

        let r = match target_inverter {
            All => inverters,
            Serial(datalog) => inverters
                .iter()
                .filter(|i| i.datalog == datalog)
                .cloned()
                .collect(),
        };

        Ok(r)
    }

    pub fn mqtt(&self) -> Ref<'_, Mqtt> {
        Ref::map(self.config.borrow(), |b| &b.mqtt)
    }



    pub fn scheduler(&self) -> Ref<'_, Option<Scheduler>> {
        Ref::map(self.config.borrow(), |b| &b.scheduler)
    }

    pub fn loglevel(&self) -> String {
        self.config.borrow().loglevel.to_owned()
    }
}

impl Config {
    pub fn new(file: String) -> Result<Self> {
        let content = std::fs::read_to_string(&file)
            .map_err(|err| anyhow!("error reading {}: {}", file, err))?;

        let mut config: Config = serde_yaml::from_str(&content)?;
        
        // Set default scheduler if none provided
        if config.scheduler.is_none() {
            config.scheduler = Some(Scheduler {
                enabled: true,
                timesync_cron: Some("0 0 * * *".to_string()),
            });
        }

        Ok(config)
    }

    fn default_mqtt_host() -> String {
        "localhost".to_string()
    }

    fn default_mqtt_port() -> u16 {
        1883
    }
    fn default_mqtt_namespace() -> String {
        "lxp".to_string()
    }

    fn default_mqtt_homeassistant_enabled() -> bool {
        true
    }

    fn default_mqtt_homeassistant_prefix() -> String {
        "homeassistant".to_string()
    }

    fn default_mqtt_max_retries() -> u32 {
        3
    }

    fn default_mqtt_circuit_breaker_threshold() -> u32 {
        5
    }

    fn default_mqtt_reconnect_delay() -> u64 {
        1
    }

    fn default_mqtt_max_reconnect_delay() -> u64 {
        300
    }

    fn default_enabled() -> bool {
        true
    }

    fn default_loglevel() -> String {
        "debug".to_string()
    }

    fn default_timesync_cron() -> Option<String> {
        Some("0 0 * * *".to_string())
    }
}

fn de_serial<'de, D>(deserializer: D) -> Result<Serial, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let raw = String::deserialize(deserializer)?;
    raw.parse().map_err(serde::de::Error::custom)
}

