use crate::pin::{Pin, PinMode, PinType};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

#[derive(Debug, Serialize, Deserialize)]
pub struct ArduinoBoard {
    pub name: String,
    pub pins: HashMap<u8, Pin>,
}

impl ArduinoBoard {
    pub fn new(name: &str) -> Self {
        let mut board = ArduinoBoard {
            name: name.to_string(),
            pins: HashMap::new(),
        };
        for i in 0..14 {
            board.pins.insert(i, Pin::new(i, PinType::Digital));
        }

        for i in 14..20 {
            board.pins.insert(i, Pin::new(i, PinType::Analog));
        }

        for &i in &[3, 5, 6, 9, 10, 11] {
            board.pins.insert(i, Pin::new(i, PinType::PWM));
        }

        board
    }

    pub fn set_pin_usage(
        &mut self,
        pin_number: u8,
        in_use: bool,
        function: Option<String>,
        mode: PinMode,
    ) -> Result<(), String> {
        if let Some(pin) = self.pins.get_mut(&pin_number) {
            pin.set_usage(in_use, function, mode);
            Ok(())
        } else {
            Err(format!("Pin {} not found", pin_number))
        }
    }

    pub fn clear_pin_usage(&mut self, pin_number: u8) -> Result<(), String> {
        if let Some(pin) = self.pins.get_mut(&pin_number) {
            pin.clear_usage();
            Ok(())
        } else {
            Err(format!("Pin {} not found", pin_number))
        }
    }

    pub fn save_to_file(&self, filename: &str) -> Result<(), String> {
        let json = serde_json::to_string_pretty(&self)
            .map_err(|e| format!("Failed to serialize board data: {}", e))?;
        fs::write(filename, json).map_err(|e| format!("Failed to write to file: {}", e))?;
        Ok(())
    }

    pub fn load_from_file(filename: &str) -> Result<Self, String> {
        if !Path::new(filename).exists() {
            return Ok(Self::new("Arduino Uno"));
        }

        let contents =
            fs::read_to_string(filename).map_err(|e| format!("Format to read file: {}", e))?;
        serde_json::from_str(&contents).map_err(|e| format!("Failed to parse JSON:{}", e))
    }

    pub fn display_status(&self) {
        println!("\n Arduino {} Pin Status:", self.name);
        println!("------------------------------------");
        for i in 0..20 {
            if let Some(pin) = self.pins.get(&i) {
                println!("{}", pin);
            }
        }
    }
}
