use crate::tables::table_scraper::Team;
use select::document::Document;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum StatType {
    Float(f32),
    Int(i32),
    Percentage(String),
    NaN(String),
}

pub fn parse_stat(input: &str) -> StatType {
    // Try parsing as f32
    if let Ok(float_val) = input.parse::<f32>() {
        return StatType::Float(float_val);
    }

    // Try parsing as i32
    if let Ok(int_val) = input.parse::<i32>() {
        return StatType::Int(int_val);
    }

    // Check if it ends with "%", then parse as Percentage
    if input.ends_with('%') {
        let value = input.trim_end_matches('%').to_string();
        return StatType::Percentage(value);
    }

    // If none of the above conditions are met, treat it as NaN
    StatType::NaN(String::from("?"))
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Stats<T> {
    pub team_1: T,
    pub team_2: T,
}

pub trait StatsTrait<T> {
    const TABLE_NAME: &'static str;
    fn get_stats(document: &Document) -> Stats<Vec<T>>;
    fn get_player_stats(team: Team) -> Vec<T>;
    fn create_player_stats(player: Vec<String>) -> T;
}
