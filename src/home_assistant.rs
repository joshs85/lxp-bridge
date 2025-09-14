//! Home Assistant MQTT Discovery Integration
//! 
//! This module provides MQTT discovery for Home Assistant, automatically creating
//! entities for all inverter registers and controls.
//! 
//! ## Entity Categories
//! 
//! Home Assistant uses `entity_category` to organize entities into different sections:
//! 
//! - **`entity_category: Some("config".to_string())`** → Entities appear in the "Configuration" section
//!   - Used for advanced settings, safety controls, and system configuration
//!   - Examples: Grid protection limits, reactive power settings, frequency controls
//! 
//! - **`entity_category: Some("diagnostic".to_string())`** → Entities appear in the "Diagnostic" section  
//!   - Used for system maintenance, troubleshooting, and administrative functions
//!   - Examples: Restart button, system reset functions, diagnostic tools
//! 
//! - **`entity_category: None` or omitted** → Entities appear in the primary controls section
//!   - Used for everyday operational controls and frequently used settings
//!   - Examples: AC charge controls, time slots, basic power management
//! 
//! ## MQTT Discovery
//! 
//! All entities are automatically discovered by Home Assistant via MQTT discovery.
//! The discovery payload includes all necessary configuration for proper entity display
//! and functionality within Home Assistant.

use crate::prelude::*;
use lxp::packet::Register;

use serde::{Serialize, Serializer};

// ValueTemplate {{{
#[derive(Clone, Debug, PartialEq)]
pub enum ValueTemplate {
    None,
    Default, // "{{ value_json.$key }}"
    String(String),
}
impl ValueTemplate {
    pub fn from_default(key: &str) -> Self {
        Self::String(format!("{{{{ value_json.{key} }}}}"))
    }
    pub fn is_none(&self) -> bool {
        *self == Self::None
    }
    pub fn is_default(&self) -> bool {
        *self == Self::Default
    }
}
impl Serialize for ValueTemplate {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            ValueTemplate::String(str) => serializer.serialize_str(str),
            _ => unreachable!(),
        }
    }
} // }}}

#[derive(Clone, Debug, Serialize)]
pub struct Availability {
    topic: String,
}

#[derive(Clone, Debug, Serialize)]
pub struct Device {
    manufacturer: String,
    name: String,
    identifiers: [String; 1],
    // model: String, // TODO: provide inverter model
}

pub struct Config {
    inverter: config::Inverter,
    mqtt_config: config::Mqtt,
}

// https://www.home-assistant.io/integrations/sensor.mqtt/
#[derive(Clone, Debug, Serialize)]
pub struct Entity<'a> {
    // this is not serialised into the JSON output, just used as a transient store to
    // work out what unique_id and topic should be
    #[serde(skip)]
    key: &'a str, // for example, soc

    unique_id: &'a str, // {namespace}_{datalog}_{name}
    name: &'a str,      // really more of a label? for example, "State of Charge"

    state_topic: &'a str,

    // these are all skipped in the output if None. this lets us use the same struct for
    // different types of entities, just our responsibility to make sure a sane set of attributes
    // are populated. Could make subtypes to enforce the various attributes being set for different
    // HA entity types but I think its not worth the extra complexity.
    #[serde(skip_serializing_if = "Option::is_none")]
    entity_category: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    state_class: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    device_class: Option<&'a str>,
    #[serde(skip_serializing_if = "ValueTemplate::is_none")]
    value_template: ValueTemplate,
    #[serde(skip_serializing_if = "Option::is_none")]
    unit_of_measurement: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    suggested_display_precision: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    icon: Option<&'a str>,

    device: Device,
    availability: Availability,
}

// https://www.home-assistant.io/integrations/switch.mqtt/
#[derive(Debug, Serialize)]
pub struct Switch {
    name: String,
    state_topic: String,
    command_topic: String,
    value_template: String,
    unique_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    entity_category: Option<String>,
    device: Device,
    availability: Availability,
}

// https://www.home-assistant.io/integrations/button.mqtt/
#[derive(Debug, Serialize)]
pub struct Button {
    name: String,
    command_topic: String,
    unique_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    entity_category: Option<String>,
    device: Device,
    availability: Availability,
}

// https://www.home-assistant.io/integrations/number.mqtt/
#[derive(Debug, Serialize)]
pub struct Number {
    name: String,
    state_topic: String,
    command_topic: String,
    value_template: String,
    unique_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    entity_category: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled_by_default: Option<bool>, // Controls if entity is enabled by default. None = skip this field (use Home Assistant default), Some(true) = enabled, Some(false) = disabled
    device: Device,
    availability: Availability,
    min: f64,
    max: f64,
    step: f64,
    unit_of_measurement: String,
}

// https://www.home-assistant.io/integrations/text.mqtt/
#[derive(Debug, Serialize)]
pub struct Text {
    name: String,
    state_topic: String,
    command_topic: String,
    command_template: String,
    value_template: String,
    unique_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    entity_category: Option<String>,
    device: Device,
    availability: Availability,
    pattern: String,
}

impl Config {
    pub fn new(inverter: &config::Inverter, mqtt_config: &config::Mqtt) -> Self {
        Self {
            inverter: inverter.clone(),
            mqtt_config: mqtt_config.clone(),
        }
    }

