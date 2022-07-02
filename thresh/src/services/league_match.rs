use std::{thread, time::Duration};

use diesel::PgConnection;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

use crate::{
    models::{
        league_match::{Match, MatchDto, MatchDtoParticipant, MatchParticipant},
        regions::{Regions, SubRegions},
        summoner::Summoner,
    },
    pool::get_connection_pool,
};

use super::{get_api_key, request::RequestOptions, summoner::create_summoner};

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

        let mut request_options = RequestOptions {
            sub_region: Some(s_region),
            league_id: None,
            summoner_puuid: None,
            match_id: None,
            region: None,
        };

        let summoners_puuids = fetch_region_summoners_puuids(&request_options, conn);

        request_options.region = Some(String::from(Regions::from(*sub_region)));

        summoners_puuids.iter().for_each(|puuid| {
            println!("Fetching matches for {}", puuid);
            let match_ids: Vec<String> =
                fetch_summoner_match_ids(puuid, &request_options).expect("Could not fetch matches");

            match_ids.iter().for_each(|m| {
                let mut match_request_options = request_options.clone();
                match_request_options.match_id = Some(m.clone());
                fetch_match(match_request_options, conn);
            });
        });
    });
}

fn fetch_region_summoners_puuids(options: &RequestOptions, conn: &PgConnection) -> Vec<String> {
    let sub_region = options.sub_region.as_ref().unwrap();

    Summoner::fetch_by_region(&sub_region, conn)
}

#[tokio::main]
async fn fetch_summoner_match_ids(
    puuid: &str,
    options: &RequestOptions,
) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let query_url = format!(
        "https://{}.api.riotgames.com/tft/match/v1/matches/by-puuid/{}/ids?start=0&count=20&api_key={}",
        options.region.as_ref().unwrap(),
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
    mut options: RequestOptions,
    conn: &PgConnection,
) -> Result<(), Box<dyn std::error::Error>> {
    let match_region = options.region.as_ref().unwrap();

    let query_url = format!(
        "https://{}.api.riotgames.com/tft/match/v1/matches/{}?api_key={}",
        match_region,
        options.match_id.unwrap(),
        get_api_key()
    );

    let league_match_dto = reqwest::get(query_url).await?.json::<MatchDto>().await;
    thread::sleep(Duration::from_millis(1000));

    match league_match_dto {
        Ok(league_match_dto) => {
            let participants = league_match_dto.clone().info.participants;

            let mut league_match = Match::from(league_match_dto);
            league_match.region = Some(match_region.to_string());

            league_match.create(conn);

            let match_id = String::from(&league_match.match_id);

            options.match_id = Some(match_id);

            create_match_participants(participants, options, conn).await;

            Ok(())
        }
        Err(e) => {
            println!("{}", e);
            Ok(())
        }
    }
}

async fn create_match_participants(
    participants: Vec<MatchDtoParticipant>,
    options: RequestOptions,
    conn: &PgConnection,
) {
    for participant_dto in participants {
        let mut participant = MatchParticipant::from(participant_dto);

        let summoner = Summoner::find_by_puuid(&participant.match_id, conn);

        if summoner.is_none() {
            let new_summoner = create_summoner(
                participant.match_id,
                &options.sub_region.as_ref().unwrap(),
                &options.league_id.as_ref().unwrap(),
                conn,
            )
            .await;

            match new_summoner {
                Ok(_) => {
                    println!("Created summoner");
                }
                Err(e) => {
                    println!("{}", e);
                    continue;
                }
            }
        }

        participant.match_id = options.match_id.as_ref().unwrap().to_string();
        participant.create(conn);
    }
}
