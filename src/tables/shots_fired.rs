use select::document::Document;
use serde::{Deserialize, Serialize};

use crate::tables::table_scraper::{self, Team};

use super::stats::{parse_stat, StatType, Stats, StatsTrait};

pub type ShotsFiredStats = Vec<PlayerShotsFiredStats>;

#[derive(Serialize, Deserialize, Debug)]
pub struct PlayerShotsFiredStats {
    name: String,
    rf: StatType,
    rh: StatType,
    hd: StatType,
    ut: StatType,
    lt: StatType,
    ra: StatType,
    la: StatType,
    rl: StatType,
    ll: StatType,
    dd: StatType,
}

impl StatsTrait<PlayerShotsFiredStats> for Stats<PlayerShotsFiredStats> {
    const TABLE_NAME: &'static str = "SHOTS FIRED";

    fn get_stats(document: &Document) -> Stats<ShotsFiredStats> {
        let (team_1, team_2) = table_scraper::get_table(document, Self::TABLE_NAME);

        Stats {
            team_1: Self::get_player_stats(team_1),
            team_2: Self::get_player_stats(team_2),
        }
    }

    fn get_player_stats(team: Team) -> ShotsFiredStats {
        let mut shots_fired_stats: ShotsFiredStats = vec![];

        for player in team {
            shots_fired_stats.push(Self::create_player_stats(player));
        }

        shots_fired_stats
    }

    fn create_player_stats(player: Vec<String>) -> PlayerShotsFiredStats {
        PlayerShotsFiredStats {
            name: player[0].parse().unwrap(),
            rf: parse_stat(&player[1]),
            rh: parse_stat(&player[2]),
            hd: parse_stat(&player[3]),
            ut: parse_stat(&player[4]),
            lt: parse_stat(&player[5]),
            ra: parse_stat(&player[6]),
            la: parse_stat(&player[7]),
            rl: parse_stat(&player[8]),
            ll: parse_stat(&player[9]),
            dd: parse_stat(&player[10]),
        }
    }
}
