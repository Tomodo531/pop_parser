use select::document::Document;
use serde::{Deserialize, Serialize};

use crate::{
    table::{self, Team},
    Stat,
};

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

pub fn get_basic_stats(document: &Document) -> Stat<BasicStats> {
    let (team_1, team_2) = table::get_table(document, TABLE_NAME);

    Stat {
        team_1: get_player_stats(team_1),
        team_2: get_player_stats(team_2),
    }
}

fn get_player_stats(team: Team) -> Vec<PlayerBasicStats> {
    let mut basic_stats: Vec<PlayerBasicStats> = vec![];
    for player in team {
        basic_stats.push(create_player_basic_stats(player));
    }

    basic_stats
}

fn create_player_basic_stats(player: Vec<String>) -> PlayerBasicStats {
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
