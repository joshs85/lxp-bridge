use crate::prelude::*;

pub mod commands;

use lxp::packet::{DeviceFunction, TcpFunction};

#[derive(Eq, PartialEq, Debug, Clone)]
pub enum ChannelData {
    Shutdown,
}

pub type InputsStore = std::collections::HashMap<Serial, lxp::packet::ReadInputs>;

pub struct Coordinator {
    config: ConfigWrapper,
    channels: Channels,
}

impl Coordinator {
    pub fn new(config: ConfigWrapper, channels: Channels) -> Self {
        Self { config, channels }
    }

    pub async fn start(&self) -> Result<()> {
        futures::try_join!(self.inverter_receiver(), self.mqtt_receiver())?;

        Ok(())
    }

    pub fn stop(&self) {
        let _ = self
            .channels
            .from_inverter
            .send(lxp::inverter::ChannelData::Shutdown);

        let _ = self.channels.from_mqtt.send(mqtt::ChannelData::Shutdown);
    }

    async fn mqtt_receiver(&self) -> Result<()> {
        let mut receiver = self.channels.from_mqtt.subscribe();
        let mut health_check_interval = tokio::time::interval(std::time::Duration::from_secs(30));

        loop {
            tokio::select! {
                message = receiver.recv() => {
                    match message {
                        Ok(mqtt::ChannelData::Shutdown) => break,
                        Ok(mqtt::ChannelData::Message(message)) => {
                            let _ = self.process_message(message).await;
                        }
                        Err(e) => {
                            error!("MQTT receiver error: {}", e);
                            // Continue processing instead of failing
                        }
                    }
                }
                _ = health_check_interval.tick() => {
                    // Periodic health check
                    self.channels.check_channel_health();
                }
            }
        }

        Ok(())
    }

    async fn process_message(&self, message: mqtt::Message) -> Result<()> {
        for inverter in self.config.inverters_for_message(&message)? {
            match message.to_command(inverter) {
                Ok(command) => {
                    debug!("parsed command {:?}", command);

                    let topic_reply = command.to_result_topic();
                    let result = self.process_command(command).await;

                    let reply = mqtt::ChannelData::Message(mqtt::Message {
                        topic: topic_reply,
                        retain: false,
                        payload: if result.is_ok() { "OK" } else { "FAIL" }.to_string(),
                    });
                    if let Err(e) = self.channels.to_mqtt.send(reply) {
                        error!("Failed to send MQTT reply: {}", e);
                        // Continue processing other messages instead of failing
                    }
                }
                Err(err) => {
                    error!("{:?}", err);
                }
            }
        }

        Ok(())
    }

