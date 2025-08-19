use crate::prelude::*;

use rumqttc::{AsyncClient, Event, EventLoop, Incoming, LastWill, MqttOptions, Publish, QoS};

// Message {{{
#[derive(Eq, PartialEq, Debug, Clone)]
pub struct Message {
    pub topic: String,
    pub retain: bool,
    pub payload: String,
}

pub enum TargetInverter {
    Serial(Serial),
    All,
}

impl Message {
    pub fn for_param(rp: lxp::packet::ReadParam) -> Result<Vec<Message>> {
        let mut r = Vec::new();

        for (register, value) in rp.pairs() {
            r.push(mqtt::Message {
                topic: format!("{}/param/{}", rp.datalog, register),
                retain: true,
                payload: serde_json::to_string(&value)?,
            });
        }

        Ok(r)
    }

    pub fn for_hold(td: lxp::packet::TranslatedData) -> Result<Vec<Message>> {
        let mut r = Vec::new();

        for (register, value) in td.pairs() {
            r.push(mqtt::Message {
                topic: format!("{}/hold/{}", td.datalog, register),
                retain: true,
                payload: serde_json::to_string(&value)?,
            });

            if register == 21 {
                let bits = lxp::packet::Register21Bits::new(value);
                r.push(mqtt::Message {
                    topic: format!("{}/hold/{}/bits", td.datalog, register),
                    retain: true,
                    payload: serde_json::to_string(&bits)?,
                });
            }

            if register == 110 {
                let bits = lxp::packet::Register110Bits::new(value);
                r.push(mqtt::Message {
                    topic: format!("{}/hold/{}/bits", td.datalog, register),
                    retain: true,
                    payload: serde_json::to_string(&bits)?,
                });
            }

            if register == 179 {
                let bits = lxp::packet::Register179Bits::new(value);
                r.push(mqtt::Message {
                    topic: format!("{}/hold/{}/bits", td.datalog, register),
                    retain: true,
                    payload: serde_json::to_string(&bits)?,
                });
            }

            if register == 137 {
                let bits = lxp::packet::Register137Bits::new(value);
                r.push(mqtt::Message {
                    topic: format!("{}/hold/{}/bits", td.datalog, register),
                    retain: true,
                    payload: serde_json::to_string(&bits)?,
                });
            }
        }

        Ok(r)
    }

    pub fn for_input_all(
        inputs: &lxp::packet::ReadInputAll,
        datalog: lxp::inverter::Serial,
    ) -> Result<Message> {
        Ok(mqtt::Message {
            topic: format!("{datalog}/inputs/all"),
            retain: false,
            payload: serde_json::to_string(&inputs)?,
        })
    }

    pub fn for_input(
        td: lxp::packet::TranslatedData,
        publish_individual: bool,
    ) -> Result<Vec<Message>> {
        use lxp::packet::ReadInput;

        let mut r = Vec::new();

        if publish_individual {
            let mut fault_code_registers_seen = false;
            let mut fault_code = 0;
            let mut warning_code_registers_seen = false;
            let mut warning_code = 0;

            for (register, value) in td.pairs() {
                r.push(mqtt::Message {
                    topic: format!("{}/input/{}", td.datalog, register),
                    retain: false,
                    payload: serde_json::to_string(&value)?,
                });

                if register == 0 {
                    r.push(mqtt::Message {
                        topic: format!("{}/input/{}/parsed", td.datalog, register),
                        retain: false,
                        payload: lxp::packet::StatusString::from_value(value).to_owned(),
                    });
                }

                if register == 60 {
                    fault_code |= value as u32;
                    fault_code_registers_seen = true;
                }
                if register == 61 {
                    fault_code |= (value as u32) << 16;
                    fault_code_registers_seen = true;
                }

                if register == 62 {
                    warning_code |= value as u32;
                    warning_code_registers_seen = true;
                }
                if register == 63 {
                    warning_code |= (value as u32) << 16;
                    warning_code_registers_seen = true;
                }
            }

            if warning_code_registers_seen {
                r.push(mqtt::Message {
                    topic: format!("{}/input/warning_code/parsed", td.datalog),
                    retain: false,
                    payload: lxp::packet::WarningCodeString::from_value(warning_code).to_owned(),
                });
            }

            if fault_code_registers_seen {
                r.push(mqtt::Message {
                    topic: format!("{}/input/fault_code/parsed", td.datalog),
                    retain: false,
                    payload: lxp::packet::FaultCodeString::from_value(fault_code).to_owned(),
                });
            }
        }

        match td.read_input() {
            Ok(ReadInput::ReadInputAll(r_all)) => r.push(mqtt::Message {
                topic: format!("{}/inputs/all", td.datalog),
                retain: false,
                payload: serde_json::to_string(&r_all)?,
            }),
            Ok(ReadInput::ReadInput1(r1)) => r.push(mqtt::Message {
                topic: format!("{}/inputs/1", td.datalog),
                retain: false,
                payload: serde_json::to_string(&r1)?,
            }),
            Ok(ReadInput::ReadInput2(r2)) => r.push(mqtt::Message {
                topic: format!("{}/inputs/2", td.datalog),
                retain: false,
                payload: serde_json::to_string(&r2)?,
            }),
            Ok(ReadInput::ReadInput3(r3)) => r.push(mqtt::Message {
                topic: format!("{}/inputs/3", td.datalog),
                retain: false,
                payload: serde_json::to_string(&r3)?,
            }),
            Err(x) => warn!("ignoring {:?}", x),
        }

        Ok(r)
    }

