use std::{thread, time::Duration};

use diesel::PgConnection;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

use crate::{
    models::{
        league::League,
        league_match::{Match, MatchDto, MatchDtoParticipant, MatchParticipant},
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
            println!("Fetching matches for {}", puuid);
            let match_ids: Vec<String> =
                fetch_summoner_match_ids(puuid, reg).expect("Could not fetch matches");

            let region_string = String::from(reg);

            match_ids.iter().for_each(|m| {
                fetch_match(&region_string, m, conn);
            });
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
) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let region_string = String::from(region);
    let query_url = format!(
        "https://{}.api.riotgames.com/tft/match/v1/matches/by-puuid/{}/ids?start=0&count=20&api_key={}",
        region_string,
        puuid,
        get_api_key()
    );

    let match_ids = reqwest::get(query_url).await?.json::<Vec<String>>().await;
    thread::sleep(Duration::from_millis(1000));

    match match_ids {
        Ok(ids) => Ok(ids),
        Err(e) => {
            println!("{}", e);
            Ok(Vec::new())
        }
    }
}

#[tokio::main]
async fn fetch_match(
    reg: &str,
    match_id: &str,
    conn: &PgConnection,
) -> Result<(), Box<dyn std::error::Error>> {
    let query_url = format!(
        "https://{}.api.riotgames.com/tft/match/v1/matches/{}?api_key={}",
        reg,
        match_id,
        get_api_key()
    );

    let league_match_dto = reqwest::get(query_url).await?.json::<MatchDto>().await;
    thread::sleep(Duration::from_millis(1000));

    match league_match_dto {
        Ok(league_match_dto) => {
            let participants = league_match_dto.clone().info.participants;

            let mut league_match = Match::from(league_match_dto);
            league_match.region = Some(String::from(reg));

            league_match.create(conn);

            let match_id = String::from(&league_match.match_id);

            create_match_participants(participants, match_id, conn);

            Ok(())
        }
        Err(e) => {
            println!("{}", e);
            Ok(())
        }
    }
}

fn create_match_participants(
    participants: Vec<MatchDtoParticipant>,
    match_id: String,
    conn: &PgConnection,
) {
    for participant_dto in participants {
        let mut participant = MatchParticipant::from(participant_dto);

        let summoner = Summoner::find_by_puuid(&participant.match_id, conn);

        if summoner.is_none() {
            return;
        }

        participant.match_id = match_id.to_string();
        participant.create(conn);
    }
}
