pub fn reverse_bool(input: &str) -> String {
    let value = input.parse::<bool>().unwrap_or(false);
    (!value).to_string()
}
