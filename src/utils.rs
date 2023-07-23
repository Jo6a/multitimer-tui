use std::collections::HashMap;

pub fn reverse_bool(input: &str) -> String {
    let value = input.parse::<bool>().unwrap_or(false);
    (!value).to_string()
}

/// Default timer types and their colors
pub fn get_initial_timer_colors() -> HashMap<String, String> {
    HashMap::from([
        ("urgent".to_string(), "Red".to_string()),
        ("important".to_string(), "Blue".to_string()),
        ("casual".to_string(), "Green".to_string()),
        ("break".to_string(), "Yellow".to_string()),
        ("focus".to_string(), "Magenta".to_string()),
        ("fun".to_string(), "LightMagenta".to_string()),
        ("study".to_string(), "LightCyan".to_string()),
        ("deadline".to_string(), "LightRed".to_string()),
        ("coding".to_string(), "LightGreen".to_string()),
    ])
}
