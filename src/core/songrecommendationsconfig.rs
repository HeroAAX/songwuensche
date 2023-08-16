extern crate serde;
extern crate serde_json;

const CONFIG_FILE: &str = "./config.json";

use std::fs;

use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct SongRecommendationsConfig {
    pub channel_id: String,
    pub filter_urls: Vec<String>,
}

impl Default for SongRecommendationsConfig {
    fn default() -> Self {
        Self {
            channel_id: "heroaax".to_owned(),
            filter_urls: vec![
                "https://youtu.be/".to_owned(),
                "https://www.youtube.com/watch?v=".to_owned(),
            ],
        }
    }
}

pub fn load_config() -> SongRecommendationsConfig {
    match fs::read_to_string(CONFIG_FILE) {
        Ok(result) => match serde_json::from_str(&result) {
            Ok(s) => return s,
            Err(_) => return SongRecommendationsConfig::default(),
        },
        Err(_) => return SongRecommendationsConfig::default(),
    }
}
