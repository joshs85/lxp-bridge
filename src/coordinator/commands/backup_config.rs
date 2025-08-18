use crate::prelude::*;
use crate::coordinator::Coordinator;
use crate::lxp::packet::{DeviceFunction, TranslatedData, Packet};
use serde_json::json;
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use bytes::BytesMut;
use tokio_util::codec::Decoder;
use crate::lxp::packet_decoder::PacketDecoder;

pub struct BackupConfigCommand {
    inverter: config::Inverter,
    output_path: String,
}

impl BackupConfigCommand {
    pub fn new(inverter: config::Inverter, output_path: String) -> Self {
        Self {
            inverter,
            output_path,
        }
    }

    pub async fn execute(&self, _coordinator: &mut Coordinator) -> Result<()> {
        info!("Starting configuration backup for inverter {}", self.inverter.serial());
        info!("Output will be saved to: {}", self.output_path);

        // Read all holding registers in blocks of 40 (same as addon startup)
        info!("Reading all holding registers (registers 0-279)");
        let mut hold_registers: HashMap<String, u16> = HashMap::new();
        
        // Read registers 0-39
        info!("Reading registers 0-39");
        let values = self.read_register_chunk(0, 40).await?;
        for (i, &value) in values.iter().enumerate() {
            hold_registers.insert(i.to_string(), value);
        }
        
        // Read registers 40-79
        info!("Reading registers 40-79");
        let values = self.read_register_chunk(40, 40).await?;
        for (i, &value) in values.iter().enumerate() {
            hold_registers.insert((i + 40).to_string(), value);
        }
        
        // Read registers 80-119
        info!("Reading registers 80-119");
        let values = self.read_register_chunk(80, 40).await?;
        for (i, &value) in values.iter().enumerate() {
            hold_registers.insert((i + 80).to_string(), value);
        }
        
        // Read registers 120-159
        info!("Reading registers 120-159");
        let values = self.read_register_chunk(120, 40).await?;
        for (i, &value) in values.iter().enumerate() {
            hold_registers.insert((i + 120).to_string(), value);
        }
        
        // Read registers 160-199
        info!("Reading registers 160-199");
        let values = self.read_register_chunk(160, 40).await?;
        for (i, &value) in values.iter().enumerate() {
            hold_registers.insert((i + 160).to_string(), value);
        }
        
        // Read registers 200-239
        info!("Reading registers 200-239");
        let values = self.read_register_chunk(200, 40).await?;
        for (i, &value) in values.iter().enumerate() {
            hold_registers.insert((i + 200).to_string(), value);
        }
        
        // Read registers 240-279
        info!("Reading registers 240-279");
        let values = self.read_register_chunk(240, 40).await?;
        for (i, &value) in values.iter().enumerate() {
            hold_registers.insert((i + 240).to_string(), value);
        }

        // Read all input registers in blocks of 40 (same as addon startup)
        info!("Reading all input registers (registers 0-199)");
        let mut input_registers: HashMap<String, u16> = HashMap::new();
        
        // Read registers 0-39
        info!("Reading input registers 0-39");
        let values = self.read_input_chunk(0, 40).await?;
        for (i, &value) in values.iter().enumerate() {
            input_registers.insert(i.to_string(), value);
        }
        
        // Read registers 40-79
        info!("Reading input registers 40-79");
        let values = self.read_input_chunk(40, 40).await?;
        for (i, &value) in values.iter().enumerate() {
            input_registers.insert((i + 40).to_string(), value);
        }
        
        // Read registers 80-119
        info!("Reading input registers 80-119");
        let values = self.read_input_chunk(80, 40).await?;
        for (i, &value) in values.iter().enumerate() {
            input_registers.insert((i + 80).to_string(), value);
        }
        
        // Read registers 120-159
        info!("Reading input registers 120-159");
        let values = self.read_input_chunk(120, 40).await?;
        for (i, &value) in values.iter().enumerate() {
            input_registers.insert((i + 120).to_string(), value);
        }
        
        // Read registers 160-199
        info!("Reading input registers 160-199");
        let values = self.read_input_chunk(160, 40).await?;
        for (i, &value) in values.iter().enumerate() {
            input_registers.insert((i + 160).to_string(), value);
        }

        // Create the backup data structure
        let backup_data = json!({
            "backup_timestamp": chrono::Utc::now().to_rfc3339(),
            "inverter_serial": self.inverter.serial().to_string(),
            "inverter_datalog": self.inverter.datalog().to_string(),
            "inverter_host": self.inverter.host(),
            "inverter_port": self.inverter.port(),
            "hold_registers": hold_registers,
            "input_registers": input_registers,
            "notes": "Real register values read from inverter using direct TCP communication (same as addon startup)"
        });

        // Write to file
        let mut file = File::create(&self.output_path)?;
        serde_json::to_writer_pretty(&mut file, &backup_data)?;
        file.flush()?;

        info!("✅ Configuration backup completed successfully!");
        info!("📁 Backup saved to: {}", self.output_path);

        Ok(())
    }

