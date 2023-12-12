use select::document::Document;
use serde::{Deserialize, Serialize};

use crate::table_scraper::{self, Team};

use super::stats::{Stats, StatsTrait};

pub type ShotsFiredStats = Vec<PlayerShotsFiredStats>;

#[derive(Serialize, Deserialize, Debug)]
pub struct PlayerShotsFiredStats {
    name: String,
    rf: i32,
    rh: i32,
    hd: i32,
    ut: i32,
    lt: i32,
    ra: i32,
    la: i32,
    rl: i32,
    ll: i32,
    dd: i32,
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
            rf: player[1].parse().unwrap(),
            rh: player[2].parse().unwrap(),
            hd: player[3].parse().unwrap(),
            ut: player[4].parse().unwrap(),
            lt: player[5].parse().unwrap(),
            ra: player[6].parse().unwrap(),
            la: player[7].parse().unwrap(),
            rl: player[8].parse().unwrap(),
            ll: player[9].parse().unwrap(),
            dd: player[10].parse().unwrap(),
        }
    }
}
