use anyhow::Result;
use basic_stats::BasicStats;
use extra_stats::ExtraStats;
use flash_stats::FlashStats;
use game_info::GameInfo;
use kills::KillsStats;
use select::document::Document;
use serde::{Deserialize, Serialize};
use shots_fired::ShotsFiredStats;

mod basic_stats;
mod extra_stats;
mod flash_stats;
mod game_info;
mod kills;
mod shots_fired;
mod table;

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
    println!("{:?}", serde_json::json!(&match_data));

    Ok(())
}

fn get_teams(document: &Document) -> (Team, Team) {
    let (team_1_basic, team_2_basic) = basic_stats::get_basic_stats(document);
    let (team_1_flash, team_2_flash) = flash_stats::get_flash_stats(document);
    let (team_1_kills, team_2_kills) = kills::get_kills_stats(document);
    let (team_1_shots, team_2_shots) = shots_fired::get_shots_fired_stats(document);
    let (team_1_extra, team_2_extra) = extra_stats::get_extra_stats(document);

    let team_1 = Team {
        basic_stats: team_1_basic,
        flash_stats: team_1_flash,
        kills: team_1_kills,
        shots_fired: team_1_shots,
        extra_stats: team_1_extra,
    };

    let team_2 = Team {
        basic_stats: team_2_basic,
        flash_stats: team_2_flash,
        kills: team_2_kills,
        shots_fired: team_2_shots,
        extra_stats: team_2_extra,
    };

    (team_1, team_2)
}
