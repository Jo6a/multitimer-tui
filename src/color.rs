use tui::style::Color;

pub fn get_background_color(darkmode: bool) -> Color {
    if darkmode {
        return Color::Black;
    } else {
        return Color::White;
    };
}

pub fn get_foreground_color(darkmode: bool) -> Color {
    if darkmode {
        return Color::White;
    } else {
        return Color::Black;
    };
}