    pub fn to_command(&self, inverter: config::Inverter) -> Result<Command> {
        use Command::*;

        let (_datalog, parts) = self.split_cmd_topic()?;

        let r = match parts[..] {
            ["read", "inputs", "1"] => ReadInputs(inverter, 1),
            ["read", "inputs", "2"] => ReadInputs(inverter, 2),
            ["read", "inputs", "3"] => ReadInputs(inverter, 3),
            ["read", "input", register] => {
                ReadInput(inverter, register.parse()?, self.payload_int_or_1()?)
            }
            ["read", "hold", register] => {
                ReadHold(inverter, register.parse()?, self.payload_int_or_1()?)
            }
            ["read", "param", register] => ReadParam(inverter, register.parse()?),
            ["read", "ac_charge", num] => ReadAcChargeTime(inverter, num.parse()?),
            ["read", "ac_first", num] => ReadAcFirstTime(inverter, num.parse()?),
            ["read", "charge_priority", num] => ReadChargePriorityTime(inverter, num.parse()?),
            ["read", "forced_discharge", num] => ReadForcedDischargeTime(inverter, num.parse()?),
            ["set", "hold", register] => SetHold(inverter, register.parse()?, self.payload_int()?),
            ["set", "param", register] => {
                WriteParam(inverter, register.parse()?, self.payload_int()?)
            }
            ["set", "ac_charge"] => AcCharge(inverter, self.payload_bool()),
            ["set", "ac_charge", num] => {
                SetAcChargeTime(inverter, num.parse()?, self.payload_start_end_time()?)
            }
            ["set", "ac_first", num] => {
                SetAcFirstTime(inverter, num.parse()?, self.payload_start_end_time()?)
            }
            ["set", "charge_priority"] => ChargePriority(inverter, self.payload_bool()),
            ["set", "charge_priority", num] => {
                SetChargePriorityTime(inverter, num.parse()?, self.payload_start_end_time()?)
            }
            ["set", "forced_discharge"] => ForcedDischarge(inverter, self.payload_bool()),
            ["set", "forced_discharge", num] => {
                SetForcedDischargeTime(inverter, num.parse()?, self.payload_start_end_time()?)
            }
            ["set", "charge_rate_pct"] => ChargeRate(inverter, self.payload_int()?),
            ["set", "discharge_rate_pct"] => DischargeRate(inverter, self.payload_int()?),
            ["set", "ac_charge_rate_pct"] => AcChargeRate(inverter, self.payload_int()?),

            ["set", "ac_charge_soc_limit_pct"] => AcChargeSocLimit(inverter, self.payload_int()?),

            ["set", "discharge_cutoff_soc_limit_pct"] => {
                DischargeCutoffSocLimit(inverter, self.payload_int()?)
            }
            // Connection and reconnection configuration
            ["set", "grid_connect_time"] => SetGridConnectTime(inverter, self.payload_int()?),
            ["set", "grid_reconnect_time"] => SetGridReconnectTime(inverter, self.payload_int()?),
            ["set", "grid_voltage_low"] => SetGridVoltageLow(inverter, self.payload_int()?),
            ["set", "grid_voltage_high"] => SetGridVoltageHigh(inverter, self.payload_int()?),
            ["set", "grid_frequency_low"] => SetGridFrequencyLow(inverter, self.payload_int()?),
            ["set", "grid_frequency_high"] => SetGridFrequencyHigh(inverter, self.payload_int()?),
            // Interface Protection - Grid Voltage Limits
            ["set", "grid_volt_limit1_low"] => SetGridVoltLimit1Low(inverter, self.payload_int()?),
            ["set", "grid_volt_limit1_high"] => SetGridVoltLimit1High(inverter, self.payload_int()?),
            ["set", "grid_volt_limit1_low_time"] => SetGridVoltLimit1LowTime(inverter, self.payload_int()?),
            ["set", "grid_volt_limit1_high_time"] => SetGridVoltLimit1HighTime(inverter, self.payload_int()?),
            ["set", "grid_volt_limit2_low"] => SetGridVoltLimit2Low(inverter, self.payload_int()?),
            ["set", "grid_volt_limit2_high"] => SetGridVoltLimit2High(inverter, self.payload_int()?),
            ["set", "grid_volt_limit2_low_time"] => SetGridVoltLimit2LowTime(inverter, self.payload_int()?),
            ["set", "grid_volt_limit3_low"] => SetGridVoltLimit3Low(inverter, self.payload_int()?),
            ["set", "grid_volt_limit3_high"] => SetGridVoltLimit3High(inverter, self.payload_int()?),
            ["set", "grid_volt_limit3_low_time"] => SetGridVoltLimit3LowTime(inverter, self.payload_int()?),
            ["set", "grid_volt_limit3_high_time"] => SetGridVoltLimit3HighTime(inverter, self.payload_int()?),
            // Interface Protection - Grid Frequency Limits
            ["set", "grid_freq_limit1_low"] => SetGridFreqLimit1Low(inverter, self.payload_int()?),
            ["set", "grid_freq_limit1_high"] => SetGridFreqLimit1High(inverter, self.payload_int()?),
            ["set", "grid_freq_limit1_low_time"] => SetGridFreqLimit1LowTime(inverter, self.payload_int()?),
            ["set", "grid_freq_limit1_high_time"] => SetGridFreqLimit1HighTime(inverter, self.payload_int()?),
            ["set", "grid_freq_limit2_low"] => SetGridFreqLimit2Low(inverter, self.payload_int()?),
            ["set", "grid_freq_limit2_high"] => SetGridFreqLimit2High(inverter, self.payload_int()?),
            ["set", "grid_freq_limit2_low_time"] => SetGridFreqLimit2LowTime(inverter, self.payload_int()?),
            ["set", "grid_freq_limit2_high_time"] => SetGridFreqLimit2HighTime(inverter, self.payload_int()?),
            ["set", "grid_freq_limit3_low"] => SetGridFreqLimit3Low(inverter, self.payload_int()?),
            ["set", "grid_freq_limit3_high"] => SetGridFreqLimit3High(inverter, self.payload_int()?),
            ["set", "grid_freq_limit3_low_time"] => SetGridFreqLimit3LowTime(inverter, self.payload_int()?),
            ["set", "grid_freq_limit3_high_time"] => SetGridFreqLimit3HighTime(inverter, self.payload_int()?),
            // Reactive Power Control (Registers 54-62)
            ["set", "max_q_percent_for_qv"] => SetMaxQPercentForQV(inverter, self.payload_int()?),
            ["set", "v1l"] => SetV1L(inverter, self.payload_int()?),
            ["set", "v2l"] => SetV2L(inverter, self.payload_int()?),
            ["set", "v1h"] => SetV1H(inverter, self.payload_int()?),
            ["set", "v2h"] => SetV2H(inverter, self.payload_int()?),
            ["set", "reactive_power_cmd_type"] => SetReactivePowerCMDType(inverter, self.payload_int()?),
            ["set", "active_power_percent_cmd"] => SetActivePowerPercentCMD(inverter, self.payload_int()?),
            ["set", "reactive_power_percent_cmd"] => SetReactivePowerPercentCMD(inverter, self.payload_int()?),
            ["set", "pf_cmd"] => SetPFCMD(inverter, self.payload_int()?),
            // Q(V) Curve Reference Parameters (Registers 185-186)
            ["set", "vref_qv"] => SetVrefQv(inverter, self.payload_int()?),
            ["set", "vref_filtertime"] => SetVrefFiltertime(inverter, self.payload_int()?),
            // Q(V) Curve Control Points (Registers 187-188)
            ["set", "q3_qv"] => SetQ3Qv(inverter, self.payload_int()?),
            ["set", "q4_qv"] => SetQ4Qv(inverter, self.payload_int()?),
            ["set", "q2_qv"] => SetQ2Qv(inverter, self.payload_int()?),
            // QP Priority Control Points (Registers 189-191)
            ["set", "p1_qp"] => SetP1Qp(inverter, self.payload_int()?),
            ["set", "p2_qp"] => SetP2Qp(inverter, self.payload_int()?),
            ["set", "p3_qp"] => SetP3Qp(inverter, self.payload_int()?),
            // Volt-Watt Open Loop Response Time (Register 183)
            ["set", "volt_watt_delay_time"] => SetVoltWattDelayTime(inverter, self.payload_int()?),
            // Generator Configuration
            ["set", "generator_cool_down_time"] => SetGeneratorCoolDownTime(inverter, self.payload_int()?),
            // AC Coupling Configuration
            ["set", "ac_coupling_enable"] => SetACCouplingEnable(inverter, self.payload_bool()),
            ["set", "ac_couple_start_soc"] => SetACCoupleStartSOC(inverter, self.payload_int()?),
            ["set", "ac_couple_end_soc"] => SetACCoupleEndSOC(inverter, self.payload_int()?),
            ["set", "ac_couple_start_volt"] => SetACCoupleStartVolt(inverter, self.payload_int()?),
            ["set", "ac_couple_end_volt"] => SetACCoupleEndVolt(inverter, self.payload_int()?),
            // Smart Load Configuration
            ["set", "smart_load_enable"] => SetSmartLoadEnable(inverter, self.payload_bool()),
            ["set", "grid_always_on"] => SetGridAlwaysOn(inverter, self.payload_bool()),
            ["set", "smart_load_start_volt"] => SetSmartLoadStartVolt(inverter, self.payload_int()?),
            ["set", "smart_load_end_volt"] => SetSmartLoadEndVolt(inverter, self.payload_int()?),
            ["set", "smart_load_start_soc"] => SetSmartLoadStartSOC(inverter, self.payload_int()?),
            ["set", "smart_load_end_soc"] => SetSmartLoadEndSOC(inverter, self.payload_int()?),
            ["set", "start_pv_power"] => SetStartPVPower(inverter, self.payload_int()?),
            // LCD Configuration
            ["set", "lcd_password"] => SetLCDPassword(inverter, self.payload_int()?),
            // Register 110 switches
            ["set", "pv_off_grid"] => PvOffGrid(inverter, self.payload_bool()),
            ["set", "fast_zero_export"] => FastZeroExport(inverter, self.payload_bool()),
            ["set", "micro_grid"] => MicroGrid(inverter, self.payload_bool()),
            ["set", "shared_battery"] => SharedBattery(inverter, self.payload_bool()),
            ["set", "charge_last"] => ChargeLast(inverter, self.payload_bool()),

            ["set", "eps"] => EPS(inverter, self.payload_bool()),
            ["set", "ovf_load_derate"] => OVFLoadDerate(inverter, self.payload_bool()),
            ["set", "frequency_active_open_loop_response_time"] => DelayTimeForOverFDerate(inverter, self.payload_ms()?),
            ["set", "ovf_derate_pct_per_hz"] => OVFDeratePctPerHz(inverter, self.payload_int()?),
            ["set", "ovf_derate_start_hz"] => OVFDerateStart(inverter, self.payload_frequency_hz()?),
            ["set", "ovf_derate_end_hz"] => OVFDerateEnd(inverter, self.payload_frequency_hz()?),
            ["set", "under_fr_increase_pct_per_hz"] => UnderFrIncreasePctPerHz(inverter, self.payload_int()?),
            ["set", "under_fr_droop_start_hz"] => UnderFrDroopStart(inverter, self.payload_frequency_hz()?),
            ["set", "under_fr_droop_end_hz"] => UnderFrDroopEnd(inverter, self.payload_frequency_hz()?),
            ["set", "drms"] => DRMS(inverter, self.payload_bool()),
            ["set", "lvrt"] => LVRT(inverter, self.payload_bool()),
            ["set", "anti_island"] => AntiIslanding(inverter, self.payload_bool()),
            ["set", "neutral_detect"] => NeutralDetect(inverter, self.payload_bool()),
            ["set", "grid_on_power_ss"] => GridOnPowerSS(inverter, self.payload_bool()),
            ["set", "sw_seamless"] => SwSeamless(inverter, self.payload_bool()),
            ["set", "set_to_standby"] => SetToStandby(inverter, self.payload_bool()),
            ["set", "iso"] => ISO(inverter, self.payload_bool()),
            ["set", "gfci"] => GFCI(inverter, self.payload_bool()),
            ["set", "dci"] => DCI(inverter, self.payload_bool()),
            ["set", "feed_in_grid"] => FeedInGrid(inverter, self.payload_bool()),
            ["restart"] => RestartInverter(inverter),

            [..] => bail!("unhandled: {:?}", self),
        };

        Ok(r)
    }

