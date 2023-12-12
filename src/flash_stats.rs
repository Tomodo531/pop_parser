use select::document::Document;

use crate::table::{self, Team};

pub type FlashStats = Vec<PlayerFlashStats>;

const TABLE_NAME: &str = "FLASH STATS";

use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug)]
pub struct PlayerFlashStats {
    name: String,
    fa: i32,
    ft: i32,
    feh: i32,
    fep: String,
    fed: f32,
    fth: i32,
    ftp: String,
    ftd: f32,
}

pub fn get_flash_stats(document: &Document) -> (FlashStats, FlashStats) {
    let (team_1, team_2) = table::get_table(document, TABLE_NAME);

    let team_1_flash_stats: FlashStats = get_player_stats(team_1);
    let team_2_flash_stats: FlashStats = get_player_stats(team_2);

    (team_1_flash_stats, team_2_flash_stats)
}

fn get_player_stats(team: Team) -> FlashStats {
    let mut flash_stats: FlashStats = vec![];

    for player in team {
        let player_flash_stats = PlayerFlashStats {
            name: player[0].parse().unwrap(),
            fa: player[1].parse().unwrap(),
            ft: player[2].parse().unwrap(),
            feh: player[3].parse().unwrap(),
            fep: player[4].parse().unwrap(),
            fed: player[5].parse().unwrap(),
            fth: player[6].parse().unwrap(),
            ftp: player[7].parse().unwrap(),
            ftd: player[8].parse().unwrap(),
        };

        flash_stats.push(player_flash_stats);
    }

    flash_stats
}
