use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PinType {
    Digital,
    Analog,
    PWM,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PinMode {
    Input,
    Output,
    InputPullup,
    InputPulldown,
    Unset,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pin {
    pub number: u8,
    pub pin_type: PinType,
    pub is_in_use: bool,
    pub current_function: Option<String>,
    mode: PinMode::Unset,
}

impl Pin {
    pub fn new(number: u8, pin_type: PinType) -> Self {
        Pin {
            number,
            pin_type,
            is_in_use: false,
            current_function: None,
            mode: PinMode::Unset,
        }
    }

    pub fn set_usage(&mut self, in_use: bool, function: Option<String>, mode: PinMode) {
        self.is_in_use = in_use;
        self.current_function = function;
        self.mode = mode;
    }

    pub fn clear_usage(&mut self) {
        self.is_in_use = false;
        self.current_function = None;
        self.mode = PinMode::Unset;
    }
}

impl fmt::Display for Pin {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Pin {}: {:?} - {} - Mode: {:?} {}",
            self.number,
            self.pin_type,
            if self.is_in_use {
                "In Use"
            } else {
                "Available"
            },
            self.mode,
            self.current_function
                .as_ref()
                .map_or("".to_string(), |f| format!("({})", f))
        )
    }
}
