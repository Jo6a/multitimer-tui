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
    UrgentColor,
    ImportantColor,
    FocusColor,
    BreakColor,
    StudyColor,
    CodingColor,
    CasualColor,
    FunColor,
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
            ConfigType::PomodoroBigBreak => ConfigType::UrgentColor,
            ConfigType::UrgentColor => ConfigType::ImportantColor,
            ConfigType::ImportantColor => ConfigType::FocusColor,
            ConfigType::FocusColor => ConfigType::BreakColor,
            ConfigType::BreakColor => ConfigType::StudyColor,
            ConfigType::StudyColor => ConfigType::CodingColor,
            ConfigType::CodingColor => ConfigType::CasualColor,
            ConfigType::CasualColor => ConfigType::FunColor,
            ConfigType::FunColor => ConfigType::DarkMode,
        };
    }

    pub fn previous(&mut self) {
        *self = match self {
            ConfigType::DarkMode => ConfigType::FunColor,
            ConfigType::ActiveColor => ConfigType::DarkMode,
            ConfigType::ReverseAddingTimer => ConfigType::ActiveColor,
            ConfigType::MoveFinishedTimer => ConfigType::ReverseAddingTimer,
            ConfigType::ActionAfterTimer => ConfigType::MoveFinishedTimer,
            ConfigType::PomodoroTime => ConfigType::ActionAfterTimer,
            ConfigType::PomodoroSmallBreak => ConfigType::PomodoroTime,
            ConfigType::PomodoroBigBreak => ConfigType::PomodoroSmallBreak,
            ConfigType::UrgentColor => ConfigType::PomodoroBigBreak,
            ConfigType::ImportantColor => ConfigType::UrgentColor,
            ConfigType::FocusColor => ConfigType::ImportantColor,
            ConfigType::BreakColor => ConfigType::FocusColor,
            ConfigType::StudyColor => ConfigType::BreakColor,
            ConfigType::CodingColor => ConfigType::StudyColor,
            ConfigType::CasualColor => ConfigType::CodingColor,
            ConfigType::FunColor => ConfigType::CasualColor,
        }
    }
}

impl fmt::Display for ConfigType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConfigType::UrgentColor => write!(f, "urgent"),
            ConfigType::ImportantColor => write!(f, "important"),
            ConfigType::FocusColor => write!(f, "focus"),
            ConfigType::BreakColor => write!(f, "break"),
            ConfigType::StudyColor => write!(f, "study"),
            ConfigType::CodingColor => write!(f, "coding"),
            ConfigType::CasualColor => write!(f, "casual"),
            ConfigType::FunColor => write!(f, "fun"),
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
