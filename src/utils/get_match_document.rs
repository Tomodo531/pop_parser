use anyhow::Result;
use select::document::Document;

pub async fn get_match_document(match_id: i32) -> Result<Document> {
    let url: String = format!("https://popflash.site/match/{}", match_id);
    println!("Fetching {}", url);

    let body: String = reqwest::get(url).await?.text().await?;

    Ok(Document::from(body.as_ref()))
}