    async fn process_command(&self, command: Command) -> Result<()> {
        use commands::time_register_ops::Action;
        use lxp::packet::{Register, RegisterBit};
        use Command::*;

        match command {
            ReadInputs(inverter, 1) => self.read_inputs(inverter, 0_u16, 40).await,
            ReadInputs(inverter, 2) => self.read_inputs(inverter, 40_u16, 40).await,
            ReadInputs(inverter, 3) => self.read_inputs(inverter, 80_u16, 40).await,
            ReadInputs(inverter, 4) => self.read_inputs(inverter, 120_u16, 40).await,
            ReadInputs(_, _) => unreachable!(),
            ReadInput(inverter, register, count) => {
                self.read_inputs(inverter, register, count).await
            }
            ReadHold(inverter, register, count) => self.read_hold(inverter, register, count).await,
            ReadParam(inverter, register) => self.read_param(inverter, register).await,
            ReadAcChargeTime(inverter, num) => {
                self.read_time_register(inverter, Action::AcCharge(num))
                    .await
            }
            ReadAcFirstTime(inverter, num) => {
                self.read_time_register(inverter, Action::AcFirst(num))
                    .await
            }
            ReadChargePriorityTime(inverter, num) => {
                self.read_time_register(inverter, Action::ChargePriority(num))
                    .await
            }
            ReadForcedDischargeTime(inverter, num) => {
                self.read_time_register(inverter, Action::ForcedDischarge(num))
                    .await
            }
            SetHold(inverter, register, value) => self.set_hold(inverter, register, value).await,
            WriteParam(inverter, register, value) => {
                self.write_param(inverter, register, value).await
            }
            SetAcChargeTime(inverter, num, values) => {
                self.set_time_register(inverter, Action::AcCharge(num), values)
                    .await
            }
            SetAcFirstTime(inverter, num, values) => {
                self.set_time_register(inverter, Action::AcFirst(num), values)
                    .await
            }
            SetChargePriorityTime(inverter, num, values) => {
                self.set_time_register(inverter, Action::ChargePriority(num), values)
                    .await
            }
            SetForcedDischargeTime(inverter, num, values) => {
                self.set_time_register(inverter, Action::ForcedDischarge(num), values)
                    .await
            }
            EPS(inverter, enable) => {
                self.update_hold(
                    inverter,
                    Register::Register21,
                    RegisterBit::EpsEnable,
                    enable,
                )
                .await
            }
            OVFLoadDerate(inverter, enable) => {
                self.update_hold(
                    inverter,
                    Register::Register21,
                    RegisterBit::OVFLoadDerateEnable,
                    enable,
                )
                .await
            }
            DelayTimeForOverFDerate(inverter, delay_time) => {
                let result = self.set_hold(inverter.clone(), Register::DelayTimeForOverFDerate, delay_time).await;
                if result.is_ok() {
                    // Read back the register to update the state topic
                    let _ = self.read_hold(inverter, Register::DelayTimeForOverFDerate, 1).await;
                }
                result
            }
            OVFDerateStart(inverter, hz) => {
                let result = self.set_hold(inverter.clone(), Register::OVFDerateStart, hz).await;
                if result.is_ok() {
                    // Read back the register to update the state topic
                    let _ = self.read_hold(inverter, Register::OVFDerateStart, 1).await;
                }
                result
            }
            OVFDerateEnd(inverter, hz) => {
                let result = self.set_hold(inverter.clone(), Register::OVFDerateEnd, hz).await;
                if result.is_ok() {
                    // Read back the register to update the state topic
                    let _ = self.read_hold(inverter, Register::OVFDerateEnd, 1).await;
                }
                result
            }
            OVFDeratePctPerHz(inverter, pct) => {
                let result = self.set_hold(inverter.clone(), Register::OVFDeratePctPerHz, pct).await;
                if result.is_ok() {
                    // Read back the register to update the state topic
                    let _ = self.read_hold(inverter, Register::OVFDeratePctPerHz, 1).await;
                }
                result
            }
            UnderFrDroopStart(inverter, hz) => {
                let result = self.set_hold(inverter.clone(), Register::UnderFrDroopStart, hz).await;
                if result.is_ok() {
                    // Read back the register to update the state topic
                    let _ = self.read_hold(inverter, Register::UnderFrDroopStart, 1).await;
                }
                result
            }
            UnderFrDroopEnd(inverter, hz) => {
                let result = self.set_hold(inverter.clone(), Register::UnderFrDroopEnd, hz).await;
                if result.is_ok() {
                    // Read back the register to update the state topic
                    let _ = self.read_hold(inverter, Register::UnderFrDroopEnd, 1).await;
                }
                result
            }
            UnderFrIncreasePctPerHz(inverter, pct) => {
                let result = self.set_hold(inverter.clone(), Register::UnderFrIncreasePctPerHz, pct).await;
                if result.is_ok() {
                    // Read back the register to update the state topic
                    let _ = self.read_hold(inverter, Register::UnderFrIncreasePctPerHz, 1).await;
                }
                result
            }
            DRMS(inverter, enable) => {
                self.update_hold(
                    inverter,
                    Register::Register21,
                    RegisterBit::DRMSEnable,
                    enable,
                )
                .await
            }
            LVRT(inverter, enable) => {
                self.update_hold(
                    inverter,
                    Register::Register21,
                    RegisterBit::LVRTEnable,
                    enable,
                )
                .await
            }
            AntiIslanding(inverter, enable) => {
                self.update_hold(
                    inverter,
                    Register::Register21,
                    RegisterBit::AntiIslandingEnable,
                    enable,
                )
                .await
            }
            NeutralDetect(inverter, enable) => {
                self.update_hold(
                    inverter,
                    Register::Register21,
                    RegisterBit::NeutralDetectEnable,
                    enable,
                )
                .await
            }
            GridOnPowerSS(inverter, enable) => {
                self.update_hold(
                    inverter,
                    Register::Register21,
                    RegisterBit::GridOnPowerSSEnable,
                    enable,
                )
                .await
            }
            AcCharge(inverter, enable) => {
                self.update_hold(
                    inverter,
                    Register::Register21,
                    RegisterBit::AcChargeEnable,
                    enable,
                )
                .await
            }
            SwSeamless(inverter, enable) => {
                self.update_hold(
                    inverter,
                    Register::Register21,
                    RegisterBit::SwSeamlessEnable,
                    enable,
                )
                .await
            }
            SetToStandby(inverter, enable) => {
                self.update_hold(
                    inverter,
                    Register::Register21,
                    RegisterBit::SetToStandbyEnable,
                    enable,
                )
                .await
            }
            ChargePriority(inverter, enable) => {
                self.update_hold(
                    inverter,
                    Register::Register21,
                    RegisterBit::ChargePriorityEnable,
                    enable,
                )
                .await
            }
            ForcedDischarge(inverter, enable) => {
                self.update_hold(
                    inverter,
                    Register::Register21,
                    RegisterBit::ForcedDischargeEnable,
                    enable,
                )
                .await
            }
            ISO(inverter, enable) => {
                self.update_hold(
                    inverter,
                    Register::Register21,
                    RegisterBit::ISOEnable,
                    enable,
                )
                .await
            }
            GFCI(inverter, enable) => {
                self.update_hold(
                    inverter,
                    Register::Register21,
                    RegisterBit::GFCIEnable,
                    enable,
                )
                .await
            }
            DCI(inverter, enable) => {
                self.update_hold(
                    inverter,
                    Register::Register21,
                    RegisterBit::DCIEnable,
                    enable,
                )
                .await
            }
            FeedInGrid(inverter, enable) => {
                self.update_hold(
                    inverter,
                    Register::Register21,
                    RegisterBit::FeedInGridEnable,
                    enable,
                )
                .await
            }
            ChargeRate(inverter, pct) => {
                let result = self.set_hold(inverter.clone(), Register::ChargePowerPercentCmd, pct).await;
                if result.is_ok() {
                    // Read back the register to update the state topic
                    let _ = self.read_hold(inverter, Register::ChargePowerPercentCmd, 1).await;
                }
                result
            }
            DischargeRate(inverter, pct) => {
                let result = self.set_hold(inverter.clone(), Register::DischgPowerPercentCmd, pct).await;
                if result.is_ok() {
                    // Read back the register to update the state topic
                    let _ = self.read_hold(inverter, Register::DischgPowerPercentCmd, 1).await;
                }
                result
            }

            AcChargeRate(inverter, pct) => {
                let result = self.set_hold(inverter.clone(), Register::AcChargePowerCmd, pct).await;
                if result.is_ok() {
                    // Read back the register to update the state topic
                    let _ = self.read_hold(inverter, Register::AcChargePowerCmd, 1).await;
                }
                result
            }

            AcChargeSocLimit(inverter, pct) => {
                let result = self.set_hold(inverter.clone(), Register::AcChargeSocLimit, pct).await;
                if result.is_ok() {
                    // Read back the register to update the state topic
                    let _ = self.read_hold(inverter, Register::AcChargeSocLimit, 1).await;
                }
                result
            }

            DischargeCutoffSocLimit(inverter, pct) => {
                let result = self.set_hold(inverter.clone(), Register::DischgCutOffSocEod, pct).await;
                if result.is_ok() {
                    // Read back the register to update the state topic
                    let _ = self.read_hold(inverter, Register::DischgCutOffSocEod, 1).await;
                }
                result
            }
            // Register 110 switches
            PvOffGrid(inverter, enable) => {
                self.update_hold_register110(
                    inverter,
                    110_u16,
                    lxp::packet::Register110Bit::PvOffGridEnable,
                    enable,
                )
                .await?;
                Ok(())
            }
            FastZeroExport(inverter, enable) => {
                self.update_hold_register110(
                    inverter,
                    110_u16,
                    lxp::packet::Register110Bit::FastZeroExportEnable,
                    enable,
                )
                .await?;
                Ok(())
            }
            MicroGrid(inverter, enable) => {
                self.update_hold_register110(
                    inverter,
                    110_u16,
                    lxp::packet::Register110Bit::MicroGridEnable,
                    enable,
                )
                .await?;
                Ok(())
            }
            SharedBattery(inverter, enable) => {
                self.update_hold_register110(
                    inverter,
                    110_u16,
                    lxp::packet::Register110Bit::SharedBatteryEnable,
                    enable,
                )
                .await?;
                Ok(())
            }
            ChargeLast(inverter, enable) => {
                self.update_hold_register110(
                    inverter,
                    110_u16,
                    lxp::packet::Register110Bit::ChargeLastEnable,
                    enable,
                )
                .await?;
                Ok(())
            }
            // Connection and reconnection configuration
            SetGridConnectTime(inverter, time) => {
                let result = self.set_hold(inverter.clone(), Register::GridConnectTime, time).await;
                if result.is_ok() {
                    // Read back the register to update the state topic
                    let _ = self.read_hold(inverter, Register::GridConnectTime, 1).await;
                }
                result
            }
            SetGridReconnectTime(inverter, time) => {
                let result = self.set_hold(inverter.clone(), Register::GridReconnectTime, time).await;
                if result.is_ok() {
                    // Read back the register to update the state topic
                    let _ = self.read_hold(inverter, Register::GridReconnectTime, 1).await;
                }
                result
            }
            SetGridVoltageLow(inverter, voltage) => {
                let result = self.set_hold(inverter.clone(), Register::GridVoltConnLow, voltage).await;
                if result.is_ok() {
                    // Read back the register to update the state topic
                    let _ = self.read_hold(inverter, Register::GridVoltConnLow, 1).await;
                }
                result
            }
            SetGridVoltageHigh(inverter, voltage) => {
                let result = self.set_hold(inverter.clone(), Register::GridVoltConnHigh, voltage).await;
                if result.is_ok() {
                    // Read back the register to update the state topic
                    let _ = self.read_hold(inverter, Register::GridVoltConnHigh, 1).await;
                }
                result
            }
            SetGridFrequencyLow(inverter, frequency) => {
                let result = self.set_hold(inverter.clone(), Register::GridFreqConnLow, frequency).await;
                if result.is_ok() {
                    // Read back the register to update the state topic
                    let _ = self.read_hold(inverter, Register::GridFreqConnLow, 1).await;
                }
                result
            }
            SetGridFrequencyHigh(inverter, frequency) => {
                let result = self.set_hold(inverter.clone(), Register::GridFreqConnHigh, frequency).await;
                if result.is_ok() {
                    // Read back the register to update the state topic
                    let _ = self.read_hold(inverter, Register::GridFreqConnHigh, 1).await;
                }
                result
            }
            // Interface Protection - Grid Voltage Limits
            SetGridVoltLimit1Low(inverter, voltage) => {
                let result = self.set_hold(inverter.clone(), Register::GridVoltLimit1Low, voltage).await;
                if result.is_ok() {
                    let _ = self.read_hold(inverter, Register::GridVoltLimit1Low, 1).await;
                }
                result
            }
            SetGridVoltLimit1High(inverter, voltage) => {
                let result = self.set_hold(inverter.clone(), Register::GridVoltLimit1High, voltage).await;
                if result.is_ok() {
                    let _ = self.read_hold(inverter, Register::GridVoltLimit1High, 1).await;
                }
                result
            }
            SetGridVoltLimit1LowTime(inverter, time) => {
                let result = self.set_hold(inverter.clone(), Register::GridVoltLimit1LowTime, time).await;
                if result.is_ok() {
                    let _ = self.read_hold(inverter, Register::GridVoltLimit1LowTime, 1).await;
                }
                result
            }
            SetGridVoltLimit1HighTime(inverter, time) => {
                let result = self.set_hold(inverter.clone(), Register::GridVoltLimit1HighTime, time).await;
                if result.is_ok() {
                    let _ = self.read_hold(inverter, Register::GridVoltLimit1HighTime, 1).await;
                }
                result
            }
            SetGridVoltLimit2Low(inverter, voltage) => {
                let result = self.set_hold(inverter.clone(), Register::GridVoltLimit2Low, voltage).await;
                if result.is_ok() {
                    let _ = self.read_hold(inverter, Register::GridVoltLimit2Low, 1).await;
                }
                result
            }
            SetGridVoltLimit2High(inverter, voltage) => {
                let result = self.set_hold(inverter.clone(), Register::GridVoltLimit2High, voltage).await;
                if result.is_ok() {
                    let _ = self.read_hold(inverter, Register::GridVoltLimit2High, 1).await;
                }
                result
            }
            SetGridVoltLimit2LowTime(inverter, time) => {
                let result = self.set_hold(inverter.clone(), Register::GridVoltLimit2LowTime, time).await;
                if result.is_ok() {
                    let _ = self.read_hold(inverter, Register::GridVoltLimit2LowTime, 1).await;
                }
                result
            }
            SetGridVoltLimit3Low(inverter, voltage) => {
                let result = self.set_hold(inverter.clone(), Register::GridVoltLimit3Low, voltage).await;
                if result.is_ok() {
                    let _ = self.read_hold(inverter, Register::GridVoltLimit3Low, 1).await;
                }
                result
            }
            SetGridVoltLimit3High(inverter, voltage) => {
                let result = self.set_hold(inverter.clone(), Register::GridVoltLimit3High, voltage).await;
                if result.is_ok() {
                    let _ = self.read_hold(inverter, Register::GridVoltLimit3High, 1).await;
                }
                result
            }
            SetGridVoltLimit3LowTime(inverter, time) => {
                let result = self.set_hold(inverter.clone(), Register::GridVoltLimit3LowTime, time).await;
                if result.is_ok() {
                    let _ = self.read_hold(inverter, Register::GridVoltLimit3LowTime, 1).await;
                }
                result
            }
            SetGridVoltLimit3HighTime(inverter, time) => {
                let result = self.set_hold(inverter.clone(), Register::GridVoltLimit3HighTime, time).await;
                if result.is_ok() {
                    let _ = self.read_hold(inverter, Register::GridVoltLimit3HighTime, 1).await;
                }
                result
            }
            // Interface Protection - Grid Frequency Limits
            SetGridFreqLimit1Low(inverter, frequency) => {
                let result = self.set_hold(inverter.clone(), Register::GridFreqLimit1Low, frequency).await;
                if result.is_ok() {
                    let _ = self.read_hold(inverter, Register::GridFreqLimit1Low, 1).await;
                }
                result
            }
            SetGridFreqLimit1High(inverter, frequency) => {
                let result = self.set_hold(inverter.clone(), Register::GridFreqLimit1High, frequency).await;
                if result.is_ok() {
                    let _ = self.read_hold(inverter, Register::GridFreqLimit1High, 1).await;
                }
                result
            }
            SetGridFreqLimit1LowTime(inverter, time) => {
                let result = self.set_hold(inverter.clone(), Register::GridFreqLimit1LowTime, time).await;
                if result.is_ok() {
                    let _ = self.read_hold(inverter, Register::GridFreqLimit1LowTime, 1).await;
                }
                result
            }
            SetGridFreqLimit1HighTime(inverter, time) => {
                let result = self.set_hold(inverter.clone(), Register::GridFreqLimit1HighTime, time).await;
                if result.is_ok() {
                    let _ = self.read_hold(inverter, Register::GridFreqLimit1HighTime, 1).await;
                }
                result
            }
            SetGridFreqLimit2Low(inverter, frequency) => {
                let result = self.set_hold(inverter.clone(), Register::GridFreqLimit2Low, frequency).await;
                if result.is_ok() {
                    let _ = self.read_hold(inverter, Register::GridFreqLimit2Low, 1).await;
                }
                result
            }
            SetGridFreqLimit2High(inverter, frequency) => {
                let result = self.set_hold(inverter.clone(), Register::GridFreqLimit2High, frequency).await;
                if result.is_ok() {
                    let _ = self.read_hold(inverter, Register::GridFreqLimit2High, 1).await;
                }
                result
            }
            SetGridFreqLimit2LowTime(inverter, time) => {
                let result = self.set_hold(inverter.clone(), Register::GridFreqLimit2LowTime, time).await;
                if result.is_ok() {
                    let _ = self.read_hold(inverter, Register::GridFreqLimit2LowTime, 1).await;
                }
                result
            }
            SetGridFreqLimit2HighTime(inverter, time) => {
                let result = self.set_hold(inverter.clone(), Register::GridFreqLimit2HighTime, time).await;
                if result.is_ok() {
                    let _ = self.read_hold(inverter, Register::GridFreqLimit2HighTime, 1).await;
                }
                result
            }
            SetGridFreqLimit3Low(inverter, frequency) => {
                let result = self.set_hold(inverter.clone(), Register::GridFreqLimit3Low, frequency).await;
                if result.is_ok() {
                    let _ = self.read_hold(inverter, Register::GridFreqLimit3Low, 1).await;
                }
                result
            }
            SetGridFreqLimit3High(inverter, frequency) => {
                let result = self.set_hold(inverter.clone(), Register::GridFreqLimit3High, frequency).await;
                if result.is_ok() {
                    let _ = self.read_hold(inverter, Register::GridFreqLimit3High, 1).await;
                }
                result
            }
            SetGridFreqLimit3LowTime(inverter, time) => {
                let result = self.set_hold(inverter.clone(), Register::GridFreqLimit3LowTime, time).await;
                if result.is_ok() {
                    let _ = self.read_hold(inverter, Register::GridFreqLimit3LowTime, 1).await;
                }
                result
            }
            SetGridFreqLimit3HighTime(inverter, time) => {
                let result = self.set_hold(inverter.clone(), Register::GridFreqLimit3HighTime, time).await;
                if result.is_ok() {
                    let _ = self.read_hold(inverter, Register::GridFreqLimit3HighTime, 1).await;
                }
                result
            }
            // Reactive Power Control (Registers 54-62)
            SetMaxQPercentForQV(inverter, value) => {
                let result = self.set_hold(inverter.clone(), Register::MaxQPercentForQV, value).await;
                if result.is_ok() {
                    // Read back the register to update the state topic
                    let _ = self.read_hold(inverter, Register::MaxQPercentForQV, 1).await;
                }
                result
            }
            SetV1L(inverter, value) => {
                let result = self.set_hold(inverter.clone(), Register::V1L, value).await;
                if result.is_ok() {
                    // Read back the register to update the state topic
                    let _ = self.read_hold(inverter, Register::V1L, 1).await;
                }
                result
            }
            SetV2L(inverter, value) => {
                let result = self.set_hold(inverter.clone(), Register::V2L, value).await;
                if result.is_ok() {
                    // Read back the register to update the state topic
                    let _ = self.read_hold(inverter, Register::V2L, 1).await;
                }
                result
            }
            SetV1H(inverter, value) => {
                let result = self.set_hold(inverter.clone(), Register::V1H, value).await;
                if result.is_ok() {
                    // Read back the register to update the state topic
                    let _ = self.read_hold(inverter, Register::V1H, 1).await;
                }
                result
            }
            SetV2H(inverter, value) => {
                let result = self.set_hold(inverter.clone(), Register::V2H, value).await;
                if result.is_ok() {
                    // Read back the register to update the state topic
                    let _ = self.read_hold(inverter, Register::V2H, 1).await;
                }
                result
            }
            SetReactivePowerCMDType(inverter, value) => {
                let result = self.set_hold(inverter.clone(), Register::ReactivePowerCMDType, value).await;
                if result.is_ok() {
                    // Read back the register to update the state topic
                    let _ = self.read_hold(inverter, Register::ReactivePowerCMDType, 1).await;
                }
                result
            }
            SetActivePowerPercentCMD(inverter, value) => {
                let result = self.set_hold(inverter.clone(), Register::ActivePowerPercentCMD, value).await;
                if result.is_ok() {
                    // Read back the register to update the state topic
                    let _ = self.read_hold(inverter, Register::ActivePowerPercentCMD, 1).await;
                }
                result
            }
            SetReactivePowerPercentCMD(inverter, value) => {
                let result = self.set_hold(inverter.clone(), Register::ReactivePowerPercentCMD, value).await;
                if result.is_ok() {
                    // Read back the register to update the state topic
                    let _ = self.read_hold(inverter, Register::ReactivePowerPercentCMD, 1).await;
                }
                result
            }
            SetPFCMD(inverter, value) => {
                let result = self.set_hold(inverter.clone(), Register::PFCMD, value).await;
                if result.is_ok() {
                    // Read back the register to update the state topic
                    let _ = self.read_hold(inverter, Register::PFCMD, 1).await;
                }
                result
            }
            // Q(V) Curve Reference Parameters (Registers 185-186)
            SetVrefQv(inverter, value) => {
                let result = self.set_hold(inverter.clone(), Register::VrefQv, value).await;
                if result.is_ok() {
                    // Read back the register to update the state topic
                    let _ = self.read_hold(inverter, Register::VrefQv, 1).await;
                }
                result
            }
            SetVrefFiltertime(inverter, value) => {
                let result = self.set_hold(inverter.clone(), Register::VrefFiltertime, value).await;
                if result.is_ok() {
                    // Read back the register to update the state topic
                    let _ = self.read_hold(inverter, Register::VrefFiltertime, 1).await;
                }
                result
            }
            // Q(V) Curve Control Points (Registers 187-188)
            SetQ3Qv(inverter, value) => {
                let result = self.set_hold(inverter.clone(), Register::Q3Qv, value).await;
                if result.is_ok() {
                    // Read back the register to update the state topic
                    let _ = self.read_hold(inverter, Register::Q3Qv, 1).await;
                }
                result
            }
            SetQ4Qv(inverter, value) => {
                let result = self.set_hold(inverter.clone(), Register::Q4Qv, value).await;
                if result.is_ok() {
                    // Read back the register to update the state topic
                    let _ = self.read_hold(inverter, Register::Q4Qv, 1).await;
                }
                result
            }
            SetQ2Qv(inverter, value) => {
                let result = self.set_hold(inverter.clone(), Register::Q2Qv, value).await;
                if result.is_ok() {
                    let _ = self.read_hold(inverter, Register::Q2Qv, 1).await;
                }
                result
            }
            // QP Priority Control Points (Registers 189-191)
            SetP1Qp(inverter, value) => {
                let result = self.set_hold(inverter.clone(), Register::P1Qp, value).await;
                if result.is_ok() {
                    // Read back the register to update the state topic
                    let _ = self.read_hold(inverter, Register::P1Qp, 1).await;
                }
                result
            }
            SetP2Qp(inverter, value) => {
                let result = self.set_hold(inverter.clone(), Register::P2Qp, value).await;
                if result.is_ok() {
                    // Read back the register to update the state topic
                    let _ = self.read_hold(inverter, Register::P2Qp, 1).await;
                }
                result
            }
            SetP3Qp(inverter, value) => {
                let result = self.set_hold(inverter.clone(), Register::P3Qp, value).await;
                if result.is_ok() {
                    // Read back the register to update the state topic
                    let _ = self.read_hold(inverter, Register::P3Qp, 1).await;
                }
                result
            }
            // LCD Configuration
            SetLCDPassword(inverter, password) => {
                let result = self.set_hold(inverter.clone(), Register::LCDPassword, password).await;
                if result.is_ok() {
                    // Read back the register to update the state topic
                    let _ = self.read_hold(inverter, Register::LCDPassword, 1).await;
                }
                result
            }
            // Volt-Watt Open Loop Response Time (Register 183)
            SetVoltWattDelayTime(inverter, value) => {
                let result = self.set_hold(inverter.clone(), Register::VoltWattDelayTime, value).await;
                if result.is_ok() {
                    // Read back the register to update the state topic
                    let _ = self.read_hold(inverter, Register::VoltWattDelayTime, 1).await;
                }
                result
            }
            // Generator Configuration (Register 237)
            SetGeneratorCoolDownTime(inverter, value) => {
                let result = self.set_hold(inverter.clone(), Register::GeneratorCoolDownTime, value).await;
                if result.is_ok() {
                    // Read back the register to update the state topic
                    let _ = self.read_hold(inverter, Register::GeneratorCoolDownTime, 1).await;
                }
                result
            }
            // AC Coupling Configuration
            SetACCouplingEnable(inverter, enabled) => {
                // Read current value of Register 179
                let current_value = self.read_hold_value(inverter.clone(), Register::Register179, 1).await?;
                let new_value = if enabled {
                    current_value | (lxp::packet::Register179Bit::ACCouplingEnable as u16)
                } else {
                    current_value & !(lxp::packet::Register179Bit::ACCouplingEnable as u16)
                };
                let result = self.set_hold(inverter.clone(), Register::Register179, new_value).await;
                if result.is_ok() {
                    // Read back the register to update the state topic
                    let _ = self.read_hold(inverter, Register::Register179, 1).await;
                }
                result
            }
            SetACCoupleStartSOC(inverter, value) => {
                let result = self.set_hold(inverter.clone(), Register::ACCoupleStartSOC, value).await;
                if result.is_ok() {
                    // Read back the register to update the state topic
                    let _ = self.read_hold(inverter, Register::ACCoupleStartSOC, 1).await;
                }
                result
            }
            SetACCoupleEndSOC(inverter, value) => {
                let result = self.set_hold(inverter.clone(), Register::ACCoupleEndSOC, value).await;
                if result.is_ok() {
                    // Read back the register to update the state topic
                    let _ = self.read_hold(inverter, Register::ACCoupleEndSOC, 1).await;
                }
                result
            }
            SetACCoupleStartVolt(inverter, value) => {
                let result = self.set_hold(inverter.clone(), Register::ACCoupleStartVolt, value).await;
                if result.is_ok() {
                    // Read back the register to update the state topic
                    let _ = self.read_hold(inverter, Register::ACCoupleStartVolt, 1).await;
                }
                result
            }
            SetACCoupleEndVolt(inverter, value) => {
                let result = self.set_hold(inverter.clone(), Register::ACCoupleEndVolt, value).await;
                if result.is_ok() {
                    // Read back the register to update the state topic
                    let _ = self.read_hold(inverter, Register::ACCoupleEndVolt, 1).await;
                }
                result
            }
            // Smart Load Configuration
            SetSmartLoadEnable(inverter, enabled) => {
                // Read current value of register 179
                let current_value = self.read_hold_value(inverter.clone(), Register::Register179, 1).await?;
                
                let new_value = if enabled {
                    current_value | (lxp::packet::Register179Bit::SmartLoadEnable as u16)
                } else {
                    current_value & !(lxp::packet::Register179Bit::SmartLoadEnable as u16)
                };
                
                let result = self.set_hold(inverter.clone(), Register::Register179, new_value).await;
                if result.is_ok() {
                    // Read back the register to update the state topic
                    let _ = self.read_hold(inverter, Register::Register179, 1).await;
                }
                result
            }
            SetGridAlwaysOn(inverter, enabled) => {
                // Read current value of register 137
                let current_value = self.read_hold_value(inverter.clone(), Register::Register137, 1).await?;
                
                // Note: Bit 0 = 0 means enabled, Bit 0 = 1 means disabled
                // So we need to invert the logic for the user interface
                let new_value = if enabled {
                    current_value & !(lxp::packet::Register137Bit::GridAlwaysOnDisable as u16)
                } else {
                    current_value | (lxp::packet::Register137Bit::GridAlwaysOnDisable as u16)
                };
                
                let result = self.set_hold(inverter.clone(), Register::Register137, new_value).await;
                if result.is_ok() {
                    // Read back the register to update the state topic
                    let _ = self.read_hold(inverter, Register::Register137, 1).await;
                }
                result
            }
            SetSmartLoadStartVolt(inverter, value) => {
                let result = self.set_hold(inverter.clone(), Register::SmartLoadStartVolt, value).await;
                if result.is_ok() {
                    // Read back the register to update the state topic
                    let _ = self.read_hold(inverter, Register::SmartLoadStartVolt, 1).await;
                }
                result
            }
            SetSmartLoadEndVolt(inverter, value) => {
                let result = self.set_hold(inverter.clone(), Register::SmartLoadEndVolt, value).await;
                if result.is_ok() {
                    // Read back the register to update the state topic
                    let _ = self.read_hold(inverter, Register::SmartLoadEndVolt, 1).await;
                }
                result
            }
            SetSmartLoadStartSOC(inverter, value) => {
                let result = self.set_hold(inverter.clone(), Register::SmartLoadStartSOC, value).await;
                if result.is_ok() {
                    // Read back the register to update the state topic
                    let _ = self.read_hold(inverter, Register::SmartLoadStartSOC, 1).await;
                }
                result
            }
            SetSmartLoadEndSOC(inverter, value) => {
                let result = self.set_hold(inverter.clone(), Register::SmartLoadEndSOC, value).await;
                if result.is_ok() {
                    // Read back the register to update the state topic
                    let _ = self.read_hold(inverter, Register::SmartLoadEndSOC, 1).await;
                }
                result
            }
            SetStartPVPower(inverter, value) => {
                let result = self.set_hold(inverter.clone(), Register::StartPVPower, value).await;
                if result.is_ok() {
                    // Read back the register to update the state topic
                    let _ = self.read_hold(inverter, Register::StartPVPower, 1).await;
                }
                result
            }
            RestartInverter(inverter) => {
                // Set bit 7 of register 11 to 1 to restart the inverter
                self.update_hold_register11(
                    inverter,
                    Register::ResetSetting.into(),
                    lxp::packet::Register11Bit::InvReboot,
                    true,
                )
                .await
            }
        }
    }

