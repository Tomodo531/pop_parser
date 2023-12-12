use select::document::Document;

use crate::table::{self, Team};

pub type ShotsFiredStats = Vec<PlayerShotsFiredStats>;

const TABLE_NAME: &str = "SHOTS FIRED";

use serde::{Deserialize, Serialize};
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

pub fn get_shots_fired_stats(document: &Document) -> (ShotsFiredStats, ShotsFiredStats) {
    let (team_1, team_2) = table::get_table(document, TABLE_NAME);

    let team_1_shots_fired_stats: ShotsFiredStats = get_player_stats(team_1);
    let team_2_shots_fired_stats: ShotsFiredStats = get_player_stats(team_2);

    (team_1_shots_fired_stats, team_2_shots_fired_stats)
}

fn get_player_stats(team: Team) -> ShotsFiredStats {
    let mut shots_fired_stats: ShotsFiredStats = vec![];

    for player in team {
        let player_shots_fired_stats = PlayerShotsFiredStats {
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
        };

        shots_fired_stats.push(player_shots_fired_stats);
    }

    shots_fired_stats
}
