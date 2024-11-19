use std::io::Error;


use std::fmt;

#[derive(Debug)]
pub enum LoginError {
    InvalidCode,
    InvalidToken,
    InvalidCredentials,
    Unknown,
}

impl std::error::Error for LoginError {}

impl fmt::Display for LoginError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LoginError::InvalidCode => write!(f, "Invalid login code provided"),
            LoginError::InvalidToken => write!(f, "Invalid token provided"),
            LoginError::InvalidCredentials => write!(f, "Invalid credentials provided"),
            LoginError::Unknown => write!(f, "Unknown error"),
        }
    }
}

fn login_with_unique_code(unique_code: &str) -> Result<String, Error> {
    Ok(unique_code.to_string())
}