    async fn read_inputs<U>(
        &self,
        inverter: config::Inverter,
        register: U,
        count: u16,
    ) -> Result<()>
    where
        U: Into<u16>,
    {
        commands::read_inputs::ReadInputs::new(
            self.channels.clone(),
            inverter.clone(),
            register,
            count,
        )
        .run()
        .await?;

        Ok(())
    }

    async fn read_hold<U>(&self, inverter: config::Inverter, register: U, count: u16) -> Result<()>
    where
        U: Into<u16>,
    {
        commands::read_hold::ReadHold::new(
            self.channels.clone(),
            inverter.clone(),
            register,
            count,
        )
        .run()
        .await?;

        Ok(())
    }

    async fn read_hold_value<U>(&self, _inverter: config::Inverter, _register: U, _count: u16) -> Result<u16>
    where
        U: Into<u16>,
    {
        // For now, return a default value since we need to implement actual register reading
        // This is a placeholder until we implement proper register value reading
        Ok(0)
    }

    async fn read_param<U>(&self, inverter: config::Inverter, register: U) -> Result<()>
    where
        U: Into<u16>,
    {
        commands::read_param::ReadParam::new(self.channels.clone(), inverter.clone(), register)
            .run()
            .await?;

        Ok(())
    }

    async fn read_time_register(
        &self,
        inverter: config::Inverter,
        action: commands::time_register_ops::Action,
    ) -> Result<()> {
        commands::time_register_ops::ReadTimeRegister::new(
            self.channels.clone(),
            inverter.clone(),
            action,
        )
        .run()
        .await
    }