    pub fn sensors(&self) -> Vec<mqtt::Message> {
        let base = Entity {
            key: &String::default(),
            unique_id: &String::default(),
            name: &String::default(),
            entity_category: None,
            device_class: None,
            state_class: None,
            unit_of_measurement: None,
            suggested_display_precision: None,
            icon: None,
            value_template: ValueTemplate::Default, // "{{ value_json.$key }}"
            // TODO: might change this to an enum that defaults to InputsAll but can be replaced
            // with a string for a specific topic?
            state_topic: &format!(
                "{}/{}/inputs/all",
                self.mqtt_config.namespace(),
                self.inverter.datalog()
            ),
            device: self.device(),
            availability: self.availability(),
        };

        let voltage = Entity {
            device_class: Some("voltage"),
            state_class: Some("measurement"),
            unit_of_measurement: Some("V"),
            ..base.clone()
        };

        let frequency = Entity {
            device_class: Some("frequency"),
            state_class: Some("measurement"),
            unit_of_measurement: Some("Hz"),
            suggested_display_precision: Some(2),
            ..base.clone()
        };

        let power = Entity {
            device_class: Some("power"),
            state_class: Some("measurement"),
            unit_of_measurement: Some("W"),
            ..base.clone()
        };

        let current = Entity {
            device_class: Some("current"),
            state_class: Some("measurement"),
            unit_of_measurement: Some("A"),
            ..base.clone()
        };

        let energy = Entity {
            device_class: Some("energy"),
            state_class: Some("total_increasing"),
            unit_of_measurement: Some("kWh"),
            ..base.clone()
        };

        let energy_storage = Entity {
            device_class: Some("energy_storage"),
            state_class: Some("measurement"),
            unit_of_measurement: Some("kWh"),
            ..base.clone()
        };

        let temperature = Entity {
            device_class: Some("temperature"),
            state_class: Some("measurement"),
            unit_of_measurement: Some("°C"),
            ..base.clone()
        };

        // now each entry in here should only have to specify specific overrides for each key.
        // if we have multiple things sharing keys, consider whether to make a new variable to
        // inherit from.
        let sensors = [
            Entity {
                key: "status",
                name: "Status",
                state_topic: &format!(
                    "{}/{}/input/0/parsed",
                    self.mqtt_config.namespace(),
                    self.inverter.datalog()
                ),
                value_template: ValueTemplate::None,
                ..base.clone()
            },
            Entity {
                key: "soc",
                name: "State of Charge",
                device_class: Some("battery"),
                state_class: Some("measurement"),
                unit_of_measurement: Some("%"),
                ..base.clone()
            },
            Entity {
                key: "fault_code",
                name: "Fault Code",
                entity_category: Some("diagnostic"),
                state_topic: &format!(
                    "{}/{}/input/fault_code/parsed",
                    self.mqtt_config.namespace(),
                    self.inverter.datalog()
                ),
                value_template: ValueTemplate::None,
                icon: Some("mdi:alert"),
                ..base.clone()
            },
            Entity {
                key: "warning_code",
                name: "Warning Code",
                entity_category: Some("diagnostic"),
                state_topic: &format!(
                    "{}/{}/input/warning_code/parsed",
                    self.mqtt_config.namespace(),
                    self.inverter.datalog()
                ),
                value_template: ValueTemplate::None,
                icon: Some("mdi:alert-outline"),
                ..base.clone()
            },
            Entity {
                key: "all_faults",
                name: "All Active Faults",
                entity_category: Some("diagnostic"),
                state_topic: &format!(
                    "{}/{}/input/fault_code/all",
                    self.mqtt_config.namespace(),
                    self.inverter.datalog()
                ),
                value_template: ValueTemplate::String("{{ value | join(', ') }}".to_string()),
                icon: Some("mdi:alert-circle"),
                ..base.clone()
            },
            Entity {
                key: "all_warnings",
                name: "All Active Warnings",
                entity_category: Some("diagnostic"),
                state_topic: &format!(
                    "{}/{}/input/warning_code/all",
                    self.mqtt_config.namespace(),
                    self.inverter.datalog()
                ),
                value_template: ValueTemplate::String("{{ value | join(', ') }}".to_string()),
                icon: Some("mdi:alert-outline"),
                ..base.clone()
            },
            Entity {
                key: "faults_json",
                name: "Faults (Machine Readable)",
                entity_category: Some("diagnostic"),
                state_topic: &format!(
                    "{}/{}/input/fault_code/json",
                    self.mqtt_config.namespace(),
                    self.inverter.datalog()
                ),
                value_template: ValueTemplate::None,
                icon: Some("mdi:code-json"),
                ..base.clone()
            },
            Entity {
                key: "warnings_json",
                name: "Warnings (Machine Readable)",
                entity_category: Some("diagnostic"),
                state_topic: &format!(
                    "{}/{}/input/warning_code/json",
                    self.mqtt_config.namespace(),
                    self.inverter.datalog()
                ),
                value_template: ValueTemplate::None,
                icon: Some("mdi:code-json"),
                ..base.clone()
            },
            Entity {
                key: "bms_event_1",
                name: "BMS Fault Code",
                entity_category: Some("diagnostic"),
                state_topic: &format!(
                    "{}/{}/input/bms_event_1/parsed",
                    self.mqtt_config.namespace(),
                    self.inverter.datalog()
                ),
                value_template: ValueTemplate::None,
                icon: Some("mdi:battery-alert"),
                ..base.clone()
            },
            Entity {
                key: "bms_event_2",
                name: "BMS Warning Code",
                entity_category: Some("diagnostic"),
                state_topic: &format!(
                    "{}/{}/input/bms_event_2/parsed",
                    self.mqtt_config.namespace(),
                    self.inverter.datalog()
                ),
                value_template: ValueTemplate::None,
                icon: Some("mdi:battery-alert-outline"),
                ..base.clone()
            },
            Entity {
                key: "all_bms_faults",
                name: "All Active BMS Faults",
                entity_category: Some("diagnostic"),
                state_topic: &format!(
                    "{}/{}/input/bms_event_1/all",
                    self.mqtt_config.namespace(),
                    self.inverter.datalog()
                ),
                value_template: ValueTemplate::String("{{ value | join(', ') }}".to_string()),
                icon: Some("mdi:battery-alert"),
                ..base.clone()
            },
            Entity {
                key: "all_bms_warnings",
                name: "All Active BMS Warnings",
                entity_category: Some("diagnostic"),
                state_topic: &format!(
                    "{}/{}/input/bms_event_2/all",
                    self.mqtt_config.namespace(),
                    self.inverter.datalog()
                ),
                value_template: ValueTemplate::String("{{ value | join(', ') }}".to_string()),
                icon: Some("mdi:battery-alert-outline"),
                ..base.clone()
            },
            Entity {
                key: "v_bat",
                name: "Battery Voltage",

                ..voltage.clone()
            },
            Entity {
                key: "v_ac_r",
                name: "Grid Voltage",

                ..voltage.clone()
            },
            Entity {
                key: "v_pv_1",
                name: "PV Voltage (String 1)",

                ..voltage.clone()
            },
            Entity {
                key: "v_pv_2",
                name: "PV Voltage (String 2)",

                ..voltage.clone()
            },
            Entity {
                key: "v_pv_3",
                name: "PV Voltage (String 3)",

                ..voltage.clone()
            },
            Entity {
                key: "v_eps_r",
                name: "EPS Voltage",

                ..voltage.clone()
            },
            Entity {
                key: "eps_voltage_l1n",
                name: "EPS Voltage L1N",
                state_topic: &format!(
                    "{}/{}/input/127",
                    self.mqtt_config.namespace(),
                    self.inverter.datalog()
                ),
                value_template: ValueTemplate::String("{{ (value | float) * 0.1 }}".to_string()),

                ..voltage.clone()
            },
            Entity {
                key: "eps_voltage_l2n",
                name: "EPS Voltage L2N",
                state_topic: &format!(
                    "{}/{}/input/128",
                    self.mqtt_config.namespace(),
                    self.inverter.datalog()
                ),
                value_template: ValueTemplate::String("{{ (value | float) * 0.1 }}".to_string()),

                ..voltage.clone()
            },
            Entity {
                key: "f_ac",
                name: "Grid Frequency",
                value_template: ValueTemplate::String("{{ (value | float) | round(3) }}".to_string()),

                ..frequency.clone()
            },
            Entity {
                key: "f_eps",
                name: "EPS Frequency",
                value_template: ValueTemplate::String("{{ (value | float) | round(3) }}".to_string()),

                ..frequency.clone()
            },
            Entity {
                key: "generator_frequency",
                name: "Generator Frequency",
                state_topic: &format!(
                    "{}/{}/input/122",
                    self.mqtt_config.namespace(),
                    self.inverter.datalog()
                ),
                value_template: ValueTemplate::String("{{ (value | float / 100) | round(3) }}".to_string()),
                device_class: Some("frequency"),
                state_class: Some("measurement"),
                unit_of_measurement: Some("Hz"),
                suggested_display_precision: Some(2),
                entity_category: Some("diagnostic"),
                ..base.clone()
            },
            Entity {
                key: "generator_voltage",
                name: "Generator Voltage",
                state_topic: &format!(
                    "{}/{}/input/121",
                    self.mqtt_config.namespace(),
                    self.inverter.datalog()
                ),
                value_template: ValueTemplate::String("{{ (value | float / 10) | round(1) }}".to_string()),
                device_class: Some("voltage"),
                state_class: Some("measurement"),
                unit_of_measurement: Some("V"),
                entity_category: Some("diagnostic"),
                ..base.clone()
            },
            Entity {
                key: "generator_power",
                name: "Generator Power",
                state_topic: &format!(
                    "{}/{}/input/123",
                    self.mqtt_config.namespace(),
                    self.inverter.datalog()
                ),
                value_template: ValueTemplate::String("{{ (value | float) | round(0) }}".to_string()),
                device_class: Some("power"),
                state_class: Some("measurement"),
                unit_of_measurement: Some("W"),
                entity_category: Some("diagnostic"),
                ..base.clone()
            },
            Entity {
                key: "s_eps",
                name: "Apparent EPS Power",
                device_class: Some("apparent_power"),
                unit_of_measurement: Some("VA"),

                ..power.clone()
            },
            Entity {
                key: "p_pv",
                name: "PV Power (Array)",

                ..power.clone()
            },
            Entity {
                key: "p_pv_1",
                name: "PV Power (String 1)",

                ..power.clone()
            },
            Entity {
                key: "p_pv_2",
                name: "PV Power (String 2)",

                ..power.clone()
            },
            Entity {
                key: "p_pv_3",
                name: "PV Power (String 3)",

                ..power.clone()
            },
            Entity {
                key: "p_battery",
                name: "Battery Power (discharge is negative)",

                ..power.clone()
            },
            Entity {
                key: "p_charge",
                name: "Battery Charge",

                ..power.clone()
            },
            Entity {
                key: "p_discharge",
                name: "Battery Discharge",

                ..power.clone()
            },
            Entity {
                key: "p_grid",
                name: "Grid Power (export is negative)",

                ..power.clone()
            },
            Entity {
                key: "p_to_user",
                name: "Power from Grid",

                ..power.clone()
            },
            Entity {
                key: "p_to_grid",
                name: "Power to Grid",

                ..power.clone()
            },
            Entity {
                key: "p_eps",
                name: "Active EPS Power",

                ..power.clone()
            },
            Entity {
                key: "p_inv",
                name: "Inverter Power",

                ..power.clone()
            },
            Entity {
                key: "p_rec",
                name: "AC Charge Power",

                ..power.clone()
            },
            Entity {
                key: "e_pv_all",
                name: "PV Generation (All time)",

                ..energy.clone()
            },
            Entity {
                key: "e_pv_all_1",
                name: "PV Generation (All time) (String 1)",

                ..energy.clone()
            },
            Entity {
                key: "e_pv_all_2",
                name: "PV Generation (All time) (String 2)",

                ..energy.clone()
            },
            Entity {
                key: "e_pv_all_3",
                name: "PV Generation (All time) (String 3)",

                ..energy.clone()
            },
            Entity {
                key: "e_pv_day",
                name: "PV Generation (Today))",

                ..energy.clone()
            },
            Entity {
                key: "e_pv_day_1",
                name: "PV Generation (Today) (String 1)",

                ..energy.clone()
            },
            Entity {
                key: "e_pv_day_2",
                name: "PV Generation (Today) (String 2)",

                ..energy.clone()
            },
            Entity {
                key: "e_pv_day_3",
                name: "PV Generation (Today) (String 3)",

                ..energy.clone()
            },
            Entity {
                key: "bat_capacity",
                name: "Battery Capacity",
                value_template: ValueTemplate::String("{{ float(value_json.bat_capacity) * 51.2 / 1000 }}".to_string()),

                ..energy_storage.clone()
            },
            Entity {
                key: "e_chg_all",
                name: "Battery Charge (All time)",

                ..energy.clone()
            },
            Entity {
                key: "e_chg_day",
                name: "Battery Charge (Today)",

                ..energy.clone()
            },
            Entity {
                key: "e_dischg_all",
                name: "Battery Discharge (All time)",

                ..energy.clone()
            },
            Entity {
                key: "e_dischg_day",
                name: "Battery Discharge (Today)",

                ..energy.clone()
            },
            Entity {
                key: "e_to_user_all",
                name: "Energy from Grid (All time)",

                ..energy.clone()
            },
            Entity {
                key: "e_to_user_day",
                name: "Energy from Grid (Today)",

                ..energy.clone()
            },
            Entity {
                key: "e_to_grid_all",
                name: "Energy to Grid (All time)",

                ..energy.clone()
            },
            Entity {
                key: "e_to_grid_day",
                name: "Energy to Grid (Today)",

                ..energy.clone()
            },
            Entity {
                key: "e_eps_all",
                name: "Energy from EPS (All time)",

                ..energy.clone()
            },
            Entity {
                key: "e_eps_day",
                name: "Energy from EPS (Today)",

                ..energy.clone()
            },
            Entity {
                key: "e_rec_all",
                name: "Energy of AC Charging (All time)",

                ..energy.clone()
            },
            Entity {
                key: "e_rec_day",
                name: "Energy of AC Charging (Today)",

                ..energy.clone()
            },
            Entity {
                key: "e_inv_all",
                name: "Energy of Inverter (All time)",

                ..energy.clone()
            },
            Entity {
                key: "e_inv_day",
                name: "Energy of Inverter (Today)",

                ..energy.clone()
            },
            Entity {
                key: "t_inner",
                name: "Inverter Temperature",

                ..temperature.clone()
            },
            Entity {
                key: "t_rad_1",
                name: "Radiator 1 Temperature",

                ..temperature.clone()
            },
            Entity {
                key: "t_rad_2",
                name: "Radiator 2 Temperature",

                ..temperature.clone()
            },
            Entity {
                key: "t_bat",
                name: "Battery Temperature",

                ..temperature.clone()
            },
            Entity {
                key: "max_chg_curr",
                name: "Max Charge Current",

                ..current.clone()
            },
            Entity {
                key: "max_dischg_curr",
                name: "Max Discharge Current",

                ..current.clone()
            },
            Entity {
                key: "inverter_current_rms",
                name: "Inverter Current RMS",
                state_topic: &format!(
                    "{}/{}/input/18",
                    self.mqtt_config.namespace(),
                    self.inverter.datalog()
                ),
                value_template: ValueTemplate::String("{{ (value | float) * 0.01 }}".to_string()),
                device_class: Some("current"),
                state_class: Some("measurement"),
                unit_of_measurement: Some("A"),
                entity_category: Some("diagnostic"),
                ..base.clone()
            },
            Entity {
                key: "min_cell_voltage",
                name: "Min Cell Voltage (BMS)",

                ..voltage.clone()
            },
            Entity {
                key: "max_cell_voltage",
                name: "Max Cell Voltage (BMS)",

                ..voltage.clone()
            },
            Entity {
                key: "min_cell_temp",
                name: "Min Cell Temperature (BMS)",

                ..temperature.clone()
            },
            Entity {
                key: "max_cell_temp",
                name: "Max Cell Temperature (BMS)",

                ..temperature.clone()
            },
            // System Information Sensors
            Entity {
                key: "model",
                name: "Inverter Model",
                state_topic: &format!(
                    "{}/{}/hold/0",
                    self.mqtt_config.namespace(),
                    self.inverter.datalog()
                ),
                entity_category: Some("diagnostic"),
                value_template: ValueTemplate::None,
                icon: Some("mdi:information"),
                ..base.clone()
            },
            Entity {
                key: "firmware",
                name: "Firmware Version",
                state_topic: &format!(
                    "{}/{}/hold/7",
                    self.mqtt_config.namespace(),
                    self.inverter.datalog()
                ),
                entity_category: Some("diagnostic"),
                value_template: ValueTemplate::None,
                icon: Some("mdi:chip"),
                ..base.clone()
            },
            Entity {
                key: "serial_number",
                name: "Serial Number",
                state_topic: &format!(
                    "{}/{}/hold/2",
                    self.mqtt_config.namespace(),
                    self.inverter.datalog()
                ),
                entity_category: Some("diagnostic"),
                value_template: ValueTemplate::None,
                icon: Some("mdi:barcode"),
                ..base.clone()
            },
            Entity {
                key: "runtime",
                name: "Total Runtime",
                entity_category: Some("diagnostic"),
                device_class: Some("duration"),
                state_class: Some("total_increasing"),
                unit_of_measurement: Some("s"),
                ..base.clone()
            },
            Entity {
                key: "charge_volt_ref",
                name: "Charge Voltage Reference",

                ..voltage.clone()
            },
            Entity {
                key: "dischg_cut_volt",
                name: "Discharge Cut-off Voltage",

                ..voltage.clone()
            },
            Entity {
                key: "bat_current",
                name: "Battery Current",

                ..current.clone()
            },
            Entity {
                key: "vbat_inv",
                name: "Inverter Battery Voltage",

                ..voltage.clone()
            },
            Entity {
                key: "export_power_to_grid",
                name: "Export Power to Grid",
                state_topic: &format!(
                    "{}/{}/input/26",
                    self.mqtt_config.namespace(),
                    self.inverter.datalog()
                ),
                value_template: ValueTemplate::None,
                device_class: Some("power"),
                state_class: Some("measurement"),
                unit_of_measurement: Some("W"),
                entity_category: Some("diagnostic"),
                ..base.clone()
            },
            Entity {
                key: "import_power_from_grid",
                name: "Import Power from Grid",
                state_topic: &format!(
                    "{}/{}/input/27",
                    self.mqtt_config.namespace(),
                    self.inverter.datalog()
                ),
                value_template: ValueTemplate::None,
                device_class: Some("power"),
                state_class: Some("measurement"),
                unit_of_measurement: Some("W"),
                entity_category: Some("diagnostic"),
                ..base.clone()
            },
            Entity {
                key: "pv1_power_generation_today",
                name: "PV1 Power Generation Today",
                state_topic: &format!(
                    "{}/{}/input/28",
                    self.mqtt_config.namespace(),
                    self.inverter.datalog()
                ),
                value_template: ValueTemplate::String("{{ (value | float) * 0.1 }}".to_string()),
                device_class: Some("energy"),
                state_class: Some("total_increasing"),
                unit_of_measurement: Some("kWh"),
                entity_category: Some("diagnostic"),
                ..base.clone()
            },
            Entity {
                key: "f_ac",
                name: "Grid Frequency",
                value_template: ValueTemplate::String("{{ (value | float) | round(3) }}".to_string()),

                ..frequency.clone()
            },
        ];

        sensors
            .map(|sensor| {
                // fill in unique_id and value_template (if default) which are derived from key
                let mut sensor = Entity {
                    unique_id: &self.unique_id(sensor.key),
                    ..sensor
                };
                if sensor.value_template.is_default() {
                    sensor.value_template = ValueTemplate::from_default(sensor.key);
                }

                mqtt::Message {
                    topic: self.ha_discovery_topic("sensor", sensor.key),
                    retain: true,
                    payload: serde_json::to_string(&sensor).unwrap(),
                }
            })
            .to_vec()
    }

