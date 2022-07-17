use diesel::PgConnection;

use crate::models::league::League;
use colored::*;

use super::get_api_key;

pub fn create_leagues_service(conn: &PgConnection) {
    println!("{}", "Creating leagues...".green());

    let regions: [&str; 11] = [
        "BR1", "EUN1", "EUW1", "JP1", "KR", "LA1", "LA2", "NA1", "OC1", "TR1", "RU",
    ];
    let tiers: [&str; 1] = ["challenger"]; // We can add master and grandmaster here if we want to

    for region in regions.iter() {
        for tier in tiers.iter() {
            fetch_league(region, tier, conn);
        }
    }

    println!("{}", "Leagues created!".green());
}

// PRIVATE

#[tokio::main]
async fn fetch_league(reg: &str, tier: &str, conn: &PgConnection) -> () {
    let query_url = format!(
        "https://{}.api.riotgames.com/tft/league/v1/{}?api_key={}",
        reg,
        tier,
        get_api_key()
    );
    println!("Fetching league: {}", query_url);
    let request = reqwest::get(query_url).await;

    let response = match request {
        Ok(response) => response,
        Err(e) => {
            println!("Error while fetching league: {}", e.to_string().red());
            return;
        }
    };

    let dto: Result<League, reqwest::Error> = response.json().await;

    match dto {
        Ok(mut league) => {
            league.region = Some(reg.to_string());
            league.create(conn);
        }
        Err(e) => {
            println!("Error while parsing league: {}", e.to_string().red());
        }
    };
}