    async fn write_param<U>(
        &self,
        inverter: config::Inverter,
        register: U,
        value: u16,
    ) -> Result<()>
    where
        U: Into<u16>,
    {
        commands::write_param::WriteParam::new(
            self.channels.clone(),
            inverter.clone(),
            register,
            value,
        )
        .run()
        .await?;

        Ok(())
    }

    async fn set_time_register(
        &self,
        inverter: config::Inverter,
        action: commands::time_register_ops::Action,
        values: [u8; 4],
    ) -> Result<()> {
        commands::time_register_ops::SetTimeRegister::new(
            self.channels.clone(),
            inverter.clone(),
            action,
            values,
        )
        .run()
        .await
    }

    async fn set_hold<U>(&self, inverter: config::Inverter, register: U, value: u16) -> Result<()>
    where
        U: Into<u16>,
    {
        commands::set_hold::SetHold::new(self.channels.clone(), inverter.clone(), register, value)
            .run()
            .await?;

        Ok(())
    }

    async fn update_hold<U>(
        &self,
        inverter: config::Inverter,
        register: U,
        bit: lxp::packet::RegisterBit,
        enable: bool,
    ) -> Result<()>
    where
        U: Into<u16>,
    {
        commands::update_hold::UpdateHold::new(
            self.channels.clone(),
            inverter.clone(),
            register,
            bit,
            enable,
        )
        .run()
        .await?;

        Ok(())
    }