    /// Generate all Home Assistant entity discovery messages
    /// 
    /// Entities are organized into logical functional groups to improve the user experience
    /// in Home Assistant by presenting related controls together:
    /// 
    /// 1. SYSTEM CONTROL: Basic system operations (restart)
    /// 2. SAFETY & PROTECTION: Core safety features and grid protection
    /// 3. POWER MANAGEMENT: Battery backup, grid interaction, and demand response
    /// 4. CHARGING CONTROLS: AC charging, charge priority, and system charging settings
    /// 5. FREQUENCY CONTROLS: Over/under frequency management and response timing
    /// 6. TIME-BASED CONTROLS: All timeslot configurations for various operations
    /// 7. ADVANCED FEATURES: Forced discharge and register 110 switches
    /// 8. SYSTEM LIMITS: Discharge cutoff and EPS settings
    /// 9. SENSORS: All monitoring and status sensors (added at the end)
    pub fn all(&self) -> Result<Vec<mqtt::Message>> {
        let mut r = vec![
            // Clean up duplicate anti_islanding topic on startup
            self.remove_old_entity("anti_islanding")?,
            
            // Clean up removed off-grid duplicate entities on startup
            self.remove_old_entity("off_grid_frequency")?,
            self.remove_old_entity("off_grid_voltage_l1")?,
            self.remove_old_entity("off_grid_voltage_l2")?,
            self.remove_old_entity("off_grid_inverter_power")?,
            self.remove_old_entity("off_grid_apparent_power")?,
            
            // ===== SYSTEM CONTROL =====
            // System control
            self.button("restart", "Restart Inverter", Some("diagnostic".to_string()))?,
            
            // ===== SAFETY & PROTECTION =====
            // Core safety features
            self.switch("anti_island", "Anti-Islanding", Some("config".to_string()))?,
            self.switch("neutral_detect", "Zero ground detection", Some("config".to_string()))?,
            self.switch("iso", "ISO", Some("config".to_string()))?,
            self.switch("gfci", "GFCI", Some("config".to_string()))?,
            self.switch("dci", "DCI", Some("config".to_string()))?,
            
            // Grid protection
            self.switch("grid_on_power_ss", "Grid-connected soft start", Some("config".to_string()))?,
            self.switch("lvrt", "Low Voltage Ride Through", Some("config".to_string()))?,
            self.switch("ovf_load_derate", "Over Frequency Load Derate", Some("config".to_string()))?,
            
            // ===== POWER MANAGEMENT =====
            // Battery backup and grid interaction
            self.switch("eps", "Battery Backup", Some("config".to_string()))?,
            self.switch("sw_seamless", "Off Grid Seamless Switching", Some("config".to_string()))?,
            self.switch("feed_in_grid", "Grid Sell Back", Some("config".to_string()))?,
            self.switch("set_to_standby", "Power On", None)?,
            
            // Demand response and advanced features
            self.switch("drms", "Demand Response Mode", Some("config".to_string()))?,
            
            // ===== CHARGING CONTROLS =====
            // AC charging
            self.switch("ac_charge", "AC Charge", None)?,
            self.number_percent(Register::AcChargePowerCmd, "AC Charge Rate (%)", 0.0, 100.0, 1.0, None, None)?,
            self.number_percent(Register::AcChargeSocLimit, "AC Charge Limit %", 0.0, 100.0, 1.0, None, None)?,
            self.number_percent(
                Register::AcChargeStartSocLimit,
                "Charge From AC Lower Limit %",
                0.0, 100.0, 1.0, None, None
            )?,
            self.number_percent(
                Register::AcChargeEndSocLimit,
                "Charge From AC Upper Limit %",
                0.0, 100.0, 1.0, None, None
            )?,
            
            // Charge priority
            self.switch("charge_priority", "PV Charge Priority", None)?,
            self.number_percent(Register::ChargePriorityPowerCmd, "Charge Priority Rate (%)", 0.0, 100.0, 1.0, None, None)?,
            self.number_percent(Register::ChargePrioritySocLimit, "Charge Priority Limit %", 0.0, 100.0, 1.0, None, None)?,
            
            // System charging
            self.number_percent(Register::ChargePowerPercentCmd, "System Charge Rate (%)", 0.0, 100.0, 1.0, None, None)?,
            self.number_percent(Register::DischgPowerPercentCmd, "System Discharge Rate (%)", 0.0, 100.0, 1.0, None, None)?,
            
            // ===== FREQUENCY CONTROLS =====
            // Over frequency controls
            self.number_hz(Register::OVFDerateStart, "Over Frequency Derate Start (Hz)", None, Some("config".to_string()))?,
            self.number_hz(Register::OVFDerateEnd, "Over Frequency Derate End (Hz)", None, Some("config".to_string()))?,
            self.number_percent_per_hz(Register::OVFDeratePctPerHz, "Over Frequency Derate Rate (%/Hz)", None, Some("config".to_string()))?,
            
            // Under frequency controls
            self.number_hz(Register::UnderFrDroopStart, "Under Frequency Droop Start (Hz)", None, Some("config".to_string()))?,
            self.number_hz(Register::UnderFrDroopEnd, "Under Frequency Droop End (Hz)", None, Some("config".to_string()))?,
            self.number_percent_per_hz(Register::UnderFrIncreasePctPerHz, "Under Frequency Increase Rate (%/Hz)", None, Some("config".to_string()))?,
            
            // Frequency response timing
            self.number_ms(Register::DelayTimeForOverFDerate, "Frequency Response Delay Time (ms)", None, Some("config".to_string()))?,
            
            // ===== REACTIVE POWER CONTROLS =====
            // Q(V) Curve Control Points (Registers 54-58)
            self.number_percent(Register::MaxQPercentForQV, "Reactive Power - Q1 Reactive Power (%)", 0.0, 100.0, 1.0, Some(false), Some("config".to_string()))?,
            self.number_voltage(Register::V1L, "Reactive Power - Q(V) Curve V1 Low (V)", 180.0, 280.0, 0.1, Some(false), Some("config".to_string()))?,
            self.number_voltage(Register::V2L, "Reactive Power - Q(V) Curve V2 Low (V)", 180.0, 280.0, 0.1, Some(false), Some("config".to_string()))?,
            self.number_voltage(Register::V1H, "Reactive Power - Q(V) Curve V1 High (V)", 180.0, 280.0, 0.1, Some(false), Some("config".to_string()))?,
            self.number_voltage(Register::V2H, "Reactive Power - Q(V) Curve V2 High (V)", 180.0, 280.0, 0.1, Some(false), Some("config".to_string()))?,
            
            // Reactive Power Command Type and Values (Registers 59-62)
            self.number_percent(Register::ReactivePowerCMDType, "Reactive Power - Command Type", 0.0, 100.0, 1.0, Some(false), Some("config".to_string()))?,
            self.number_percent(Register::ActivePowerPercentCMD, "Reactive Power - Active Power Percentage Command (%)", 0.0, 100.0, 1.0, Some(false), Some("config".to_string()))?,
            self.number_percent(Register::ReactivePowerPercentCMD, "Reactive Power - Reactive Power Percentage Command (%)", 0.0, 100.0, 1.0, Some(false), Some("config".to_string()))?,
            self.number_percent(Register::PFCMD, "Reactive Power - Power Factor Command", 0.0, 100.0, 1.0, Some(false), Some("config".to_string()))?,
            
            // Q(V) Curve Reference Parameters (Registers 185-186)
            self.number_voltage(Register::VrefQv, "Reactive Power - Q(V) Curve Reference Voltage (V)", 180.0, 280.0, 0.1, Some(false), Some("config".to_string()))?,
            self.number_time(Register::VrefFiltertime, "Reactive Power - Q(V) Curve Voltage Filter Time (s)", 0.0, 300.0, 0.01, Some(false), Some("config".to_string()))?,
            
            // Q(V) Curve Control Points (Registers 121, 187-188)
            self.number_percent(Register::Q2Qv, "Reactive Power - Q(V) Curve Q2 Reactive Power (%)", 0.0, 100.0, 1.0, Some(false), Some("config".to_string()))?,
            self.number_percent(Register::Q3Qv, "Reactive Power - Q(V) Curve Q3 Reactive Power (%)", 0.0, 100.0, 1.0, Some(false), Some("config".to_string()))?,
            self.number_percent(Register::Q4Qv, "Reactive Power - Q(V) Curve Q4 Reactive Power (%)", 0.0, 100.0, 1.0, Some(false), Some("config".to_string()))?,
            
            // QP Priority Control Points (Registers 189-191)
            self.number_percent(Register::P1Qp, "Reactive Power - QP Priority Curve P1 Power (%)", 0.0, 100.0, 1.0, Some(false), Some("config".to_string()))?,
            self.number_percent(Register::P2Qp, "Reactive Power - QP Priority Curve P2 Power (%)", 0.0, 100.0, 1.0, Some(false), Some("config".to_string()))?,
            self.number_percent(Register::P3Qp, "Reactive Power - QP Priority Curve P3 Power (%)", 0.0, 100.0, 1.0, Some(false), Some("config".to_string()))?,
            
            // Volt-Watt Open Loop Response Time (Register 183)
            self.number_time(Register::VoltWattDelayTime, "Reactive Power - Volt-Watt Open Loop Response Time (s)", 0.0, 300.0, 0.01, Some(false), Some("config".to_string()))?,
            
            // Generator Configuration
            self.number_time(Register::GeneratorCoolDownTime, "Generator Cool-Down Time (min)", 0.0, 60.0, 0.1, None, Some("config".to_string()))?,
            
            // ===== AC COUPLING CONFIGURATION =====
            self.switch_register179("ac_coupling_enable", "AC Coupling Enable", Some("config".to_string()))?,
            self.number_percent(Register::ACCoupleStartSOC, "AC Couple Start SOC (%)", 0.0, 100.0, 1.0, None, Some("config".to_string()))?,
            self.number_percent(Register::ACCoupleEndSOC, "AC Couple End SOC (%)", 0.0, 101.0, 1.0, None, Some("config".to_string()))?,
            self.number_voltage(Register::ACCoupleStartVolt, "AC Couple Start Voltage (V)", 40.0, 100.0, 0.1, None, Some("config".to_string()))?,
            self.number_voltage(Register::ACCoupleEndVolt, "AC Couple End Voltage (V)", 40.0, 100.0, 0.1, None, Some("config".to_string()))?,
            
            // ===== SMART LOAD CONFIGURATION =====
            self.switch_register179("smart_load_enable", "Smart Load Enable", Some("config".to_string()))?,
            self.switch_register137("grid_always_on", "Grid Always On", Some("config".to_string()))?,
            self.number_voltage(Register::SmartLoadStartVolt, "Smart Load Start Voltage (V)", 40.0, 60.0, 0.1, None, Some("config".to_string()))?,
            self.number_voltage(Register::SmartLoadEndVolt, "Smart Load End Voltage (V)", 40.0, 60.0, 0.1, None, Some("config".to_string()))?,
            self.number_percent(Register::SmartLoadStartSOC, "Smart Load Start SOC (%)", 0.0, 100.0, 1.0, None, Some("config".to_string()))?,
            self.number_percent(Register::SmartLoadEndSOC, "Smart Load End SOC (%)", 0.0, 100.0, 1.0, None, Some("config".to_string()))?,
            self.number_power(Register::StartPVPower, "Start PV Power (kW)", 0.0, 10.0, 0.1, None, Some("config".to_string()))?,
            
            // ===== CONNECTION & RECONNECTION =====
            // Grid connection settings
            self.number_time(Register::GridConnectTime, "Grid Connection Delay (s)", 30.0, 600.0, 1.0, Some(false), Some("config".to_string()))?,
            self.number_time(Register::GridReconnectTime, "Grid Reconnection Delay (s)", 0.0, 900.0, 1.0, Some(false), Some("config".to_string()))?,
            self.number_voltage(Register::GridVoltConnLow, "Grid Voltage Low Limit (V)", 180.0, 280.0, 0.1, Some(false), Some("config".to_string()))?,
            self.number_voltage(Register::GridVoltConnHigh, "Grid Voltage High Limit (V)", 180.0, 280.0, 0.1, Some(false), Some("config".to_string()))?,
            self.number_frequency(Register::GridFreqConnLow, "Grid Frequency Low Limit (Hz)", 47.5, 63.0, 0.01, Some(false), Some("config".to_string()))?,
            self.number_frequency(Register::GridFreqConnHigh, "Grid Frequency High Limit (Hz)", 47.5, 63.0, 0.01, Some(false), Some("config".to_string()))?,
            
            // ===== INTERFACE PROTECTION =====
            // Grid Voltage Protection Level 1
            self.number_voltage(Register::GridVoltLimit1Low, "Grid Voltage Limit1 Low (V)", 180.0, 280.0, 0.1, Some(false), Some("config".to_string()))?,
            self.number_voltage(Register::GridVoltLimit1High, "Grid Voltage Limit1 High (V)", 180.0, 280.0, 1.0, Some(false), Some("config".to_string()))?,
            self.number_time(Register::GridVoltLimit1LowTime, "Grid Voltage Limit1 Low Time (s)", 0.0, 300.0, 0.01, Some(false), Some("config".to_string()))?,
            self.number_time(Register::GridVoltLimit1HighTime, "Grid Voltage Limit1 High Time (s)", 0.0, 300.0, 0.01, Some(false), Some("config".to_string()))?,
            // Grid Voltage Protection Level 2
            self.number_voltage(Register::GridVoltLimit2Low, "Grid Voltage Limit2 Low (V)", 180.0, 280.0, 0.1, Some(false), Some("config".to_string()))?,
            self.number_voltage(Register::GridVoltLimit2High, "Grid Voltage Limit2 High (V)", 180.0, 280.0, 0.1, Some(false), Some("config".to_string()))?,
            self.number_time(Register::GridVoltLimit2LowTime, "Grid Voltage Limit2 Low Time (s)", 0.0, 300.0, 0.01, Some(false), Some("config".to_string()))?,
            // Grid Voltage Protection Level 3
            self.number_voltage(Register::GridVoltLimit3Low, "Grid Voltage Limit3 Low (V)", 180.0, 280.0, 0.1, Some(false), Some("config".to_string()))?,
            self.number_voltage(Register::GridVoltLimit3High, "Grid Voltage Limit3 High (V)", 180.0, 280.0, 0.1, Some(false), Some("config".to_string()))?,
            self.number_time(Register::GridVoltLimit3LowTime, "Grid Voltage Limit3 Low Time (s)", 0.0, 300.0, 0.01, Some(false), Some("config".to_string()))?,
            self.number_time(Register::GridVoltLimit3HighTime, "Grid Voltage Limit3 High Time (s)", 0.0, 300.0, 0.01, Some(false), Some("config".to_string()))?,
            // Grid Frequency Protection Level 1
            self.number_frequency(Register::GridFreqLimit1Low, "Grid Frequency Limit1 Low (Hz)", 47.5, 63.0, 0.01, Some(false), Some("config".to_string()))?,
            self.number_frequency(Register::GridFreqLimit1High, "Grid Frequency Limit1 High (Hz)", 47.5, 63.0, 0.01, Some(false), Some("config".to_string()))?,
            self.number_time(Register::GridFreqLimit1LowTime, "Grid Frequency Limit1 Low Time (s)", 0.0, 300.0, 0.01, Some(false), Some("config".to_string()))?,
            self.number_time(Register::GridFreqLimit1HighTime, "Grid Frequency Limit1 High Time (s)", 0.0, 300.0, 0.01, Some(false), Some("config".to_string()))?,
            // Grid Frequency Protection Level 2
            self.number_frequency(Register::GridFreqLimit2Low, "Grid Frequency Limit2 Low (Hz)", 47.5, 63.0, 0.01, Some(false), Some("config".to_string()))?,
            self.number_frequency(Register::GridFreqLimit2High, "Grid Frequency Limit2 High (Hz)", 47.5, 63.0, 0.01, Some(false), Some("config".to_string()))?,
            self.number_time(Register::GridFreqLimit2LowTime, "Grid Frequency Limit2 Low Time (s)", 0.0, 300.0, 0.01, Some(false), Some("config".to_string()))?,
            self.number_time(Register::GridFreqLimit2HighTime, "Grid Frequency Limit2 High Time (s)", 0.0, 300.0, 0.01, Some(false), Some("config".to_string()))?,
            // Grid Frequency Protection Level 3
            self.number_frequency(Register::GridFreqLimit3Low, "Grid Frequency Limit3 Low (Hz)", 47.5, 63.0, 0.01, Some(false), Some("config".to_string()))?,
            self.number_frequency(Register::GridFreqLimit3High, "Grid Frequency Limit3 High (Hz)", 47.5, 63.0, 0.01, Some(false), Some("config".to_string()))?,
            self.number_time(Register::GridFreqLimit3LowTime, "Grid Frequency Limit3 Low Time (s)", 0.0, 300.0, 0.01, Some(false), Some("config".to_string()))?,
            self.number_time(Register::GridFreqLimit3HighTime, "Grid Frequency Limit3 High Time (s)", 0.0, 300.0, 0.01, Some(false), Some("config".to_string()))?,
            
            // ===== TIME-BASED CONTROLS =====
            // AC charge timeslots
            self.time_range("ac_charge/1", "AC Charge Timeslot 1")?,
            self.time_range("ac_charge/2", "AC Charge Timeslot 2")?,
            self.time_range("ac_charge/3", "AC Charge Timeslot 3")?,
            
            // AC first timeslots
            self.time_range("ac_first/1", "AC First Timeslot 1")?,
            self.time_range("ac_first/2", "AC First Timeslot 2")?,
            self.time_range("ac_first/3", "AC First Timeslot 3")?,
            
            // Charge priority timeslots
            self.time_range("charge_priority/1", "Charge Priority Timeslot 1")?,
            self.time_range("charge_priority/2", "Charge Priority Timeslot 2")?,
            self.time_range("charge_priority/3", "Charge Priority Timeslot 3")?,
            
            // Forced discharge timeslots
            self.time_range("forced_discharge/1", "Forced Discharge Timeslot 1")?,
            self.time_range("forced_discharge/2", "Forced Discharge Timeslot 2")?,
            self.time_range("forced_discharge/3", "Forced Discharge Timeslot 3")?,
            
            // ===== ADVANCED FEATURES =====
            // Forced discharge
            self.switch("forced_discharge", "Forced Discharge", None)?,
            self.number_percent(Register::ForcedDischgSocLimit, "Forced Discharge Limit %", 0.0, 100.0, 1.0, None, None)?,
            
            // Register 110 switches (advanced features)
            self.switch_register110("pv_off_grid", "PV Off Grid Enable", Some("config".to_string()))?,
            self.switch_register110("fast_zero_export", "Fast Zero Export Enable", Some("config".to_string()))?,
            self.switch_register110("micro_grid", "Micro Grid Enable", Some("config".to_string()))?,
            self.switch_register110("shared_battery", "Shared Battery Enable", Some("config".to_string()))?,
            self.switch_register110("charge_last", "Charge Last Enable", None)?,
            
            // ===== LCD CONFIGURATION =====
            self.number_lcd_password()?,
            
            // ===== SYSTEM LIMITS =====
            // Discharge cutoff
            self.number_percent(Register::DischgCutOffSocEod, "Discharge Cutoff %", 0.0, 100.0, 1.0, None, None)?,
            self.number_percent(
                Register::EpsDischgCutoffSocEod,
                "Discharge Cutoff for EPS %",
                0.0, 100.0, 1.0, None, None
            )?,
        ];

        r.append(&mut self.sensors());

        Ok(r)
    }

