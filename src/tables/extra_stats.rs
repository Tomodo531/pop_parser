use select::document::Document;
use serde::{Deserialize, Serialize};

use crate::tables::table_scraper::{self, Team};

use super::stats::{parse_stat, StatType, Stats, StatsTrait};

pub type ExtraStats = Vec<PlayerExtraStats>;

#[derive(Serialize, Deserialize, Debug)]
pub struct PlayerExtraStats {
    name: String,
    fk: StatType,
    fd: StatType,
    sk: StatType,
    sd: StatType,
    pk: StatType,
    pd: StatType,
    nk: StatType,
    nd: StatType,
    kare: StatType,
    dare: StatType,
    sr: StatType,
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
            fk: parse_stat(&player[1]),
            fd: parse_stat(&player[2]),
            sk: parse_stat(&player[3]),
            sd: parse_stat(&player[4]),
            pk: parse_stat(&player[5]),
            pd: parse_stat(&player[6]),
            nk: parse_stat(&player[7]),
            nd: parse_stat(&player[8]),
            kare: parse_stat(&player[9]),
            dare: parse_stat(&player[10]),
            sr: parse_stat(&player[11]),
        }
    }
}