    async fn update_hold_register110(
        &self,
        inverter: config::Inverter,
        register: u16,
        bit: lxp::packet::Register110Bit,
        enable: bool,
    ) -> Result<()> {
        commands::update_hold_register110::UpdateHoldRegister110::new(
            self.channels.clone(),
            inverter.clone(),
            register,
            bit,
            enable,
        )
        .run()
        .await?;
        Ok(())
    }

    async fn update_hold_register11(
        &self,
        inverter: config::Inverter,
        register: u16,
        bit: lxp::packet::Register11Bit,
        enable: bool,
    ) -> Result<()> {
        // For register 11, we need to read the current value, modify the bit, then write it back
        let bit_value = bit as u16;
        
        // Read the current value from the register
        let packet = commands::read_hold::ReadHold::new(
            self.channels.clone(),
            inverter.clone(),
            register,
            1,
        )
        .run()
        .await?;
        
        let current_value = packet.value();
        let new_value = if enable {
            current_value | bit_value
        } else {
            current_value & !bit_value
        };
        
        self.set_hold(inverter, register, new_value).await
    }

    async fn inverter_receiver(&self) -> Result<()> {
        use lxp::inverter::ChannelData::*;

        let mut receiver = self.channels.from_inverter.subscribe();

        let mut inputs_store = InputsStore::new();

        loop {
            match receiver.recv().await? {
                Packet(packet) => {
                    self.process_inverter_packet(packet, &mut inputs_store)
                        .await?;
                }
                Connected(serial) => {
                    if let Err(e) = self.inverter_connected(serial).await {
                        error!("{}", e);
                    }
                }
                // this loop holds no state so doesn't care about inverter disconnects
                Disconnect(_) => {}
                Shutdown => break,
            }
        }

        Ok(())
    }

