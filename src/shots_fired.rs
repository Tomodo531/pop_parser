use select::document::Document;
use serde::{Deserialize, Serialize};

use crate::{table::{self, Team}, Stat};

pub type ShotsFiredStats = Vec<PlayerShotsFiredStats>;

const TABLE_NAME: &str = "SHOTS FIRED";

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

pub fn get_shots_fired_stats(document: &Document) -> Stat<ShotsFiredStats> {
    let (team_1, team_2) = table::get_table(document, TABLE_NAME);

    Stat {
        team_1: get_player_stats(team_1),
        team_2: get_player_stats(team_2),
    }
}

fn get_player_stats(team: Team) -> ShotsFiredStats {
    let mut shots_fired_stats: ShotsFiredStats = vec![];

    for player in team {
        shots_fired_stats.push(create_player_shots_fired_stats(player));
    }

    shots_fired_stats
}

fn create_player_shots_fired_stats(player: Vec<String>) -> PlayerShotsFiredStats {
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