    fn ha_discovery_topic(&self, kind: &str, name: &str) -> String {
        format!(
            "{}/{}/{}_{}/{}/config",
            self.mqtt_config.homeassistant_prefix(),
            kind,
            self.mqtt_config.namespace(),
            self.inverter.datalog(),
            // The forward slash is used in some names (e.g. ac_charge/1) but
            // has semantic meaning in MQTT, so must be changed
            name.replace('/', "_"),
        )
    }


    fn switch(&self, name: &str, label: &str, entity_category: Option<String>) -> Result<mqtt::Message> {
        let config = Switch {
            value_template: format!("{{{{ value_json.{name}_en }}}}"),
            state_topic: format!(
                "{}/{}/hold/21/bits",
                self.mqtt_config.namespace(),
                self.inverter.datalog()
            ),
            command_topic: format!(
                "{}/cmd/{}/set/{}",
                self.mqtt_config.namespace(),
                self.inverter.datalog(),
                name
            ),
            unique_id: format!("{}_{}_{}", self.mqtt_config.namespace(), self.inverter.datalog(), name),
            name: label.to_string(),
            entity_category: entity_category.map(|s| s.to_string()),
            device: self.device(),
            availability: self.availability(),
        };

        Ok(mqtt::Message {
            topic: self.ha_discovery_topic("switch", name),
            retain: true,
            payload: serde_json::to_string(&config)?,
        })
    }

