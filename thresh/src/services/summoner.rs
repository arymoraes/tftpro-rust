use diesel::PgConnection;
use serde::Deserialize;

use crate::models::{self, league::League};

use super::get_api_key;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct ResponseSummoner {
    summoner_id: String,
}

#[derive(Deserialize, Debug)]
struct APIResponse {
    pub entries: Vec<ResponseSummoner>,
}

pub fn get_summoners_service(conn: &PgConnection) {
    let leagues = League::all(conn);

    for league in leagues {
        fetch_summoners_from_league(&league, conn).unwrap();
    }
}

#[tokio::main]
async fn fetch_summoners_from_league(
    league: &League,
    conn: &PgConnection,
) -> Result<(), Box<dyn std::error::Error>> {
    let league_region = league.region.as_ref().unwrap();

    let query_url = format!(
        "https://{}.api.riotgames.com/tft/league/v1/leagues/{}?api_key={}",
        league_region,
        league.league_id,
        get_api_key()
    );

    let league_entries = reqwest::get(query_url).await?.json::<APIResponse>().await?;

    for summoner in league_entries.entries {
        create_summoner(
            summoner.summoner_id,
            &league_region,
            &league.league_id,
            conn,
        )
        .await?;
    }

    Ok(())

    // now for each league we get all the summoners
}

async fn create_summoner(
    summoner_id: String,
    region: &str,
    league_id: &str,
    conn: &PgConnection,
) -> Result<(), Box<dyn std::error::Error>> {
    let query_url = format!(
        "https://{}.api.riotgames.com/tft/summoner/v1/summoners/{}?api_key={}",
        region,
        summoner_id,
        get_api_key()
    );

    let response = reqwest::get(query_url)
        .await?
        .json::<models::summoner::SummonerRaw>()
        .await?;

    let summoner = models::summoner::Summoner {
        id: response.id,
        name: response.name,
        region: Some(region.to_string()),
        league_id: league_id.to_string(),
        profile_icon_id: response.profile_icon_id,
        summoner_level: response.summoner_level,
        revision_date: response.revision_date,
        account_id: response.account_id,
        puuid: response.puuid,
    };

    summoner.create(conn);

    Ok(())
}
