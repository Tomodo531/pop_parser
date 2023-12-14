use crate::error::AppError;
use crate::game_info::{get_game_info, GameInfo};
use crate::tables::basic_stats::{BasicStats, PlayerBasicStats};
use crate::tables::extra_stats::{ExtraStats, PlayerExtraStats};
use crate::tables::flash_stats::{FlashStats, PlayerFlashStats};
use crate::tables::kills::{KillsStats, PlayerKillsStats};
use crate::tables::shots_fired::{PlayerShotsFiredStats, ShotsFiredStats};
use crate::tables::stats::{Stats, StatsTrait};
use anyhow::Result;
use axum::{debug_handler, Json};
use select::document::Document;
use serde::{Deserialize, Serialize};

use crate::utils::get_match_document::get_match_document;
use axum::extract::Path;

pub mod basic_stats;
pub mod extra_stats;
pub mod flash_stats;
pub mod kills;
pub mod shots_fired;
pub mod stats;
pub mod table_scraper;

#[derive(Serialize, Deserialize, Debug)]
pub struct MatchData {
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

#[debug_handler]
pub async fn get_match_data(Path(match_id): Path<i32>) -> Result<Json<MatchData>, AppError> {
    let document = get_match_document(match_id).await?;
    let (team_1, team_2) = get_teams(&document);
    let game_info = get_game_info(&document);

    let match_data = MatchData {
        game_info,
        team_1,
        team_2,
    };

    Ok(Json(match_data))
}

fn get_teams(document: &Document) -> (Team, Team) {
    let basic_stats = Stats::<PlayerBasicStats>::get_stats(document);
    let flash_stats = Stats::<PlayerFlashStats>::get_stats(document);
    let kills = Stats::<PlayerKillsStats>::get_stats(document);
    let shots_fired = Stats::<PlayerShotsFiredStats>::get_stats(document);
    let extra_stats = Stats::<PlayerExtraStats>::get_stats(document);

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
