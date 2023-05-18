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
