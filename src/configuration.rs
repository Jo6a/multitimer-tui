use chrono::Local;
use ratatui::widgets::TableState;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

use crate::color::AcceptedColors;
use crate::timer::Timer;
use crate::ui_states::{ConfigType, TimerAction};
use crate::utils::reverse_bool;

#[derive(Serialize, Deserialize)]
pub struct Configuration<'a> {
    pub darkmode: bool,
    pub activecolor: String,
    pub reverseadding: bool,
    pub move_finished_timer: bool,
    pub action_timeout: String,
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
    pub activecolor_str: String,
    #[serde(skip_serializing, skip_deserializing)]
    pub reverseadding_str: String,
    #[serde(skip_serializing, skip_deserializing)]
    pub move_finished_timer_str: String,
    #[serde(skip_serializing, skip_deserializing)]
    pub action_timeout_str: String,
    #[serde(skip_serializing, skip_deserializing)]
    pub pomodoro_time_table_str: String,
    #[serde(skip_serializing, skip_deserializing)]
    pub pomodoro_smallbreak_table_str: String,
    #[serde(skip_serializing, skip_deserializing)]
    pub pomodoro_bigbreak_table_str: String,
    #[serde(skip_serializing, skip_deserializing)]
    pub config_type: ConfigType,
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
            activecolor: "Green".to_string(),
            reverseadding: false,
            move_finished_timer: true,
            action_timeout: "None".to_string(),
            show_popup: false,
            titles: Vec::new(),
            index: 0,
            state: TableState::default(),
            darkmode_str: "".to_string(),
            activecolor_str: "".to_string(),
            reverseadding_str: "".to_string(),
            move_finished_timer_str: "".to_string(),
            action_timeout_str: "".to_string(),
            pomodoro_time_table_str: "".to_string(),
            pomodoro_smallbreak_table_str: "".to_string(),
            pomodoro_bigbreak_table_str: "".to_string(),
            config_type: ConfigType::default(),
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
                if i >= 7 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
        self.config_type.next();
    }

    pub fn previous_table_entry(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    7
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
        self.config_type.previous();
    }

    pub fn clear_table_entry(&mut self) {
        match self.state.selected().unwrap() {
            0 => self.darkmode_str.clear(),
            1 => self.activecolor_str.clear(),
            2 => self.reverseadding_str.clear(),
            3 => self.move_finished_timer_str.clear(),
            4 => self.action_timeout_str.clear(),
            5 => self.pomodoro_time_table_str.clear(),
            6 => self.pomodoro_smallbreak_table_str.clear(),
            7 => self.pomodoro_bigbreak_table_str.clear(),
            _ => {}
        }
    }

    pub fn write_table_entry(&mut self, c: char) {
        match self.state.selected().unwrap() {
            0 => self.darkmode_str.push(c),
            1 => self.activecolor_str.push(c),
            2 => self.reverseadding_str.push(c),
            3 => self.move_finished_timer_str.push(c),
            4 => self.action_timeout_str.push(c),
            5 => self.pomodoro_time_table_str.push(c),
            6 => self.pomodoro_smallbreak_table_str.push(c),
            7 => self.pomodoro_bigbreak_table_str.push(c),
            _ => {}
        }
    }

    pub fn pop_table_entry(&mut self) -> Option<char> {
        match self.state.selected().unwrap() {
            0 => self.darkmode_str.pop(),
            1 => self.activecolor_str.pop(),
            2 => self.reverseadding_str.pop(),
            3 => self.move_finished_timer_str.pop(),
            4 => self.action_timeout_str.pop(),
            5 => self.pomodoro_time_table_str.pop(),
            6 => self.pomodoro_smallbreak_table_str.pop(),
            7 => self.pomodoro_bigbreak_table_str.pop(),
            _ => " ".to_string().pop(),
        }
    }

    pub fn save_table_changes(&mut self) {
        self.darkmode = if self.darkmode_str.is_empty() {
            self.darkmode_str = "false".to_string();
            false
        } else {
            self.darkmode_str.parse::<bool>().unwrap_or_default()
        };
        self.activecolor = if self.activecolor_str.is_empty() {
            self.activecolor_str = "Green".to_string();
            "Green".to_string()
        } else {
            self.activecolor_str.clone()
        };

        self.reverseadding = self.reverseadding_str.parse::<bool>().unwrap_or_default();
        self.move_finished_timer = self
            .move_finished_timer_str
            .parse::<bool>()
            .unwrap_or_default();

        self.action_timeout = if self.action_timeout_str.is_empty() {
            self.action_timeout_str = "None".to_string();
            "None".to_string()
        } else {
            self.action_timeout_str.clone()
        };
        self.pomodoro_time = if self.pomodoro_time_table_str.is_empty() {
            self.pomodoro_time_table_str = "25".to_string();
            25
        } else {
            self.pomodoro_time_table_str.parse::<u16>().unwrap()
        };
        self.pomodoro_smallbreak = if self.pomodoro_smallbreak_table_str.is_empty() {
            self.pomodoro_smallbreak_table_str = "5".to_string();
            5
        } else {
            self.pomodoro_smallbreak_table_str.parse::<u16>().unwrap()
        };

        self.pomodoro_bigbreak = if self.pomodoro_bigbreak_table_str.is_empty() {
            self.pomodoro_bigbreak_table_str = "10".to_string();
            10
        } else {
            self.pomodoro_bigbreak_table_str.parse::<u16>().unwrap()
        };
        self.write_to_file().unwrap();
    }

    pub fn update_timers(&mut self) {
        let mut dt = Local::now();
        let mut dt2 = dt;
        let mut last_left = None;
        let mut last_right = None;
        for (i, timer) in self.timers.iter_mut().enumerate() {
            if timer.timeleft_secs != 0 {
                if timer.left_view {
                    dt += chrono::Duration::seconds(timer.timeleft_secs as i64);
                    timer.endtime = dt;
                    last_left = Some(i);
                } else {
                    dt2 += chrono::Duration::seconds(timer.timeleft_secs as i64);
                    timer.endtime = dt2;
                    last_right = Some(i);
                }
            }
            timer.id = i as u16;
            timer.is_active = false;
            timer.action_info = "   ".to_string();
        }
        if self.action_timeout != "None" {
            let action_display = match self.action_timeout.as_str() {
                "Hibernate" => "(H)",
                "Shutdown" => "(S)",
                _ => "",
            };
            if let (Some(left), Some(right)) = (last_left, last_right) {
                if self.timers[left].timeleft_secs > self.timers[right].timeleft_secs {
                    self.timers[left].action_info = action_display.to_string();
                } else {
                    self.timers[right].action_info = action_display.to_string();
                }
            } else if let Some(i) = last_left.or(last_right) {
                self.timers[i].action_info = action_display.to_string();
            }
        }
    }

    pub fn add_timer_to_config(&mut self, timer: Timer, reverse_adding: bool) {
        if (self.reverseadding && !reverse_adding) || (!self.reverseadding && reverse_adding) {
            self.timers.insert(0, timer);
        } else {
            self.timers.push(timer);
        }
    }

    pub fn create_timer_for_input(
        &mut self,
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
        Timer::new(
            argument2.to_owned(),
            seconds + minutes * 60 + hours * 3600,
            left_view,
        )
    }

    pub fn num_rightview_timers(&mut self) -> usize {
        self.timers.iter().filter(|t| !t.left_view).count()
    }

    pub fn check_all_timers_done(&mut self) -> bool {
        for timer in self.timers.iter() {
            if timer.timeleft_secs > 0 {
                return false;
            }
        }
        true
    }

    pub fn move_value_right(&mut self) {
        match self.config_type {
            ConfigType::DarkMode => self.darkmode_str = reverse_bool(&self.darkmode_str),
            ConfigType::ActiveColor => {
                let parsed_color = AcceptedColors::from_str(&self.activecolor_str)
                    .unwrap()
                    .next_color();
                self.activecolor_str = parsed_color.to_string();
            }
            ConfigType::ReverseAddingTimer => {
                self.reverseadding_str = reverse_bool(&self.reverseadding_str)
            }
            ConfigType::MoveFinishedTimer => {
                self.move_finished_timer_str = reverse_bool(&self.move_finished_timer_str)
            }
            ConfigType::ActionAfterTimer => {
                let parsed_value = TimerAction::from_str(&self.action_timeout_str)
                    .unwrap()
                    .next();
                self.action_timeout_str = parsed_value.to_string();
            }
            ConfigType::PomodoroTime => {
                let mut parsed_value = self.pomodoro_time_table_str.parse::<i32>().unwrap();
                if parsed_value < 99 {
                    parsed_value += 1;
                }
                self.pomodoro_time_table_str = parsed_value.to_string();
            }
            ConfigType::PomodoroSmallBreak => {
                let mut parsed_value = self.pomodoro_smallbreak_table_str.parse::<i32>().unwrap();
                if parsed_value < 99 {
                    parsed_value += 1;
                }
                self.pomodoro_smallbreak_table_str = parsed_value.to_string();
            }
            ConfigType::PomodoroBigBreak => {
                let mut parsed_value = self.pomodoro_bigbreak_table_str.parse::<i32>().unwrap();
                if parsed_value < 99 {
                    parsed_value += 1;
                }
                self.pomodoro_bigbreak_table_str = parsed_value.to_string();
            }
        };
    }

    pub fn move_value_left(&mut self) {
        match self.config_type {
            ConfigType::DarkMode => self.darkmode_str = reverse_bool(&self.darkmode_str),
            ConfigType::ActiveColor => {
                let parsed_color = AcceptedColors::from_str(&self.activecolor_str)
                    .unwrap()
                    .previous_color();
                self.activecolor_str = parsed_color.to_string();
            }
            ConfigType::ReverseAddingTimer => {
                self.reverseadding_str = reverse_bool(&self.reverseadding_str)
            }
            ConfigType::MoveFinishedTimer => {
                self.move_finished_timer_str = reverse_bool(&self.move_finished_timer_str)
            }
            ConfigType::ActionAfterTimer => {
                let parsed_value = TimerAction::from_str(&self.action_timeout_str)
                    .unwrap()
                    .previous();
                self.action_timeout_str = parsed_value.to_string();
            }
            ConfigType::PomodoroTime => {
                let mut parsed_value = self.pomodoro_time_table_str.parse::<i32>().unwrap();
                if parsed_value > 0 {
                    parsed_value -= 1;
                }
                self.pomodoro_time_table_str = parsed_value.to_string();
            }
            ConfigType::PomodoroSmallBreak => {
                let mut parsed_value = self.pomodoro_smallbreak_table_str.parse::<i32>().unwrap();
                if parsed_value > 0 {
                    parsed_value -= 1;
                }
                self.pomodoro_smallbreak_table_str = parsed_value.to_string();
            }
            ConfigType::PomodoroBigBreak => {
                let mut parsed_value = self.pomodoro_bigbreak_table_str.parse::<i32>().unwrap();
                if parsed_value > 0 {
                    parsed_value -= 1;
                }
                self.pomodoro_bigbreak_table_str = parsed_value.to_string();
            }
        };
    }
}
