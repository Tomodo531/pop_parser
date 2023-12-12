use anyhow::Result;
use game_info::GameInfo;
use select::document::Document;
use serde::{Deserialize, Serialize};
use tables::basic_stats::{self, BasicStats};
use tables::extra_stats::{self, ExtraStats};
use tables::flash_stats::{self, FlashStats};
use tables::kills::{self, KillsStats};
use tables::shots_fired::{self, ShotsFiredStats};

mod game_info;
mod table_scraper;
mod tables;

#[derive(Serialize, Deserialize, Debug)]
pub struct Stats<T> {
    pub team_1: T,
    pub team_2: T,
}

#[derive(Serialize, Deserialize, Debug)]
struct MatchData {
    game_info: GameInfo,
    team_1: Team,
    team_2: Team,
}

#[derive(Serialize, Deserialize, Debug)]
struct Team {
    basic_stats: BasicStats,
    flash_stats: FlashStats,
    kills: KillsStats,
    shots_fired: ShotsFiredStats,
    extra_stats: ExtraStats,
}

#[tokio::main]
async fn main() -> Result<()> {
    let match_id = 1370415;
    let url = format!("https://popflash.site/match/{}", match_id);
    let body: String = reqwest::get(url).await?.text().await?;
    let document = Document::from(body.as_ref());

    let game_info = game_info::get_game_info(&document);
    let (team_1, team_2) = get_teams(&document);

    let match_data: MatchData = MatchData {
        game_info,
        team_1,
        team_2,
    };

    println!("{:?}", serde_json::to_string(&match_data));
    Ok(())
}

fn get_teams(document: &Document) -> (Team, Team) {
    let basic_stats = basic_stats::get_basic_stats(document);
    let flash_stats = flash_stats::get_flash_stats(document);
    let kills = kills::get_kills_stats(document);
    let shots_fired = shots_fired::get_shots_fired_stats(document);
    let extra_stats = extra_stats::get_extra_stats(document);

    let team_1 = Team {
        basic_stats: basic_stats.team_1,
        flash_stats: flash_stats.team_1,
        kills: kills.team_1,
        shots_fired: shots_fired.team_1,
        extra_stats: extra_stats.team_1,
    };

    let team_2 = Team {
        basic_stats: basic_stats.team_2,
        flash_stats: flash_stats.team_2,
        kills: kills.team_2,
        shots_fired: shots_fired.team_2,
        extra_stats: extra_stats.team_2,
    };

    (team_1, team_2)
}