    fn switch_register110(&self, name: &str, label: &str, entity_category: Option<String>) -> Result<mqtt::Message> {
        let config = Switch {
            value_template: format!("{{{{ value_json.{name}_en }}}}"),
            state_topic: format!(
                "{}/{}/hold/110/bits",
                self.mqtt_config.namespace(),
                self.inverter.datalog()
            ),
            command_topic: format!(
                "{}/cmd/{}/set/{}",
                self.mqtt_config.namespace(),
                self.inverter.datalog(),
                name
            ),
            unique_id: format!("{}_{}_{}", self.mqtt_config.namespace(), self.inverter.datalog(), name),
            name: label.to_string(),
            entity_category: entity_category.map(|s| s.to_string()), // Use provided value or None
            device: self.device(),
            availability: self.availability(),
        };

        Ok(mqtt::Message {
            topic: self.ha_discovery_topic("switch", name),
            retain: true,
            payload: serde_json::to_string(&config)?,
        })
    }

    fn switch_register179(&self, name: &str, label: &str, entity_category: Option<String>) -> Result<mqtt::Message> {
        // Map the name to the actual JSON field name
        let json_field = match name {
            "ac_coupling_enable" => "ac_coupling_enable",
            "smart_load_enable" => "smart_load_enable",
            _ => name,
        };

        let config = Switch {
            value_template: format!("{{{{ value_json.{json_field} }}}}"),
            state_topic: format!(
                "{}/{}/hold/179/bits",
                self.mqtt_config.namespace(),
                self.inverter.datalog()
            ),
            command_topic: format!(
                "{}/cmd/{}/set/{}",
                self.mqtt_config.namespace(),
                self.inverter.datalog(),
                name
            ),
            unique_id: format!("{}_{}_{}", self.mqtt_config.namespace(), self.inverter.datalog(), name),
            name: label.to_string(),
            entity_category: entity_category.map(|s| s.to_string()), // Use provided value or None
            device: self.device(),
            availability: self.availability(),
        };

        Ok(mqtt::Message {
            topic: self.ha_discovery_topic("switch", name),
            retain: true,
            payload: serde_json::to_string(&config)?,
        })
    }

