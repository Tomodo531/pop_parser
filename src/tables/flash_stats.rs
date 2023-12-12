use select::document::Document;
use serde::{Deserialize, Serialize};

use crate::table_scraper::{self, Team};

use super::stats::{Stats, StatsTrait};

pub type FlashStats = Vec<PlayerFlashStats>;

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

impl StatsTrait<PlayerFlashStats> for Stats<PlayerFlashStats> {
    const TABLE_NAME: &'static str = "FLASH STATS";

    fn get_stats(document: &Document) -> Stats<FlashStats> {
        let (team_1, team_2) = table_scraper::get_table(document, Self::TABLE_NAME);

        Stats {
            team_1: Self::get_player_stats(team_1),
            team_2: Self::get_player_stats(team_2),
        }
    }

    fn get_player_stats(team: Team) -> FlashStats {
        let mut flash_stats: FlashStats = vec![];
        for player in team {
            flash_stats.push(Self::create_player_stats(player));
        }

        flash_stats
    }

    fn create_player_stats(player: Vec<String>) -> PlayerFlashStats {
        PlayerFlashStats {
            name: player[0].parse().unwrap(),
            fa: player[1].parse().unwrap(),
            ft: player[2].parse().unwrap(),
            feh: player[3].parse().unwrap(),
            fep: player[4].parse().unwrap(),
            fed: player[5].parse().unwrap(),
            fth: player[6].parse().unwrap(),
            ftp: player[7].parse().unwrap(),
            ftd: player[8].parse().unwrap(),
        }
    }
}
