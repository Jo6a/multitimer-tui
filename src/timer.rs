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
    pub initial_time: u64,
    pub timeleft_secs: u64,
    #[serde(skip_serializing, skip_deserializing)]
    pub endtime: DateTime<Local>,
    #[serde(skip_serializing, skip_deserializing)]
    pub action_info: String,
    pub timer_type: Option<String>,
    pub repeat_times: u64,
}

impl Timer {
    pub fn new(
        description: String,
        timeleft_secs: u64,
        left_view: bool,
        timer_type: Option<String>,
    ) -> Self {
        Self {
            id: 0,
            is_active: false,
            left_view,
            description,
            initial_time: timeleft_secs,
            timeleft_secs,
            endtime: Local::now(),
            action_info: "   ".to_string(),
            timer_type,
            repeat_times: 0,
        }
    }

    pub fn formatted(&self) -> String {
        let hours = self.timeleft_secs / 3600;
        let minutes = (self.timeleft_secs % 3600) / 60;
        let seconds = self.timeleft_secs % 60;

        format!(
            "{:02}:{:02}:{:02} ({}){}     @{}:{}     {}",
            hours,
            minutes,
            seconds,
            self.endtime.format("%Y-%m-%d %H:%M:%S"),
            self.action_info,
            self.id,
            self.description,
            if self.repeat_times > 0 {
                format!("repeat: {}", self.repeat_times)
            } else {
                "".to_string()
            }
        )
    }

    pub fn tick(&mut self) -> bool {
        self.is_active = true;
        if self.timeleft_secs > 0 {
            self.timeleft_secs -= 1;
            if self.timeleft_secs != 0 {
                return false;
            }

            Command::new("bash")
                .args(["-c", "echo -e \"\\a\" "])
                .spawn()
                .expect("Playing sound failed");
            self.is_active = false;

            if cfg!(target_os = "linux") {
                let _ = Command::new("notify-send")
                    .args(["Timer beendet", &self.description])
                    .spawn();
            } else if cfg!(target_os = "windows") {
                let _ = Command::new("msg")
                    .args(["*", "Timer beendet", &self.description])
                    .spawn();
            }
            if self.repeat_times > 0 {
                self.timeleft_secs = self.initial_time;
                self.repeat_times -= 1;
                return false;
            }

            return true;
        }
        false
    }
}
