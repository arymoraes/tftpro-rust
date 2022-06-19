use diesel::PgConnection;

use crate::models::league::League;

use super::get_api_key;

pub fn create_leagues_service(conn: &PgConnection) {
    let regions: [&str; 11] = [
        "BR1", "EUN1", "EUW1", "JP1", "KR", "LA1", "LA2", "NA1", "OC1", "TR1", "RU",
    ];
    let tiers: [&str; 3] = ["master", "grandmaster", "challenger"];

    for region in regions.iter() {
        for tier in tiers.iter() {
            fetch_league(region, tier, conn).unwrap();
        }
    }
}

// PRIVATE

#[tokio::main]
async fn fetch_league(
    reg: &str,
    tier: &str,
    conn: &PgConnection,
) -> Result<League, Box<dyn std::error::Error>> {
    let query_url = format!(
        "https://{}.api.riotgames.com/tft/league/v1/{}?api_key={}",
        reg,
        tier,
        get_api_key()
    );
    println!("Fetching league: {}", query_url);

    let mut league = reqwest::get(query_url).await?.json::<League>().await?;
    league.region = Some(reg.to_string());
    league.create(conn);

    Ok(league)
}
