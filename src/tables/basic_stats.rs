use select::document::Document;
use serde::{Deserialize, Serialize};

use crate::table_scraper::{self, Team};

use super::stats::{Stats, StatsTrait};

pub type BasicStats = Vec<PlayerBasicStats>;

#[derive(Serialize, Deserialize, Debug)]
pub struct PlayerBasicStats {
    name: String,
    k: i32,
    a: i32,
    d: i32,
    adr: f32,
    hltv: f32,
    ck: i32,
    bp: i32,
    bd: i32,
    hs: f32,
    rp: i32,
}

impl StatsTrait<PlayerBasicStats> for Stats<PlayerBasicStats> {
    const TABLE_NAME: &'static str = "BASIC STATS";

    fn get_stats(document: &Document) -> Stats<BasicStats> {
        let (team_1, team_2) = table_scraper::get_table(document, Self::TABLE_NAME);

        Stats {
            team_1: Self::get_player_stats(team_1),
            team_2: Self::get_player_stats(team_2),
        }
    }

    fn get_player_stats(team: Team) -> BasicStats {
        let mut basic_stats: BasicStats = vec![];
        for player in team {
            basic_stats.push(Self::create_player_stats(player));
        }

        basic_stats
    }

    fn create_player_stats(player: Vec<String>) -> PlayerBasicStats {
        PlayerBasicStats {
            name: player[0].parse().unwrap(),
            k: player[1].parse().unwrap(),
            a: player[2].parse().unwrap(),
            d: player[3].parse().unwrap(),
            adr: player[4].parse().unwrap(),
            hltv: player[5].parse().unwrap(),
            ck: player[6].parse().unwrap(),
            bp: player[7].parse().unwrap(),
            bd: player[8].parse().unwrap(),
            hs: player[9].parse().unwrap(),
            rp: player[10].parse().unwrap(),
        }
    }
}
