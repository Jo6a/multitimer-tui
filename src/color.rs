use std::{fmt, str::FromStr};

use tui::style::Color;

pub enum AcceptedColors {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    Gray,
    DarkGray,
    LightRed,
    LightGreen,
    LightYellow,
    LightBlue,
    LightMagenta,
    LightCyan,
}

impl AcceptedColors {
    pub fn next_color(&self) -> AcceptedColors {
        match self {
            AcceptedColors::Black => AcceptedColors::Red,
            AcceptedColors::Red => AcceptedColors::Green,
            AcceptedColors::Green => AcceptedColors::Yellow,
            AcceptedColors::Yellow => AcceptedColors::Blue,
            AcceptedColors::Blue => AcceptedColors::Magenta,
            AcceptedColors::Magenta => AcceptedColors::Cyan,
            AcceptedColors::Cyan => AcceptedColors::Gray,
            AcceptedColors::Gray => AcceptedColors::DarkGray,
            AcceptedColors::DarkGray => AcceptedColors::LightRed,
            AcceptedColors::LightRed => AcceptedColors::LightGreen,
            AcceptedColors::LightGreen => AcceptedColors::LightYellow,
            AcceptedColors::LightYellow => AcceptedColors::LightBlue,
            AcceptedColors::LightBlue => AcceptedColors::LightMagenta,
            AcceptedColors::LightMagenta => AcceptedColors::LightCyan,
            AcceptedColors::LightCyan => AcceptedColors::Black,
        }
    }

    pub fn previous_color(&self) -> AcceptedColors {
        match self {
            AcceptedColors::Black => AcceptedColors::LightCyan,
            AcceptedColors::Red => AcceptedColors::Black,
            AcceptedColors::Green => AcceptedColors::Red,
            AcceptedColors::Yellow => AcceptedColors::Green,
            AcceptedColors::Blue => AcceptedColors::Yellow,
            AcceptedColors::Magenta => AcceptedColors::Blue,
            AcceptedColors::Cyan => AcceptedColors::Magenta,
            AcceptedColors::Gray => AcceptedColors::Cyan,
            AcceptedColors::DarkGray => AcceptedColors::Gray,
            AcceptedColors::LightRed => AcceptedColors::DarkGray,
            AcceptedColors::LightGreen => AcceptedColors::LightRed,
            AcceptedColors::LightYellow => AcceptedColors::LightGreen,
            AcceptedColors::LightBlue => AcceptedColors::LightYellow,
            AcceptedColors::LightMagenta => AcceptedColors::LightBlue,
            AcceptedColors::LightCyan => AcceptedColors::LightMagenta,
        }
    }
}

impl fmt::Display for AcceptedColors {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AcceptedColors::Black => write!(f, "Black"),
            AcceptedColors::Red => write!(f, "Red"),
            AcceptedColors::Green => write!(f, "Green"),
            AcceptedColors::Yellow => write!(f, "Yellow"),
            AcceptedColors::Blue => write!(f, "Blue"),
            AcceptedColors::Magenta => write!(f, "Magenta"),
            AcceptedColors::Cyan => write!(f, "Cyan"),
            AcceptedColors::Gray => write!(f, "Gray"),
            AcceptedColors::DarkGray => write!(f, "DarkGray"),
            AcceptedColors::LightRed => write!(f, "LightRed"),
            AcceptedColors::LightGreen => write!(f, "LightGreen"),
            AcceptedColors::LightYellow => write!(f, "LightYellow"),
            AcceptedColors::LightBlue => write!(f, "LightBlue"),
            AcceptedColors::LightMagenta => write!(f, "LightMagenta"),
            AcceptedColors::LightCyan => write!(f, "LightCyan"),
        }
    }
}

impl FromStr for AcceptedColors {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Black" => Ok(AcceptedColors::Black),
            "Red" => Ok(AcceptedColors::Red),
            "Green" => Ok(AcceptedColors::Green),
            "Yellow" => Ok(AcceptedColors::Yellow),
            "Blue" => Ok(AcceptedColors::Blue),
            "Magenta" => Ok(AcceptedColors::Magenta),
            "Cyan" => Ok(AcceptedColors::Cyan),
            "Gray" => Ok(AcceptedColors::Gray),
            "DarkGray" => Ok(AcceptedColors::DarkGray),
            "LightRed" => Ok(AcceptedColors::LightRed),
            "LightGreen" => Ok(AcceptedColors::LightGreen),
            "LightYellow" => Ok(AcceptedColors::LightYellow),
            "LightBlue" => Ok(AcceptedColors::LightBlue),
            "LightMagenta" => Ok(AcceptedColors::LightMagenta),
            "LightCyan" => Ok(AcceptedColors::LightCyan),
            _ => Ok(AcceptedColors::Green),
        }
    }
}

pub fn get_background_color(darkmode: bool) -> Color {
    if darkmode {
        Color::Black
    } else {
        Color::White
    }
}

pub fn get_foreground_color(darkmode: bool) -> Color {
    if darkmode {
        Color::White
    } else {
        Color::Black
    }
}

pub fn get_active_color(colorstr: &str) -> Color {
    match colorstr.to_lowercase().as_str() {
        "red" => Color::Red,
        "yellow" => Color::Yellow,
        "green" => Color::Green,
        "blue" => Color::Blue,
        "black" => Color::Black,
        "white" => Color::White,
        "lightgreen" => Color::LightGreen,
        "lightblue" => Color::LightBlue,
        "lightred" => Color::LightRed,
        "lightcyan" => Color::LightCyan,
        "lightmagenta" => Color::LightMagenta,
        "lightyellow" => Color::LightYellow,
        "gray" => Color::Gray,
        "darkgray" => Color::DarkGray,
        "magenta" => Color::Magenta,
        "cyan" => Color::Cyan,
        _ => Color::Green,
    }
}