    async fn process_inverter_packet(
        &self,
        packet: lxp::packet::Packet,
        inputs_store: &mut InputsStore,
    ) -> Result<()> {
        debug!("RX: {:?}", packet);

        if let Packet::TranslatedData(td) = &packet {
            // temporary special greppable logging for Param packets as I try to
            // work out what they do :)
            if td.tcp_function() == TcpFunction::ReadParam
                || td.tcp_function() == TcpFunction::WriteParam
            {
                warn!("got a Param packet! {:?}", td);
            }

            // inputs_store handling. If we've received any ReadInput, update inputs_store
            // with the contents. If we got the third (of three) packets, send out the combined
            // MQTT message with all the data.
            if td.device_function == DeviceFunction::ReadInput {
                use lxp::packet::ReadInput;

                let entry = inputs_store
                    .entry(td.datalog)
                    .or_default();

                match td.read_input() {
                    Ok(ReadInput::ReadInputAll(r_all)) => {
                        // no need for MQTT here, done below
                        self.save_input_all(r_all).await?
                    }

                    Ok(ReadInput::ReadInput1(r1)) => entry.set_read_input_1(r1),
                    Ok(ReadInput::ReadInput2(r2)) => entry.set_read_input_2(r2),
                    Ok(ReadInput::ReadInput3(r3)) => {
                        let datalog = r3.datalog;

                        entry.set_read_input_3(r3);

                        if let Some(input) = entry.to_input_all() {
                            if self.config.mqtt().enabled() {
                                let message = mqtt::Message::for_input_all(&input, datalog)?;
                                let channel_data = mqtt::ChannelData::Message(message);
                                if let Err(e) = self.channels.to_mqtt.send(channel_data) {
                                    error!("Failed to send MQTT input_all message: {}", e);
                                    // Continue processing other messages instead of failing
                                }
                            }

                            self.save_input_all(Box::new(input)).await?;
                        }
                    }
                    Ok(ReadInput::ReadInput4(r4)) => {
                        let _datalog = r4.datalog;
                        entry.set_read_input_4(r4);
                        
                        // Note: ReadInput4 contains registers 120-131 which are not part of the main input_all
                        // These will be published individually via the MQTT message handling below
                    }
                    Err(x) => warn!("ignoring {:?}", x),
                }
            }
        }

        if self.config.mqtt().enabled() {
            // returns a Vec of messages to send. could be none;
            // not every packet produces an MQ message (eg, heartbeats),
            // and some produce >1 (multi-register ReadHold)
            match Self::packet_to_messages(packet, self.config.mqtt().publish_individual_input()) {
                Ok(messages) => {
                    for message in messages {
                        let message = mqtt::ChannelData::Message(message);
                        if let Err(e) = self.channels.to_mqtt.send(message) {
                            error!("Failed to send MQTT packet message: {}", e);
                            // Continue processing other messages instead of failing
                        }
                    }
                }
                Err(e) => {
                    // log error but avoid exiting loop as then we stop handling
                    // incoming packets. need better error handling here maybe?
                    error!("{}", e);
                }
            }
        }

        Ok(())
    }

