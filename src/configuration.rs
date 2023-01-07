use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use std::process::Command;
use tui::style::Color;
use tui::widgets::TableState;

#[derive(Serialize, Deserialize)]
pub struct Configuration<'a> {
    pub darkmode: bool,
    pub reverseadding: bool,
    pub pomodoro_time: u16,
    pub pomodoro_smallbreak: u16,
    pub pomodoro_bigbreak: u16,
    pub timers: Vec<Timer>,
    #[serde(skip_serializing, skip_deserializing)]
    pub show_popup: bool,
    #[serde(skip_serializing, skip_deserializing)]
    pub titles: Vec<&'a str>,
    #[serde(skip_serializing, skip_deserializing)]
    pub index: usize,
    #[serde(skip_serializing, skip_deserializing)]
    pub state: TableState,
    #[serde(skip_serializing, skip_deserializing)]
    pub darkmode_str: String,
    #[serde(skip_serializing, skip_deserializing)]
    pub reverseadding_str: String,
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
            reverseadding: false,
            show_popup: false,
            titles: Vec::new(),
            index: 0,
            state: TableState::default(),
            darkmode_str: "".to_string(),
            reverseadding_str: "".to_string(),
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
                if i >= 5 - 1 {
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
                    5 - 1
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
            0 => self.darkmode_str.clear(),
            1 => self.reverseadding_str.clear(),
            2 => self.pomodoro_time_table_str.clear(),
            3 => self.pomodoro_smallbreak_table_str.clear(),
            4 => self.pomodoro_bigbreak_table_str.clear(),
            _ => return,
        }
    }

    pub fn write_table_entry(&mut self, c: char) {
        match self.state.selected().unwrap() {
            0 => self.darkmode_str.push(c),
            1 => self.reverseadding_str.push(c),
            2 => self.pomodoro_time_table_str.push(c),
            3 => self.pomodoro_smallbreak_table_str.push(c),
            4 => self.pomodoro_bigbreak_table_str.push(c),
            _ => return,
        }
    }

    pub fn pop_table_entry(&mut self) -> Option<char> {
        match self.state.selected().unwrap() {
            0 => self.darkmode_str.pop(),
            1 => self.reverseadding_str.pop(),
            2 => self.pomodoro_time_table_str.pop(),
            3 => self.pomodoro_smallbreak_table_str.pop(),
            4 => self.pomodoro_bigbreak_table_str.pop(),
            _ => return " ".to_string().pop(),
        }
    }

    pub fn save_table_changes(&mut self) {
        self.darkmode = self.darkmode_str.parse::<bool>().unwrap_or_default();
        self.reverseadding = self.reverseadding_str.parse::<bool>().unwrap_or_default();
        self.pomodoro_time = self.pomodoro_time_table_str.parse::<u16>().unwrap();
        self.pomodoro_smallbreak = self.pomodoro_smallbreak_table_str.parse::<u16>().unwrap();
        self.pomodoro_bigbreak = self.pomodoro_bigbreak_table_str.parse::<u16>().unwrap();
        self.write_to_file().unwrap();
    }

    pub fn get_background_color(self) -> Color {
        if self.darkmode {
            return Color::Black;
        } else {
            return Color::White;
        };
    }

    pub fn get_foreground_color(self) -> Color {
        if self.darkmode {
            return Color::White;
        } else {
            return Color::Black;
        };
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

pub fn update_timers(timers: &mut Vec<Timer>) {
    let mut dt = Local::now();
    let mut dt2 = dt.clone();
    for (i, timer) in timers.into_iter().enumerate() {
        if timer.timeleft_secs != 0 {
            if timer.left_view {
                dt += chrono::Duration::seconds(timer.timeleft_secs as i64);
                timer.endtime = dt;
            } else {
                dt2 += chrono::Duration::seconds(timer.timeleft_secs as i64);
                timer.endtime = dt2;
            }
        }
        timer.id = i as u16;
        timer.is_active = false;
    }
}

pub fn add_timer_to_config(config: &mut Configuration, timer: Timer) {
    if config.reverseadding {
        config.timers.insert(0, timer);
    } else {
        config.timers.push(timer);
    }
}

pub fn create_timer_for_input(
    argument1: &String,
    argument2: &mut String,
    left_view: bool,
) -> Timer {
    let hours: u16;
    let minutes: u16;
    let seconds: u16;
    if argument1.len() == 8 {
        hours = argument1[0..2].parse::<u16>().unwrap_or_default();
        minutes = argument1[3..5].parse::<u16>().unwrap_or_default();
        seconds = argument1[6..8].parse::<u16>().unwrap_or_default();
    } else {
        let min_entered = argument1[..].parse::<u16>().unwrap_or_default();
        if min_entered == 0 {
            *argument2 = argument1.to_owned() + " " + &argument2[..]; /* no argument1 with minutes entered */
        }
        hours = min_entered / 60;
        minutes = min_entered % 60;
        seconds = 0;
    }
    let timer = Timer::new(
        argument2.to_owned(),
        seconds + minutes * 60 + hours * 3600,
        left_view,
    );
    timer
}

pub fn num_rightview_timers(timers: &Vec<Timer>) -> usize {
    timers.iter().filter(|t| t.left_view == false).count()
}
