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
    pub hours: u16,
    pub minutes: u16,
    pub seconds: u16,
    pub endtime: DateTime<Local>,
}

impl Timer {
    pub fn new(
        description: String,
        hours: u16,
        minutes: u16,
        seconds: u16,
        left_view: bool,
    ) -> Self {
        Self {
            id: 0,
            is_active: false,
            left_view,
            description,
            hours,
            minutes,
            seconds,
            endtime: Local::now(),
        }
    }

    pub fn formatted(&self) -> String {
        format!(
            "{:02}:{:02}:{:02} ({})         @{}:{}",
            self.hours,
            self.minutes,
            self.seconds,
            self.endtime.format("%Y-%m-%d %H:%M:%S"),
            self.id.to_string(),
            self.description
        )
    }

    pub fn tick(&mut self) {
        self.is_active = true;
        if self.seconds > 0 {
            self.seconds -= 1;
        } else if self.minutes > 0 {
            self.minutes -= 1;
            self.seconds = 59;
        } else if self.hours > 0 {
            self.hours -= 1;
            self.minutes = 59;
            self.seconds = 59;
        }

        if self.seconds == 0 && self.minutes == 0 && self.hours == 0 {
            Command::new("bash")
                .args(&["-c", "echo -e \"\\a\" "])
                .spawn()
                .expect("Playing sound failed");
            self.is_active = false;
        }
    }
}
pub fn update_timers(timers: &mut Vec<Timer>) {
    let mut dt = Local::now();
    let mut dt2 = dt.clone();
    for (i, timer) in timers.into_iter().enumerate() {
        if timer.seconds != 0 || timer.minutes != 0 || timer.hours != 0 {
            if timer.left_view {
                dt += chrono::Duration::seconds(timer.seconds as i64)
                    + chrono::Duration::minutes(timer.minutes as i64)
                    + chrono::Duration::hours(timer.hours as i64);

                timer.endtime = dt;
            } else {
                dt2 += chrono::Duration::seconds(timer.seconds as i64)
                    + chrono::Duration::minutes(timer.minutes as i64)
                    + chrono::Duration::hours(timer.hours as i64);

                timer.endtime = dt2;
            }

            timer.id = i as u16;
            timer.is_active = false;
        }
    }
}

pub fn num_rightview_timers(timers: &Vec<Timer>) -> usize {
    timers.iter().filter(|t| t.left_view == false).count()
}