use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use std::process::Command;

#[derive(Serialize, Deserialize)]
pub struct Timer {
    #[serde(skip_serializing, skip_deserializing)]
    pub id: u16,
    #[serde(skip_serializing, skip_deserializing)]
    pub is_active: bool,
    pub left_view: bool,
    pub description: String,
    pub timeleft_secs: u16,
    pub endtime: DateTime<Local>,
}

impl Timer {
    pub fn new(description: String, timeleft_secs: u16, left_view: bool) -> Self {
        Self {
            id: 0,
            is_active: false,
            left_view,
            description,
            timeleft_secs,
            endtime: Local::now(),
        }
    }

    pub fn formatted(&self) -> String {
        let hours = self.timeleft_secs / 3600;
        let minutes = (self.timeleft_secs % 3600) / 60;
        let seconds = self.timeleft_secs % 60;
        format!(
            "{:02}:{:02}:{:02} ({})         @{}:{}",
            hours,
            minutes,
            seconds,
            self.endtime.format("%Y-%m-%d %H:%M:%S"),
            self.id.to_string(),
            self.description
        )
    }

    pub fn tick(&mut self) {
        self.is_active = true;
        if self.timeleft_secs > 0 {
            self.timeleft_secs -= 1;
        }

        if self.timeleft_secs == 0 {
            Command::new("bash")
                .args(&["-c", "echo -e \"\\a\" "])
                .spawn()
                .expect("Playing sound failed");
            self.is_active = false;
        }
    }
}