    // given a cmd Message, return the datalog it is intended for.
    //
    // eg cmd/AB12345678/set/ac_charge => (AB12345678, ['set', 'ac_charge'])
    pub fn split_cmd_topic(&self) -> Result<(TargetInverter, Vec<&str>)> {
        let parts: Vec<&str> = self.topic.split('/').collect();

        // bail if the topic is too short to handle.
        // this *shouldn't* happen as our subscribe is for lxp/cmd/{datalog}/#
        if parts.len() < 2 {
            bail!("ignoring badly formed MQTT topic: {}", self.topic);
        }

        // parts[0] should be cmd
        let datalog = parts[1];
        let rest = parts[2..].to_vec();

        if datalog == "all" {
            Ok((TargetInverter::All, rest))
        } else {
            let serial = Serial::from_str(datalog)?;
            Ok((TargetInverter::Serial(serial), rest))
        }
    }

    // not entirely happy with this return type but it avoids needing to expose a struct for now
    fn payload_start_end_time(&self) -> Result<[u8; 4]> {
        use serde::Deserialize;
        #[derive(Deserialize)]
        struct StartEndTime {
            start: String,
            end: String,
        }

        // {"start":"20:00", "end":"21:00"} -> [20, 0, 21, 0]
        let t = serde_json::from_str::<StartEndTime>(&self.payload)?;
        // split on : then make u8s to return in an array
        let start: Vec<&str> = t.start.split(':').collect();
        let end: Vec<&str> = t.end.split(':').collect();
        if start.len() != 2 || end.len() != 2 {
            bail!("badly formatted time, use HH:MM")
        }
        Ok([
            start[0].parse()?,
            start[1].parse()?,
            end[0].parse()?,
            end[1].parse()?,
        ])
    }