    fn switch_register137(&self, name: &str, label: &str, entity_category: Option<String>) -> Result<mqtt::Message> {
        // Map the name to the actual JSON field name
        let json_field = match name {
            "grid_always_on" => "grid_always_on_disable",
            _ => name,
        };

        let config = Switch {
            value_template: if name == "grid_always_on" {
                // Invert the logic: 0=enabled, 1=disabled, so we show the opposite
                "{{ 'OFF' if value_json.grid_always_on_disable == 'ON' else 'ON' }}".to_string()
            } else {
                format!("{{{{ value_json.{json_field} }}}}")
            },
            state_topic: format!(
                "{}/{}/hold/137/bits",
                self.mqtt_config.namespace(),
                self.inverter.datalog()
            ),
            command_topic: format!(
                "{}/cmd/{}/set/{}",
                self.mqtt_config.namespace(),
                self.inverter.datalog(),
                name
            ),
            unique_id: format!("{}_{}_{}", self.mqtt_config.namespace(), self.inverter.datalog(), name),
            name: label.to_string(),
            entity_category: entity_category.map(|s| s.to_string()), // Use provided value or None
            device: self.device(),
            availability: self.availability(),
        };

        Ok(mqtt::Message {
            topic: self.ha_discovery_topic("switch", name),
            retain: true,
            payload: serde_json::to_string(&config)?,
        })
    }

    fn button(&self, name: &str, label: &str, entity_category: Option<String>) -> Result<mqtt::Message> {
        let config = Button {
            name: label.to_string(),
            command_topic: format!(
                "{}/cmd/{}/{}",
                self.mqtt_config.namespace(),
                self.inverter.datalog(),
                name
            ),
            unique_id: format!("{}_{}_{}", self.mqtt_config.namespace(), self.inverter.datalog(), name),
            entity_category: entity_category.map(|s| s.to_string()), // Use provided value or None
            device: self.device(),
            availability: self.availability(),
        };

        Ok(mqtt::Message {
            topic: self.ha_discovery_topic("button", name),
            retain: true,
            payload: serde_json::to_string(&config)?,
        })
    }

    /// Remove an old entity from Home Assistant by sending an empty retained message
    /// This is the proper way to remove entities from Home Assistant via MQTT discovery
    /// 
    /// Currently used to clean up the duplicate "anti_islanding" topic that conflicts
    /// with the correct "anti_island" topic. The duplicate topic is created by some
    /// external system and needs to be removed on startup.
    fn remove_old_entity(&self, name: &str) -> Result<mqtt::Message> {
        Ok(mqtt::Message {
            topic: self.ha_discovery_topic("switch", name),
            retain: true,
            payload: "".to_string(), // Empty payload tells Home Assistant to remove the entity
        })
    }

    fn number_power(
        &self,
        register: Register,
        label: &str,
        min: f64,
        max: f64,
        step: f64,
        enabled_by_default: Option<bool>,
        entity_category: Option<String>,
    ) -> Result<mqtt::Message> {
        // Map register to command name for proper MQTT routing
        let command_topic = match register {
            Register::StartPVPower => format!(
                "{}/cmd/{}/set/start_pv_power",
                self.mqtt_config.namespace(),
                self.inverter.datalog()
            ),
            _ => format!(
                "{}/cmd/{}/set/hold/{}",
                self.mqtt_config.namespace(),
                self.inverter.datalog(),
                register.clone() as u16
            ),
        };

        let config = Number {
            name: label.to_string(),
            state_topic: format!(
                "{}/{}/hold/{}",
                self.mqtt_config.namespace(),
                self.inverter.datalog(),
                register.clone() as u16,
            ),
            command_topic,
            value_template: "{{ float(value) / 10 }}".to_string(), // Convert from 0.1kW units
            unique_id: format!("{}_{}_number_{:?}", self.mqtt_config.namespace(), self.inverter.datalog(), register),
            entity_category, // Use provided value or None (which will be skipped during serialization and result in the controll being added to primary controlls.)  Options: diagnostic, config
            enabled_by_default, // Use provided value or None (which will be skipped during serialization)
            device: self.device(),
            availability: self.availability(),
            min,
            max,
            step,
            unit_of_measurement: "kW".to_string(),
        };

        Ok(mqtt::Message {
            topic: self.ha_discovery_topic("number", &format!("{register:?}")),
            retain: true,
            payload: serde_json::to_string(&config)?,
        })
    }

    fn number_percent(
        &self,
        register: Register,
        label: &str,
        min: f64,
        max: f64,
        step: f64,
        enabled_by_default: Option<bool>,
        entity_category: Option<String>,
    ) -> Result<mqtt::Message> {
        // Map register to command name for proper MQTT routing
        let command_topic = match register {
            Register::AcChargePowerCmd => format!(
                "{}/cmd/{}/set/ac_charge_rate_pct",
                self.mqtt_config.namespace(),
                self.inverter.datalog()
            ),
            Register::AcChargeSocLimit => format!(
                "{}/cmd/{}/set/ac_charge_soc_limit_pct",
                self.mqtt_config.namespace(),
                self.inverter.datalog()
            ),
            Register::ChargePowerPercentCmd => format!(
                "{}/cmd/{}/set/charge_rate_pct",
                self.mqtt_config.namespace(),
                self.inverter.datalog()
            ),
            Register::DischgPowerPercentCmd => format!(
                "{}/cmd/{}/set/discharge_rate_pct",
                self.mqtt_config.namespace(),
                self.inverter.datalog()
            ),
            Register::DischgCutOffSocEod => format!(
                "{}/cmd/{}/set/discharge_cutoff_soc_limit_pct",
                self.mqtt_config.namespace(),
                self.inverter.datalog()
            ),
            // Smart Load SOC registers have named commands
            Register::SmartLoadStartSOC => format!(
                "{}/cmd/{}/set/smart_load_start_soc",
                self.mqtt_config.namespace(),
                self.inverter.datalog()
            ),
            Register::SmartLoadEndSOC => format!(
                "{}/cmd/{}/set/smart_load_end_soc",
                self.mqtt_config.namespace(),
                self.inverter.datalog()
            ),
            // These registers don't have named commands, use raw register format
            Register::ChargePriorityPowerCmd | Register::ChargePrioritySocLimit | Register::ForcedDischgSocLimit | Register::AcChargeStartSocLimit | Register::AcChargeEndSocLimit | Register::EpsDischgCutoffSocEod | Register::MaxQPercentForQV | Register::ActivePowerPercentCMD | Register::ReactivePowerPercentCMD | Register::Q3Qv | Register::Q4Qv | Register::P1Qp | Register::P2Qp | Register::P3Qp | Register::ReactivePowerCMDType | Register::PFCMD | Register::Q2Qv | Register::VoltWattDelayTime | Register::ACCoupleStartSOC | Register::ACCoupleEndSOC => format!(
                "{}/cmd/{}/set/hold/{}",
                self.mqtt_config.namespace(),
                self.inverter.datalog(),
                register.clone() as u16
            ),
            _ => return Err(anyhow!("number_percent: unsupported register {:?}", register)),
        };

        let config = Number {
            name: label.to_string(),
            state_topic: format!(
                "{}/{}/hold/{}",
                self.mqtt_config.namespace(),
                self.inverter.datalog(),
                register.clone() as u16,
            ),
            command_topic,
            value_template: "{{ float(value) }}".to_string(),
            unique_id: format!("{}_{}_number_{:?}", self.mqtt_config.namespace(), self.inverter.datalog(), register),
            entity_category, // Use provided value or None (which will be skipped during serialization and result in the controll being added to primary controlls.)  Options: diagnostic, config
            enabled_by_default, // Use provided value or None (which will be skipped during serialization)
            device: self.device(),
            availability: self.availability(),
            min,
            max,
            step,
            unit_of_measurement: "%".to_string(),
        };

        Ok(mqtt::Message {
            topic: self.ha_discovery_topic("number", &format!("{register:?}")),
            retain: true,
            payload: serde_json::to_string(&config)?,
        })
    }

