use serde::Deserialize;
use std::collections::HashMap;
use std::fs;

#[derive(Debug, Deserialize, Clone)]
pub struct Question {
    pub id: String,
    pub label: String,
    pub placeholder: String,
    pub paragraph: bool,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Category {
    pub label: String,
    pub description: String,
    pub questions: Vec<Question>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub guild_id: u64,
    pub category_channel_id: u64,
    pub support_role_id: u64,
    pub categories: HashMap<String, Category>,
}

impl Config {
    pub fn load() -> anyhow::Result<Self> {
        let content = fs::read_to_string("config.json")?;
        let config: Config = serde_json::from_str(&content)?;
        Ok(config)
    }
}