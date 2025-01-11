pub fn sanitize_string(str: &str) -> &str {
    str.trim_matches(|c: char| c.is_whitespace() || c == char::from(0))
}