    fn number_hz(
        &self,
        register: Register,
        label: &str,
        enabled_by_default: Option<bool>,
        entity_category: Option<String>,
    ) -> Result<mqtt::Message> {
        // Map register to command name for proper MQTT routing
        let command_name = match register {
            Register::OVFDerateStart => "ovf_derate_start_hz",
            Register::OVFDerateEnd => "ovf_derate_end_hz",
            Register::UnderFrDroopStart => "under_fr_droop_start_hz",
            Register::UnderFrDroopEnd => "under_fr_droop_end_hz",
            _ => return Err(anyhow!("number_hz: unsupported register {:?}", register)),
        };

        let config = Number {
            name: label.to_string(),
            state_topic: format!(
                "{}/{}/hold/{}",
                self.mqtt_config.namespace(),
                self.inverter.datalog(),
                register.clone() as u16,
            ),
            command_topic: format!(
                "{}/cmd/{}/set/{}",
                self.mqtt_config.namespace(),
                self.inverter.datalog(),
                command_name,
            ),
            value_template: "{{ float(value) / 100 }}".to_string(),
            unique_id: format!("{}_{}_number_{:?}", self.mqtt_config.namespace(), self.inverter.datalog(), register),
            entity_category, // Use provided value or None (which will be skipped during serialization)
            enabled_by_default, // Use provided value or None (which will be skipped during serialization)
            device: self.device(),
            availability: self.availability(),
            min: 0.0,
            max: 100.0,
            step: 0.01,
            unit_of_measurement: "Hz".to_string(),
        };

        Ok(mqtt::Message {
            topic: self.ha_discovery_topic("number", &format!("{register:?}")),
            retain: true,
            payload: serde_json::to_string(&config)?,
        })
    }

    fn number_percent_per_hz(
        &self,
        register: Register,
        label: &str,
        enabled_by_default: Option<bool>,
        entity_category: Option<String>,
    ) -> Result<mqtt::Message> {
        // Map register to command name for proper MQTT routing
        let command_name = match register {
            Register::OVFDeratePctPerHz => "ovf_derate_pct_per_hz",
            Register::UnderFrIncreasePctPerHz => "under_fr_increase_pct_per_hz",
            _ => return Err(anyhow!("number_percent_per_hz: unsupported register {:?}", register)),
        };

        let config = Number {
            name: label.to_string(),
            state_topic: format!(
                "{}/{}/hold/{}",
                self.mqtt_config.namespace(),
                self.inverter.datalog(),
                register.clone() as u16,
            ),
            command_topic: format!(
                "{}/cmd/{}/set/{}",
                self.mqtt_config.namespace(),
                self.inverter.datalog(),
                command_name,
            ),
            value_template: "{{ value }}".to_string(),
            unique_id: format!("{}_{}_number_{:?}", self.mqtt_config.namespace(), self.inverter.datalog(), register),
            entity_category, // Use provided value or None (which will be skipped during serialization)
            enabled_by_default, // Use provided value or None (which will be skipped during serialization)
            device: self.device(),
            availability: self.availability(),
            min: 0.0,
            max: 100.0,
            step: 1.0,
            unit_of_measurement: "%/Hz".to_string(),
        };

        Ok(mqtt::Message {
            topic: self.ha_discovery_topic("number", &format!("{register:?}")),
            retain: true,
            payload: serde_json::to_string(&config)?,
        })
    }

    fn number_ms(
        &self,
        register: Register,
        label: &str,
        enabled_by_default: Option<bool>,
        entity_category: Option<String>,
    ) -> Result<mqtt::Message> {
        // Map register to command name for proper MQTT routing
        let command_name = match register {
            Register::DelayTimeForOverFDerate => "frequency_active_open_loop_response_time",
            _ => return Err(anyhow!("number_ms: unsupported register {:?}", register)),
        };

        let config = Number {
            name: label.to_string(),
            state_topic: format!(
                "{}/{}/hold/{}",
                self.mqtt_config.namespace(),
                self.inverter.datalog(),
                register.clone() as u16,
            ),
            command_topic: format!(
                "{}/cmd/{}/set/{}",
                self.mqtt_config.namespace(),
                self.inverter.datalog(),
                command_name,
            ),
            value_template: "{{ value }}".to_string(),
            unique_id: format!("{}_{}_number_{:?}", self.mqtt_config.namespace(), self.inverter.datalog(), register),
            entity_category, // Use provided value or None (which will be skipped during serialization)
            enabled_by_default, // Use provided value or None (which will be skipped during serialization)
            device: self.device(),
            availability: self.availability(),
            min: 0.0,
            max: 10000.0,
            step: 1.0,
            unit_of_measurement: "ms".to_string(),
        };

        Ok(mqtt::Message {
            topic: self.ha_discovery_topic("number", &format!("{register:?}")),
            retain: true,
            payload: serde_json::to_string(&config)?,
        })
    }

    fn number_voltage(
        &self,
        register: Register,
        label: &str,
        min: f64,
        max: f64,
        step: f64,
        enabled_by_default: Option<bool>,
        entity_category: Option<String>,
    ) -> Result<mqtt::Message> {
        // Map register to command name for proper MQTT routing
        let command_name = match register {
            Register::GridVoltConnLow => "grid_voltage_low",
            Register::GridVoltConnHigh => "grid_voltage_high",
            // Interface Protection - Grid Voltage Limits
            Register::GridVoltLimit1Low => "grid_volt_limit1_low",
            Register::GridVoltLimit1High => "grid_volt_limit1_high",
            Register::GridVoltLimit2Low => "grid_volt_limit2_low",
            Register::GridVoltLimit2High => "grid_volt_limit2_high",
            Register::GridVoltLimit3Low => "grid_volt_limit3_low",
            Register::GridVoltLimit3High => "grid_volt_limit3_high",
            // Reactive Power Q(V) Curve Voltage Points
            Register::V1L => "v1l",
            Register::V2L => "v2l",
            Register::V1H => "v1h",
            Register::V2H => "v2h",
            Register::VrefQv => "vref_qv",
            // AC Coupling Voltage Thresholds
            Register::ACCoupleStartVolt => "ac_couple_start_volt",
            Register::ACCoupleEndVolt => "ac_couple_end_volt",
            // Smart Load Voltage Thresholds
            Register::SmartLoadStartVolt => "smart_load_start_volt",
            Register::SmartLoadEndVolt => "smart_load_end_volt",
            _ => return Err(anyhow!("number_voltage: unsupported register {:?}", register)),
        };

        let config = Number {
            name: label.to_string(),
            state_topic: format!(
                "{}/{}/hold/{}",
                self.mqtt_config.namespace(),
                self.inverter.datalog(),
                register.clone() as u16,
            ),
            command_topic: format!(
                "{}/cmd/{}/set/{}",
                self.mqtt_config.namespace(),
                self.inverter.datalog(),
                command_name,
            ),
            value_template: "{{ float(value) / 10 }}".to_string(), // Convert from 0.1V units
            unique_id: format!("{}_{}_number_{:?}", self.mqtt_config.namespace(), self.inverter.datalog(), register),
            entity_category, // Use provided value or None (which will be skipped during serialization)
            enabled_by_default, // Use provided value or None (which will be skipped during serialization)
            device: self.device(),
            availability: self.availability(),
            min,
            max,
            step,
            unit_of_measurement: "V".to_string(),
        };

        Ok(mqtt::Message {
            topic: self.ha_discovery_topic("number", &format!("{register:?}")),
            retain: true,
            payload: serde_json::to_string(&config)?,
        })
    }