    fn payload_int_or_1(&self) -> Result<u16> {
        self.payload_int().or(Ok(1))
    }

    fn payload_int(&self) -> Result<u16> {
        if self.payload.is_empty() {
            bail!("payload_int: empty payload");
        }
        
        self.payload
            .trim()
            .parse()
            .map_err(|err| anyhow!("payload_int: cannot parse '{}' as integer: {}", self.payload, err))
    }

    fn payload_frequency_hz(&self) -> Result<u16> {
        if self.payload.is_empty() {
            bail!("payload_frequency_hz: empty payload");
        }
        
        // Parse as float first, then multiply by 100 and convert to integer
        // This handles cases where user enters 59.99 Hz -> sends 5999 to inverter
        let freq: f64 = self.payload
            .trim()
            .parse()
            .map_err(|err| anyhow!("payload_frequency_hz: cannot parse '{}' as frequency: {}", self.payload, err))?;
        Ok((freq * 100.0).round() as u16)
    }

    fn payload_ms(&self) -> Result<u16> {
        if self.payload.is_empty() {
            bail!("payload_ms: empty payload");
        }
        
        // Parse as float and round to nearest integer
        // This handles cases where user enters 61.99 ms -> sends 62 to inverter
        let ms: f64 = self.payload
            .trim()
            .parse()
            .map_err(|err| anyhow!("payload_ms: cannot parse '{}' as milliseconds: {}", self.payload, err))?;
        Ok(ms.round() as u16)
    }


