pub fn encode(username: &str, password: Option<String>) -> String {
    format!(
        "Basic {}",
        base64::encode(&format!("{}:{}", username, password.unwrap_or_default())[..])
    )
}
