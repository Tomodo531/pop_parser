use select::document::Document;
use serde::{Deserialize, Serialize};

use crate::tables::table_scraper::{self, Team};

use super::stats::{Stats, StatsTrait, StatType, parse_stat};

pub type BasicStats = Vec<PlayerBasicStats>;

#[derive(Serialize, Deserialize, Debug)]
pub struct PlayerBasicStats {
    name: String,
    k: StatType,
    a: StatType,
    d: StatType,
    adr: StatType,
    hltv: StatType,
    ck: StatType,
    bp: StatType,
    bd: StatType,
    hs: StatType,
    rp: StatType,
}

impl StatsTrait<PlayerBasicStats> for Stats<PlayerBasicStats> {
    const TABLE_NAME: &'static str = "BASIC STATS";

    fn get_stats(document: &Document) -> Stats<BasicStats> {
        let (team_1, team_2) = table_scraper::get_table(document, Self::TABLE_NAME);

        Stats {
            team_1: Self::get_player_stats(team_1),
            team_2: Self::get_player_stats(team_2),
        }
    }

    fn get_player_stats(team: Team) -> BasicStats {
        let mut basic_stats: BasicStats = vec![];
        for player in team {
            basic_stats.push(Self::create_player_stats(player));
        }

        basic_stats
    }

    fn create_player_stats(player: Vec<String>) -> PlayerBasicStats {
        PlayerBasicStats {
            name: player[0].parse().unwrap(),
            k: parse_stat(&player[1]),
            a: parse_stat(&player[2]),
            d: parse_stat(&player[3]),
            adr: parse_stat(&player[4]),
            hltv: parse_stat(&player[5]),
            ck: parse_stat(&player[6]),
            bp: parse_stat(&player[7]),
            bd: parse_stat(&player[8]),
            hs: parse_stat(&player[9]),
            rp: parse_stat(&player[10]),
        }
    }
}