    fn payload_bool(&self) -> bool {
        matches!(
            self.payload.to_ascii_lowercase().as_str(),
            "1" | "t" | "true" | "on" | "y" | "yes"
        )
    }
} // }}}

#[derive(Eq, PartialEq, Debug, Clone)]
pub enum ChannelData {
    Message(Message),
    Shutdown,
}

pub type Sender = broadcast::Sender<ChannelData>;

pub struct Mqtt {
    config: ConfigWrapper,
    shutdown: bool,
    channels: Channels,
}

impl Mqtt {
    pub fn new(config: ConfigWrapper, channels: Channels) -> Self {
        Self {
            config,
            channels,
            shutdown: false,
        }
    }

    pub async fn start(&self) -> Result<()> {
        let c = &self.config;

        if !c.mqtt().enabled() {
            info!("mqtt disabled, skipping");
            return Ok(());
        }

        let mut options = MqttOptions::new("lxp-bridge", c.mqtt().host(), c.mqtt().port());

        let will = LastWill {
            topic: self.lwt_topic(),
            message: bytes::Bytes::from("offline"),
            qos: QoS::AtLeastOnce,
            retain: true,
        };
        options.set_last_will(will);

        options.set_keep_alive(std::time::Duration::from_secs(60));
        if let (Some(u), Some(p)) = (c.mqtt().username(), c.mqtt().password()) {
            options.set_credentials(u, p);
        }

        info!(
            "initializing mqtt at {}:{}",
            c.mqtt().host(),
            c.mqtt().port()
        );

        let (client, eventloop) = AsyncClient::new(options, 10);

        futures::try_join!(
            self.setup(client.clone()),
            self.receiver(eventloop),
            self.sender(client)
        )?;

        Ok(())
    }

