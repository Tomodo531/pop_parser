use select::document::Document;
use serde::{Deserialize, Serialize};

use crate::tables::table_scraper::{self, Team};

use super::stats::{parse_stat, StatType, Stats, StatsTrait};

pub type FlashStats = Vec<PlayerFlashStats>;

#[derive(Serialize, Deserialize, Debug)]
pub struct PlayerFlashStats {
    name: String,
    fa: StatType,
    ft: StatType,
    feh: StatType,
    fep: StatType,
    fed: StatType,
    fth: StatType,
    ftp: StatType,
    ftd: StatType,
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
            fa: parse_stat(&player[1]),
            ft: parse_stat(&player[2]),
            feh: parse_stat(&player[3]),
            fep: parse_stat(&player[4]),
            fed: parse_stat(&player[5]),
            fth: parse_stat(&player[6]),
            ftp: parse_stat(&player[7]),
            ftd: parse_stat(&player[8]),
        }
    }
}
