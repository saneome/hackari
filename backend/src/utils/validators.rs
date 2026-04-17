use once_cell::sync::Lazy;
use regex::Regex;

static EMAIL_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap());

pub static LOCATION_TYPE_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^(online|offline|hybrid)$").unwrap());

pub fn validate_email(email: &str) -> bool {
    EMAIL_REGEX.is_match(email)
}

pub fn validate_password(password: &str) -> Result<(), &'static str> {
    if password.len() < 8 {
        return Err("Password must be at least 8 characters long");
    }
    if !password.chars().any(|c| c.is_ascii_uppercase()) {
        return Err("Password must contain at least one uppercase letter");
    }
    if !password.chars().any(|c| c.is_ascii_lowercase()) {
        return Err("Password must contain at least one lowercase letter");
    }
    if !password.chars().any(|c| c.is_ascii_digit()) {
        return Err("Password must contain at least one digit");
    }
    Ok(())
}

pub fn validate_name(name: &str) -> Result<(), &'static str> {
    if name.len() < 2 {
        return Err("Name must be at least 2 characters long");
    }
    if name.len() > 100 {
        return Err("Name must be at most 100 characters long");
    }
    Ok(())
}
