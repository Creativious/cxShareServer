use std::fs;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Config {
    address: String,
    port: u16
}

#[derive(Serialize, Deserialize)]
pub struct Database {
    pub address: String,
    pub port: u16,
    pub user: String,
    pub password: String,
    pub database: String
}

impl Database {
    pub fn new(address: String, port: u16, user: String, password: String, database: String) -> Database {
        Database { address, port, user, password, database }
    }
}

impl Config {
    pub fn new(address: String, port: u16) -> Config {
        Config { address, port }
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub fn address(&self) -> &str {
        &self.address
    }

    pub fn url(&self) -> String {
        format!("{}:{}", self.address, self.port)
    }

    pub fn from_path(path: &str) -> Option<Config> {
        let contents = fs::read_to_string(path).expect("Something went wrong reading the file");
        toml::from_str(&contents).unwrap_or(None)
    }
}