use tui::style::Color;

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
    match colorstr {
        "Red" => Color::Red,
        "Yellow" => Color::Yellow,
        "Green" => Color::Green,
        "Blue" => Color::Blue,
        "Black" => Color::Black,
        "White" => Color::White,
        "LightGreen" => Color::LightGreen,
        "LightBlue" => Color::LightBlue,
        "LightRed" => Color::LightRed,
        "LightCyan" => Color::LightCyan,
        "LightMagenta" => Color::LightMagenta,
        "LightYellow" => Color::LightYellow,
        "Gray" => Color::Gray,
        "DarkGray" => Color::DarkGray,
        "Magenta" => Color::Magenta,
        "Cyan" => Color::Cyan,
        _ => Color::Yellow
    }
}
