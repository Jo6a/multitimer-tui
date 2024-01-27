use std::{fmt, str::FromStr};

pub enum UiState {
    TimerUi,
    ConfigUi,
}

impl UiState {
    pub fn get_current_ui(index: usize) -> Self {
        match index {
            0 => UiState::TimerUi,
            1 => UiState::ConfigUi,
            _ => UiState::TimerUi,
        }
    }
}

pub enum ConfigType {
    DarkMode,
    ActiveColor,
    ReverseAddingTimer,
    MoveFinishedTimer,
    ActionAfterTimer,
    PomodoroTime,
    PomodoroSmallBreak,
    PomodoroBigBreak,
}

impl Default for ConfigType {
    fn default() -> Self {
        Self::DarkMode
    }
}

impl ConfigType {
    pub fn next(&mut self) {
        *self = match self {
            ConfigType::DarkMode => ConfigType::ActiveColor,
            ConfigType::ActiveColor => ConfigType::ReverseAddingTimer,
            ConfigType::ReverseAddingTimer => ConfigType::MoveFinishedTimer,
            ConfigType::MoveFinishedTimer => ConfigType::ActionAfterTimer,
            ConfigType::ActionAfterTimer => ConfigType::PomodoroTime,
            ConfigType::PomodoroTime => ConfigType::PomodoroSmallBreak,
            ConfigType::PomodoroSmallBreak => ConfigType::PomodoroBigBreak,
            ConfigType::PomodoroBigBreak => ConfigType::DarkMode,
        };
    }

    pub fn previous(&mut self) {
        *self = match self {
            ConfigType::DarkMode => ConfigType::PomodoroBigBreak,
            ConfigType::ActiveColor => ConfigType::DarkMode,
            ConfigType::ReverseAddingTimer => ConfigType::ActiveColor,
            ConfigType::MoveFinishedTimer => ConfigType::ReverseAddingTimer,
            ConfigType::ActionAfterTimer => ConfigType::MoveFinishedTimer,
            ConfigType::PomodoroTime => ConfigType::ActionAfterTimer,
            ConfigType::PomodoroSmallBreak => ConfigType::PomodoroTime,
            ConfigType::PomodoroBigBreak => ConfigType::PomodoroSmallBreak,
        }
    }
}

impl fmt::Display for ConfigType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            _ => write!(f, ""),
        }
    }
}

pub enum TimerAction {
    None,
    Hibernate,
    Shutdown,
}

impl TimerAction {
    pub fn next(&self) -> Self {
        match self {
            TimerAction::None => TimerAction::Hibernate,
            TimerAction::Hibernate => TimerAction::Shutdown,
            TimerAction::Shutdown => TimerAction::None,
        }
    }

    pub fn previous(&mut self) -> Self {
        match self {
            TimerAction::None => TimerAction::Shutdown,
            TimerAction::Hibernate => TimerAction::None,
            TimerAction::Shutdown => TimerAction::Hibernate,
        }
    }
}

impl fmt::Display for TimerAction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TimerAction::None => write!(f, "None"),
            TimerAction::Hibernate => write!(f, "Hibernate"),
            TimerAction::Shutdown => write!(f, "Shutdown"),
        }
    }
}

impl FromStr for TimerAction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "None" => Ok(TimerAction::None),
            "Hibernate" => Ok(TimerAction::Hibernate),
            "Shutdown" => Ok(TimerAction::Shutdown),
            _ => Ok(TimerAction::None),
        }
    }
}
