use select::document::Document;
use serde::{Deserialize, Serialize};

use crate::table_scraper::Team;

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
