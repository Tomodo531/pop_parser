use select::document::Document;
use serde::{Deserialize, Serialize};

use crate::table_scraper::{self, Team};

use super::stats::{Stats, StatsTrait};

pub type KillsStats = Vec<PlayerKillsStats>;

#[derive(Serialize, Deserialize, Debug)]
pub struct PlayerKillsStats {
    name: String,
    awp: i32,
    k1: i32,
    k2: i32,
    k3: i32,
    k4: i32,
    k5: i32,
    v1v1: i32,
    v2: i32,
    v3: i32,
    v4: i32,
    v5: i32,
}

impl StatsTrait<PlayerKillsStats> for Stats<PlayerKillsStats> {
    const TABLE_NAME: &'static str = "KILLS";

    fn get_stats(document: &Document) -> Stats<KillsStats> {
        let (team_1, team_2) = table_scraper::get_table(document, Self::TABLE_NAME);

        Stats {
            team_1: Self::get_player_stats(team_1),
            team_2: Self::get_player_stats(team_2),
        }
    }

    fn get_player_stats(team: Team) -> KillsStats {
        let mut kills_stats: KillsStats = vec![];

        for player in team {
            kills_stats.push(Self::create_player_stats(player));
        }

        kills_stats
    }

    fn create_player_stats(player: Vec<String>) -> PlayerKillsStats {
        PlayerKillsStats {
            name: player[0].parse().unwrap(),
            awp: player[1].parse().unwrap(),
            k1: player[2].parse().unwrap(),
            k2: player[3].parse().unwrap(),
            k3: player[4].parse().unwrap(),
            k4: player[5].parse().unwrap(),
            k5: player[6].parse().unwrap(),
            v1v1: player[7].parse().unwrap(),
            v2: player[8].parse().unwrap(),
            v3: player[9].parse().unwrap(),
            v4: player[10].parse().unwrap(),
            v5: player[11].parse().unwrap(),
        }
    }
}
