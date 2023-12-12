use select::document::Document;
use serde::{Deserialize, Serialize};

use crate::table_scraper::{self, Team};

use super::stats::{Stats, StatsTrait};

pub type ExtraStats = Vec<PlayerExtraStats>;

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

impl StatsTrait<PlayerExtraStats> for Stats<PlayerExtraStats> {
    const TABLE_NAME: &'static str = "EXTRA STATS";

    fn get_stats(document: &Document) -> Stats<ExtraStats> {
        let (team_1, team_2) = table_scraper::get_table(document, Self::TABLE_NAME);

        Stats {
            team_1: Self::get_player_stats(team_1),
            team_2: Self::get_player_stats(team_2),
        }
    }

    fn get_player_stats(team: Team) -> ExtraStats {
        let mut extra_stats: ExtraStats = vec![];
        for player in team {
            extra_stats.push(Self::create_player_stats(player));
        }

        extra_stats
    }

    fn create_player_stats(player: Vec<String>) -> PlayerExtraStats {
        PlayerExtraStats {
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
        }
    }
}