    pub fn stop(&mut self) {
        self.shutdown = true;

        let _ = self.channels.from_mqtt.send(ChannelData::Shutdown);
    }

    async fn setup(&self, client: AsyncClient) -> Result<()> {
        client
            .publish(self.lwt_topic(), QoS::AtLeastOnce, true, "online")
            .await?;

        client
            .subscribe(
                format!("{}/cmd/all/#", self.config.mqtt().namespace()),
                QoS::AtMostOnce,
            )
            .await?;

        for inverter in self.config.enabled_inverters() {
            client
                .subscribe(
                    format!(
                        "{}/cmd/{}/#",
                        self.config.mqtt().namespace(),
                        inverter.datalog()
                    ),
                    QoS::AtMostOnce,
                )
                .await?;

            if self.config.mqtt().homeassistant_enabled() {
                let ha = home_assistant::Config::new(&inverter, &self.config.mqtt());
                for msg in ha.all()?.into_iter() {
                    let _ = client
                        .publish(&msg.topic, QoS::AtLeastOnce, msg.retain, msg.payload)
                        .await;
                }
            }
        }

        Ok(())
    }

    // mqtt -> coordinator
    async fn receiver(&self, mut eventloop: EventLoop) -> Result<()> {
        loop {
            if self.shutdown {
                break;
            }

            if let Ok(event) =
                tokio::time::timeout(std::time::Duration::from_secs(1), eventloop.poll()).await
            {
                match event {
                    Ok(Event::Incoming(Incoming::Publish(publish))) => {
                        self.handle_message(publish)?;
                    }
                    Err(e) => {
                        // should automatically reconnect on next poll()..
                        error!("{}", e);
                        info!("reconnecting in 5s");
                        tokio::time::sleep(std::time::Duration::from_secs(5)).await;
                    }
                    _ => {} // keepalives etc
                }
            }
        }

        info!("receiver loop exiting");

        Ok(())
    }

    fn handle_message(&self, publish: Publish) -> Result<()> {
        // remove the namespace, including the first /
        // doing it this way means we don't break if namespace happens to contain a /
        let topic = publish.topic[self.config.mqtt().namespace().len() + 1..].to_owned();

        let message = Message {
            topic,
            retain: publish.retain,
            payload: String::from_utf8(publish.payload.to_vec())?,
        };
        debug!("RX: {:?}", message);
        if self
            .channels
            .from_mqtt
            .send(ChannelData::Message(message))
            .is_err()
        {
            bail!("send(from_mqtt) failed - channel closed?");
        }

        Ok(())
    }

    // coordinator -> mqtt
    async fn sender(&self, client: AsyncClient) -> Result<()> {
        use ChannelData::*;

        let mut receiver = self.channels.to_mqtt.subscribe();

        loop {
            match receiver.recv().await? {
                Shutdown => break,
                Message(message) => {
                    let topic = format!("{}/{}", self.config.mqtt().namespace(), message.topic);
                    info!("publishing: {} = {}", topic, message.payload);
                    let _ = client
                        .publish(&topic, QoS::AtLeastOnce, message.retain, message.payload)
                        .await
                        .map_err(|err| error!("publish {} failed: {:?} .. skipping", topic, err));
                }
            }
        }

        info!("sender loop exiting");

        Ok(())
    }

    fn lwt_topic(&self) -> String {
        format!("{}/LWT", self.config.mqtt().namespace())
    }
}
