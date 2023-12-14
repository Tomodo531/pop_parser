use anyhow::Result;
use axum::routing::get;
use axum::Router;

mod error;
mod game_info;
mod tables;
mod utils;

#[tokio::main]
async fn main() -> Result<()> {
    // build our application with a single route
    let app = Router::new().route("/match/:id", get(tables::get_match_data));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}
