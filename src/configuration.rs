use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use std::process::Command;
use tui::widgets::TableState;

#[derive(Serialize, Deserialize)]
pub struct Configuration<'a> {
    pub pomodoro_time: u16,
    pub pomodoro_smallbreak: u16,
    pub pomodoro_bigbreak: u16,
    pub timers: Vec<Timer>,
    #[serde(skip_serializing, skip_deserializing)]
    pub darkmode: bool,
    #[serde(skip_serializing, skip_deserializing)]
    pub show_popup: bool,
    #[serde(skip_serializing, skip_deserializing)]
    pub titles: Vec<&'a str>,
    #[serde(skip_serializing, skip_deserializing)]
    pub index: usize,
    #[serde(skip_serializing, skip_deserializing)]
    pub state: TableState,
    #[serde(skip_serializing, skip_deserializing)]
    pub pomodoro_time_table_str: String,
    #[serde(skip_serializing, skip_deserializing)]
    pub pomodoro_smallbreak_table_str: String,
    #[serde(skip_serializing, skip_deserializing)]
    pub pomodoro_bigbreak_table_str: String,
}

impl<'a> Configuration<'a> {
    pub fn new(
        pomodoro_time: u16,
        pomodoro_smallbreak: u16,
        pomodoro_bigbreak: u16,
    ) -> Configuration<'a> {
        Configuration {
            pomodoro_time,
            pomodoro_smallbreak,
            pomodoro_bigbreak,
            timers: Vec::new(),
            darkmode: true,
            show_popup: false,
            titles: Vec::new(),
            index: 0,
            state: TableState::default(),
            pomodoro_time_table_str: "".to_string(),
            pomodoro_smallbreak_table_str: "".to_string(),
            pomodoro_bigbreak_table_str: "".to_string(),
        }
    }

    pub fn write_to_file(&self) -> Result<(), std::io::Error> {
        std::fs::write("config.json", serde_json::to_string_pretty(self).unwrap())
    }

    pub fn next(&mut self) {
        self.index = (self.index + 1) % self.titles.len();
    }

    pub fn previous(&mut self) {
        if self.index > 0 {
            self.index -= 1;
        } else {
            self.index = self.titles.len() - 1;
        }
    }

    pub fn next_table_entry(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= 4 - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn previous_table_entry(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    4 - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn clear_table_entry(&mut self) {
        match self.state.selected().unwrap() {
            0 => self.pomodoro_time_table_str.clear(),
            1 => self.pomodoro_smallbreak_table_str.clear(),
            2 => self.pomodoro_bigbreak_table_str.clear(),
            _ => return,
        }
    }

    pub fn write_table_entry(&mut self, c: char) {
        match self.state.selected().unwrap() {
            0 => self.pomodoro_time_table_str.push(c),
            1 => self.pomodoro_smallbreak_table_str.push(c),
            2 => self.pomodoro_bigbreak_table_str.push(c),
            _ => return,
        }
    }

    pub fn pop_table_entry(&mut self) -> Option<char> {
        match self.state.selected().unwrap() {
            0 => self.pomodoro_time_table_str.pop(),
            1 => self.pomodoro_smallbreak_table_str.pop(),
            2 => self.pomodoro_bigbreak_table_str.pop(),
            _ => return " ".to_string().pop(),
        }
    }

    pub fn save_table_changes(&mut self) {
        self.pomodoro_time = self.pomodoro_time_table_str.parse::<u16>().unwrap();
        self.pomodoro_smallbreak = self.pomodoro_smallbreak_table_str.parse::<u16>().unwrap();
        self.pomodoro_bigbreak = self.pomodoro_bigbreak_table_str.parse::<u16>().unwrap();
        self.write_to_file().unwrap();
    }
}

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
        }
        timer.id = i as u16;
        timer.is_active = false;
    }
}

pub fn num_rightview_timers(timers: &Vec<Timer>) -> usize {
    timers.iter().filter(|t| t.left_view == false).count()
}
