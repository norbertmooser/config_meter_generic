#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs::File;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub write_registers: Vec<ConfigWriteRegister>,
    pub read_registers: Vec<ConfigRegister>,
}

#[derive(Debug, Deserialize)]
pub struct ConfigRegister {
    pub name: String,
    pub address: u16,
}

#[derive(Debug, Deserialize)]
pub struct ConfigWriteRegister {
    pub name: String,
    pub address: u16,
    pub value: f32,
}

impl Config {
    pub fn from_file(file_path: &str) -> Result<Self, Box<dyn Error>> {
        let file = File::open(file_path)?;
        let config: Config = serde_yaml::from_reader(file)?;
        Ok(config)
    }
    pub fn get_read_registers(&self) -> Vec<ConfigRegister> {
        self.read_registers.iter()
            .map(|reg| ConfigRegister { name: reg.name.clone(), address: reg.address })
            .collect()
    }

    pub fn get_write_registers(&self) -> Vec<ConfigWriteRegister> {
        self.write_registers.iter()
            .map(|reg| ConfigWriteRegister { name: reg.name.clone(), address: reg.address, value: reg.value })
            .collect()
    }
}