use select::document::Document;

use crate::table::{self, Team};

pub type ExtraStats = Vec<PlayerExtraStats>;

const TABLE_NAME: &str = "EXTRA STATS";

use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug)]
pub struct PlayerExtraStats {
    name: String,
    fk: i32,
    fd: i32,
    sk: i32,
    sd: i32,
    pk: i32,
    pd: i32,
    nk: i32,
    nd: i32,
    kare: i32,
    dare: i32,
    sr: i32,
}

pub fn get_extra_stats(document: &Document) -> (ExtraStats, ExtraStats) {
    let (team_1, team_2) = table::get_table(document, TABLE_NAME);

    let team_1_extra_stats: ExtraStats = get_player_stats(team_1);
    let team_2_extra_stats: ExtraStats = get_player_stats(team_2);

    (team_1_extra_stats, team_2_extra_stats)
}

fn get_player_stats(team: Team) -> ExtraStats {
    let mut extra_stats: ExtraStats = vec![];

    for player in team {
        let player_extra_stats = PlayerExtraStats {
            name: player[0].parse().unwrap(),
            fk: player[1].parse().unwrap(),
            fd: player[2].parse().unwrap(),
            sk: player[3].parse().unwrap(),
            sd: player[4].parse().unwrap(),
            pk: player[5].parse().unwrap(),
            pd: player[6].parse().unwrap(),
            nk: player[7].parse().unwrap(),
            nd: player[8].parse().unwrap(),
            kare: player[9].parse().unwrap(),
            dare: player[10].parse().unwrap(),
            sr: player[11].parse().unwrap(),
        };

        extra_stats.push(player_extra_stats);
    }

    extra_stats
}
