use select::document::Document;
use serde::{Deserialize, Serialize};

use crate::tables::table_scraper::{self, Team};

use super::stats::{Stats, StatsTrait, parse_stat, StatType};

pub type KillsStats = Vec<PlayerKillsStats>;

#[derive(Serialize, Deserialize, Debug)]
pub struct PlayerKillsStats {
    name: String,
    awp: StatType,
    k1: StatType,
    k2: StatType,
    k3: StatType,
    k4: StatType,
    k5: StatType,
    v1v1: StatType,
    v2: StatType,
    v3: StatType,
    v4: StatType,
    v5: StatType,
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
            awp: parse_stat(&player[1]),
            k1: parse_stat(&player[2]),
            k2: parse_stat(&player[3]),
            k3: parse_stat(&player[4]),
            k4: parse_stat(&player[5]),
            k5: parse_stat(&player[6]),
            v1v1: parse_stat(&player[7]),
            v2: parse_stat(&player[8]),
            v3: parse_stat(&player[9]),
            v4: parse_stat(&player[10]),
            v5: parse_stat(&player[11]),
        }
    }
}
