use diesel::PgConnection;

use crate::models::league::League;

use super::get_api_key;

pub fn create_matches_service() {
    let regions: [&str; 11] = [
        "BR1", "EUN1", "EUW1", "JP1", "KR", "LA1", "LA2", "NA1", "OC1", "TR1", "RU",
    ];

    for region in regions.iter() {
        for tier in tiers.iter() {
            fetch_region_summoners(region, conn).unwrap();
        }
    }
}

// PRIVATE

fn fetch_region_summoners(reg: &str, conn: &PgConnection) -> () {
    let summoners = Summoner::fetch_by_region(reg, conn);

    for summoner in summoner.iter() {}
}

#[tokio::main]
async fn fetch_summoner_match_ids(
    summoner_id: &str,
    conn: &PgConnection,
) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let query_url = format!(
        "https://{}.api.riotgames.com/tft/match/v1/matches/by-summoner/{}?api_key={}",
        reg,
        summoner_id,
        get_api_key()
    );
    println!("Fetching summoner match ids: {}", query_url);

    let mut match_ids = reqwest::get(query_url).await?.json::<Vec<String>>().await?;

    Ok(match_ids)
}

#[tokio::main]
async fn fetch_match(
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
