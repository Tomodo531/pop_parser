use select::{document::Document, predicate::Class};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct GameInfo {
    team_1_rounds: i32,
    team_2_rounds: i32,
    community: String,
    map: String,
    region: String,
    date: String,
    time: String,
    mode: String,
}

pub fn get_game_info(document: &Document) -> GameInfo {
    let header = document.find(Class("cUydK")).next().unwrap();

    let rounds: Vec<String> = header
        .find(Class("text-7xl"))
        .map(|rounds| rounds.text())
        .collect();

    let community = header.find(Class("eKiPsS")).next().unwrap().text();

    let general_info: Vec<String> = header
        .find(Class("text-gray-400"))
        .next()
        .unwrap()
        .text()
        .split(" · ")
        .map(|item| item.to_string())
        .collect();

    GameInfo {
        team_1_rounds: rounds[0].parse().unwrap(),
        team_2_rounds: rounds[1].parse().unwrap(),
        community,
        map: general_info[0].parse().unwrap(),
        region: general_info[0].parse().unwrap(),
        date: general_info[0].parse().unwrap(),
        time: general_info[0].parse().unwrap(),
        mode: general_info[0].parse().unwrap(),
    }
}
