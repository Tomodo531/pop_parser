use select::document::Document;
use serde::{Deserialize, Serialize};

use crate::table::{self, Team};

pub type BasicStats = Vec<PlayerBasicStats>;

const TABLE_NAME: &str = "BASIC STATS";

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

pub fn get_basic_stats(document: &Document) -> (BasicStats, BasicStats) {
    let (team_1, team_2) = table::get_table(document, TABLE_NAME);

    let team_1_basic_stats: BasicStats = get_player_stats(team_1);
    let team_2_basic_stats: BasicStats = get_player_stats(team_2);

    (team_1_basic_stats, team_2_basic_stats)
}

fn get_player_stats(team: Team) -> BasicStats {
    let mut basic_stats: BasicStats = vec![];
    for player in team {
        let player_basic_stats = PlayerBasicStats {
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
        };

        basic_stats.push(player_basic_stats);
    }

    basic_stats
}
