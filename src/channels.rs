use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct Channels {
    pub from_inverter: broadcast::Sender<lxp::inverter::ChannelData>,
    pub to_inverter: broadcast::Sender<lxp::inverter::ChannelData>,
    pub from_mqtt: broadcast::Sender<mqtt::ChannelData>,
    pub to_mqtt: broadcast::Sender<mqtt::ChannelData>,
}

impl Default for Channels {
    fn default() -> Self {
        Self::new()
    }
}

impl Channels {
    pub fn new() -> Self {
        Self {
            from_inverter: Self::channel(),
            to_inverter: Self::channel(),
            from_mqtt: Self::channel(),
            to_mqtt: Self::channel(),
        }
    }

    fn channel<T: Clone>() -> broadcast::Sender<T> {
        // Increased buffer size to prevent overflow
        broadcast::channel(8192).0 // we only need tx half
    }

    /// Check if a channel is getting full and log a warning
    pub fn check_channel_health(&self) {
        // Check MQTT channels as they're most critical
        let receiver = self.to_mqtt.subscribe();
        let len = receiver.len();
        if len > 6000 { // 75% of buffer
            warn!("MQTT to_mqtt channel is getting full: {}/8192 messages", len);
        }
        
        let receiver = self.from_mqtt.subscribe();
        let len = receiver.len();
        if len > 6000 { // 75% of buffer
            warn!("MQTT from_mqtt channel is getting full: {}/8192 messages", len);
        }
    }
}