    fn number_time(
        &self,
        register: Register,
        label: &str,
        min: f64,
        max: f64,
        step: f64,
        enabled_by_default: Option<bool>,
        entity_category: Option<String>,
    ) -> Result<mqtt::Message> {
        // Map register to command name for proper MQTT routing
        let command_name = match register {
            Register::GridConnectTime => "grid_connect_time",
            Register::GridReconnectTime => "grid_reconnect_time",
            // Interface Protection - Grid Voltage Time Limits
            Register::GridVoltLimit1LowTime => "grid_volt_limit1_low_time",
            Register::GridVoltLimit1HighTime => "grid_volt_limit1_high_time",
            Register::GridVoltLimit2LowTime => "grid_volt_limit2_low_time",
            Register::GridVoltLimit3LowTime => "grid_volt_limit3_low_time",
            Register::GridVoltLimit3HighTime => "grid_volt_limit3_high_time",
            // Interface Protection - Grid Frequency Time Limits
            Register::GridFreqLimit1LowTime => "grid_freq_limit1_low_time",
            Register::GridFreqLimit1HighTime => "grid_freq_limit1_high_time",
            Register::GridFreqLimit2LowTime => "grid_freq_limit2_low_time",
            Register::GridFreqLimit2HighTime => "grid_freq_limit2_high_time",
            Register::GridFreqLimit3LowTime => "grid_freq_limit3_low_time",
            Register::GridFreqLimit3HighTime => "grid_freq_limit3_high_time",
            // Reactive Power Q(V) Curve Time Parameters
            Register::VrefFiltertime => "vref_filtertime",
            // Volt-Watt Open Loop Response Time
            Register::VoltWattDelayTime => "volt_watt_delay_time",
            // Generator Configuration
            Register::GeneratorCoolDownTime => "generator_cool_down_time",
            _ => return Err(anyhow!("number_time: unsupported register {:?}", register)),
        };

        let config = Number {
            name: label.to_string(),
            state_topic: format!(
                "{}/{}/hold/{}",
                self.mqtt_config.namespace(),
                self.inverter.datalog(),
                register.clone() as u16,
            ),
            command_topic: format!(
                "{}/cmd/{}/set/{}",
                self.mqtt_config.namespace(),
                self.inverter.datalog(),
                command_name,
            ),
            value_template: if matches!(register, 
                Register::GridVoltLimit1LowTime | Register::GridVoltLimit1HighTime | 
                Register::GridVoltLimit2LowTime | Register::GridVoltLimit3LowTime | 
                Register::GridVoltLimit3HighTime | Register::GridFreqLimit1LowTime | 
                Register::GridFreqLimit1HighTime | Register::GridFreqLimit2LowTime | 
                Register::GridFreqLimit2HighTime | Register::GridFreqLimit3LowTime | 
                Register::GridFreqLimit3HighTime | Register::VrefFiltertime | Register::VoltWattDelayTime) {
                "{{ float(value) / 100 }}".to_string() // Convert from 0.01s units for interface protection and reactive power
            } else if matches!(register, Register::GeneratorCoolDownTime) {
                "{{ float(value) * 6 }}".to_string() // Convert from 0.1 minute units to seconds (0.1 * 60 = 6)
            } else {
                "{{ value }}".to_string() // No conversion for regular time registers
            },
            unique_id: format!("{}_{}_number_{:?}", self.mqtt_config.namespace(), self.inverter.datalog(), register),
            entity_category, // Use provided value or None (which will be skipped during serialization)
            enabled_by_default, // Use provided value or None (which will be skipped during serialization)
            device: self.device(),
            availability: self.availability(),
            min,
            max,
            step,
            unit_of_measurement: "s".to_string(), // All time values are in seconds
        };

        Ok(mqtt::Message {
            topic: self.ha_discovery_topic("number", &format!("{register:?}")),
            retain: true,
            payload: serde_json::to_string(&config)?,
        })
    }

    fn number_frequency(
        &self,
        register: Register,
        label: &str,
        min: f64,
        max: f64,
        step: f64,
        enabled_by_default: Option<bool>,
        entity_category: Option<String>,
    ) -> Result<mqtt::Message> {
        // Map register to command name for proper MQTT routing
        let command_name = match register {
            Register::GridFreqConnLow => "grid_frequency_low",
            Register::GridFreqConnHigh => "grid_frequency_high",
            // Interface Protection - Grid Frequency Limits
            Register::GridFreqLimit1Low => "grid_freq_limit1_low",
            Register::GridFreqLimit1High => "grid_freq_limit1_high",
            Register::GridFreqLimit2Low => "grid_freq_limit2_low",
            Register::GridFreqLimit2High => "grid_freq_limit2_high",
            Register::GridFreqLimit3Low => "grid_freq_limit3_low",
            Register::GridFreqLimit3High => "grid_freq_limit3_high",
            _ => return Err(anyhow!("number_frequency: unsupported register {:?}", register)),
        };

        let config = Number {
            name: label.to_string(),
            state_topic: format!(
                "{}/{}/hold/{}",
                self.mqtt_config.namespace(),
                self.inverter.datalog(),
                register.clone() as u16,
            ),
            command_topic: format!(
                "{}/cmd/{}/set/{}",
                self.mqtt_config.namespace(),
                self.inverter.datalog(),
                command_name,
            ),
            value_template: "{{ float(value) / 100 }}".to_string(), // Convert from 0.01Hz units
            unique_id: format!("{}_{}_number_{:?}", self.mqtt_config.namespace(), self.inverter.datalog(), register),
            entity_category, // Use provided value or None (which will be skipped during serialization)
            enabled_by_default, // Use provided value or None (which will be skipped during serialization)
            device: self.device(),
            availability: self.availability(),
            min,
            max,
            step,
            unit_of_measurement: "Hz".to_string(),
        };

        Ok(mqtt::Message {
            topic: self.ha_discovery_topic("number", &format!("{register:?}")),
            retain: true,
            payload: serde_json::to_string(&config)?,
        })
    }

    /// Creates a number entity for LCD password configuration
    fn number_lcd_password(&self) -> Result<mqtt::Message> {
        let config = Number {
            name: "LCD Password".to_string(),
            state_topic: format!(
                "{}/{}/hold/{}",
                self.mqtt_config.namespace(),
                self.inverter.datalog(),
                Register::LCDPassword as u16,
            ),
            command_topic: format!(
                "{}/cmd/{}/set/lcd_password",
                self.mqtt_config.namespace(),
                self.inverter.datalog(),
            ),
                            value_template: "{{ '%05d' | format(value) }}".to_string(), // Format as 5-digit zero-padded string
            unique_id: format!("{}_{}_number_lcd_password", self.mqtt_config.namespace(), self.inverter.datalog()),
            entity_category: Some("config".to_string()), // Place in Configuration section
            enabled_by_default: Some(false), // Disable by default for security
            device: self.device(),
            availability: self.availability(),
            min: 0.0,
            max: 65535.0, // u16 max value
            step: 1.0,
            unit_of_measurement: "".to_string(), // No unit for password
        };

        Ok(mqtt::Message {
            topic: self.ha_discovery_topic("number", "lcd_password"),
            retain: true,
            payload: serde_json::to_string(&config)?,
        })
    }



    // Models a time range as an MQTT Text field taking values like: 00:00-23:59
    fn time_range(&self, name: &str, label: &str) -> Result<mqtt::Message> {
        let config = Text {
            name: label.to_string(),
            state_topic: format!(
                "{}/{}/{}",
                self.mqtt_config.namespace(),
                self.inverter.datalog(),
                name
            ),
            command_topic: format!(
                "{}/cmd/{}/set/{}",
                self.mqtt_config.namespace(),
                self.inverter.datalog(),
                name
            ),
            command_template: r#"{% set parts = value.split("-") %}{"start":"{{ parts[0] }}", "end":"{{ parts[1] }}"}"#.to_string(),
            value_template: r#"{{ value_json["start"] }}-{{ value_json["end"] }}"#.to_string(),
            unique_id: format!("{}_{}_text_{}", self.mqtt_config.namespace(), self.inverter.datalog(), name.replace('/', "_")),
            entity_category: None, // Time-based controls go to primary controls
            device: self.device(),
            availability: self.availability(),
            pattern: r"([01]?[0-9]|2[0-3]):[0-5][0-9]-([01]?[0-9]|2[0-3]):[0-5][0-9]".to_string(),
        };

        Ok(mqtt::Message {
            topic: self.ha_discovery_topic("text", &name.replace('/', "_")),
            retain: true,
            payload: serde_json::to_string(&config)?,
        })
    }

    fn unique_id(&self, name: &str) -> String {
        format!("{}_{}_{}", self.mqtt_config.namespace(), self.inverter.datalog(), name)
    }

    fn device(&self) -> Device {
        Device {
            manufacturer: "LuxPower".to_string(),
            name: format!("LXP Inverter {}", self.inverter.datalog()),
            identifiers: [format!("{}_{}", self.mqtt_config.namespace(), self.inverter.datalog())],
        }
    }

    fn availability(&self) -> Availability {
        Availability {
            topic: format!("{}/LWT", self.mqtt_config.namespace()),
        }
    }
}
