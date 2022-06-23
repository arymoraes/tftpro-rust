use diesel::PgConnection;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

use crate::{
    models::{
        league::League,
        regions::{Regions, SubRegions},
        summoner::Summoner,
    },
    pool::get_connection_pool,
};

use super::get_api_key;

pub fn create_matches_service() {
    let sub_regions: [SubRegions; 11] = [
        SubRegions::BR1,
        SubRegions::EUN1,
        SubRegions::EUW1,
        SubRegions::JP1,
        SubRegions::KR,
        SubRegions::LA1,
        SubRegions::LA2,
        SubRegions::NA1,
        SubRegions::OC1,
        SubRegions::TR1,
        SubRegions::RU,
    ];

    let pool = get_connection_pool();

    sub_regions.par_iter().for_each(|sub_region| {
        let pool = pool.clone();
        let conn = &mut pool.get().unwrap();

        let s_region = String::from(*sub_region);

        let summoners_puuids = fetch_region_summoners_puuids(&s_region, conn);

        let reg = Regions::from(*sub_region);

        summoners_puuids.iter().for_each(|puuid| {
            fetch_summoner_match_ids(puuid, reg, conn);
        });
    });
}

fn fetch_region_summoners_puuids(reg: &String, conn: &PgConnection) -> Vec<String> {
    Summoner::fetch_by_region(reg, conn)
}

#[tokio::main]
async fn fetch_summoner_match_ids(
    puuid: &str,
    region: Regions,
    conn: &PgConnection,
) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let region_string = String::from(region);
    let query_url = format!(
        "https://{}.api.riotgames.com/tft/match/v1/matches/by-puuid/{}/ids?start=0&count=20?api_key={}",
        // "https://{}.api.riotgames.com/tft/match/v1/matches/by-summoner/{}?api_key={}",
        region_string,
        puuid,
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
