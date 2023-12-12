use select::document::Document;

use crate::table::{self, Team};

pub type KillsStats = Vec<PlayerKillsStats>;

const TABLE_NAME: &str = "KILLS";

use serde::{Deserialize, Serialize};
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

pub fn get_kills_stats(document: &Document) -> (KillsStats, KillsStats) {
    let (team_1, team_2) = table::get_table(document, TABLE_NAME);

    let team_1_kills_stats: KillsStats = get_player_stats(team_1);
    let team_2_kills_stats: KillsStats = get_player_stats(team_2);

    (team_1_kills_stats, team_2_kills_stats)
}

fn get_player_stats(team: Team) -> KillsStats {
    let mut kills_stats: KillsStats = vec![];

    for player in team {
        let player_kills_stats = PlayerKillsStats {
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
        };

        kills_stats.push(player_kills_stats);
    }

    kills_stats
}
