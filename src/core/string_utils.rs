pub fn is_none_or_whitespace(input: Option<&str>) -> bool {
    match input {
        Some(s) => s.trim().is_empty(),
        None => true,
    }
}

pub fn is_whitespace(input: &str) -> bool {
    input.trim().is_empty()
}