    /// Read a chunk of holding registers using direct TCP communication
    async fn read_register_chunk(&self, start_register: u16, count: u16) -> Result<Vec<u16>> {
        // Create the request packet (copying from ReadHold command)
        let request_packet = Packet::TranslatedData(TranslatedData {
            datalog: self.inverter.datalog(),
            device_function: DeviceFunction::ReadHold,
            inverter: self.inverter.serial(),
            register: start_register,
            values: count.to_le_bytes().to_vec(),
        });

        // Send the request and get the response
        let response_packet = self.send_tcp_request(&request_packet).await?;
        
        // Extract the values from the response
        if let Packet::TranslatedData(td) = response_packet {
            let mut values = Vec::new();
            for chunk in td.values.chunks(2) {
                if chunk.len() == 2 {
                    let value = u16::from_le_bytes([chunk[0], chunk[1]]);
                    values.push(value);
                }
            }
            Ok(values)
        } else {
            Err(anyhow!("Expected TranslatedData response, got {:?}", response_packet))
        }
    }

    /// Read a chunk of input registers using direct TCP communication
    async fn read_input_chunk(&self, start_register: u16, count: u16) -> Result<Vec<u16>> {
        // Create the request packet (copying from ReadInputs command)
        let request_packet = Packet::TranslatedData(TranslatedData {
            datalog: self.inverter.datalog(),
            device_function: DeviceFunction::ReadInput,
            inverter: self.inverter.serial(),
            register: start_register,
            values: count.to_le_bytes().to_vec(),
        });

        // Send the request and get the response
        let response_packet = self.send_tcp_request(&request_packet).await?;
        
        // Extract the values from the response
        if let Packet::TranslatedData(td) = response_packet {
            let mut values = Vec::new();
            for chunk in td.values.chunks(2) {
                if chunk.len() == 2 {
                    let value = u16::from_le_bytes([chunk[0], chunk[1]]);
                    values.push(value);
                }
            }
            Ok(values)
        } else {
            Err(anyhow!("Expected TranslatedData response, got {:?}", response_packet))
        }
    }

    /// Send a TCP request and wait for response (copying from existing inverter communication)
    async fn send_tcp_request(&self, request_packet: &Packet) -> Result<Packet> {
        // Connect to the inverter
        let mut stream = TcpStream::connect((self.inverter.host(), self.inverter.port())).await?;
        
        // Build the TCP frame (copying from TcpFrameFactory)
        let request_bytes = lxp::packet::TcpFrameFactory::build(request_packet);
        
        // Send the request
        stream.write_all(&request_bytes).await?;
        
        // Read the response
        let _buf = BytesMut::new();
        let mut decoder = PacketDecoder::new();
        
        // Read response data
        let mut response_data = Vec::new();
        let mut buffer = [0u8; 1024];
        
        loop {
            let n = stream.read(&mut buffer).await?;
            if n == 0 {
                break;
            }
            response_data.extend_from_slice(&buffer[..n]);
            
            // Try to decode packets from the accumulated data
            let mut temp_buf = BytesMut::from(&response_data[..]);
            while let Some(packet) = decoder.decode(&mut temp_buf)? {
                // Found a complete packet, return it
                return Ok(packet);
            }
            
            // If we haven't found a complete packet yet, continue reading
            if response_data.len() > 4096 {
                // Prevent infinite loop with large responses
                break;
            }
        }
        
        Err(anyhow!("No complete packet found in response"))
    }
}