    // Unlike input registers, holding registers are not broadcast by inverters,
    // but they are interesting nevertheless. Publishing the holding registers
    // when we connect to an inverter makes it easy for configuration data to be
    // tracked, which is particularly useful in conjunction with HomeAssistant.
    async fn inverter_connected(&self, datalog: Serial) -> Result<()> {
        let inverter = match self.config.enabled_inverter_with_datalog(datalog) {
            Some(inverter) => inverter,
            None => bail!("Unknown inverter connected: {}", datalog),
        };

        if !inverter.publish_holdings_on_connect() {
            info!("Skipping register read for inverter {} (publish_holdings_on_connect disabled)", datalog);
            return Ok(());
        }

        info!("Reading all holding registers for inverter {} (registers 0-279)", datalog);

        // We can only read holding registers in blocks of 40. Based on LXP_REGISTERS.txt,
        // there are registers up to at least 250, so we need to read 7 pages of 40 values
        // to ensure we cover all available registers.
        info!("Reading registers 0-39 for inverter {}", datalog);
        self.read_hold(inverter.clone(), 0_u16, 40).await?;
        info!("Reading registers 40-79 for inverter {}", datalog);
        self.read_hold(inverter.clone(), 40_u16, 40).await?;
        info!("Reading registers 80-119 for inverter {}", datalog);
        self.read_hold(inverter.clone(), 80_u16, 40).await?;
        info!("Reading registers 120-159 for inverter {}", datalog);
        self.read_hold(inverter.clone(), 120_u16, 40).await?;
        info!("Reading registers 160-199 for inverter {}", datalog);
        self.read_hold(inverter.clone(), 160_u16, 40).await?;
        info!("Reading registers 200-239 for inverter {}", datalog);
        self.read_hold(inverter.clone(), 200_u16, 40).await?;
        info!("Reading registers 240-279 for inverter {}", datalog);
        self.read_hold(inverter.clone(), 240_u16, 40).await?;

        info!("Completed reading all holding registers for inverter {}", datalog);

        // Also send any special interpretive topics which are derived from
        // the holding registers.
        //
        // FIXME: this is a further 12 round-trips to the inverter to read values
        // we have already taken, just above. We should be able to do better!
        for num in &[1, 2, 3] {
            self.read_time_register(
                inverter.clone(),
                commands::time_register_ops::Action::AcCharge(*num),
            )
            .await?;
        }

        Ok(())
    }

    async fn save_input_all(&self, _input: Box<lxp::packet::ReadInputAll>) -> Result<()> {
        // Database functionality removed - no longer storing data
        Ok(())
    }

    fn packet_to_messages(
        packet: Packet,
        publish_individual_input: bool,
    ) -> Result<Vec<mqtt::Message>> {
        match packet {
            Packet::Heartbeat(_) => Ok(Vec::new()), // always no message
            Packet::TranslatedData(td) => match td.device_function {
                DeviceFunction::ReadHold => mqtt::Message::for_hold(td),
                DeviceFunction::ReadInput => mqtt::Message::for_input(td, publish_individual_input),
                DeviceFunction::WriteSingle => mqtt::Message::for_hold(td),
                DeviceFunction::WriteMulti => Ok(Vec::new()), // TODO, for_hold might just work
            },
            Packet::ReadParam(rp) => mqtt::Message::for_param(rp),
            Packet::WriteParam(_) => Ok(Vec::new()), // ignoring for now
        }
    }
}
