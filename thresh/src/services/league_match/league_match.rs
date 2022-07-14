use std::{thread, time::Duration};

use diesel::{PgConnection, QueryDsl};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

use crate::{
    models::{
        league_match::{
            augments::MatchParticipantAugment,
            dto::MatchDto,
            league_match::{Match, NewMatch},
            match_participant::{self, MatchParticipant, MatchParticipantFull},
            traits::MatchParticipantTrait,
            units::{MatchParticipantUnit, MatchParticipantUnitFull},
        },
        regions::{Regions, SubRegions},
        summoner::Summoner,
    },
    pool::get_connection_pool,
    schema::matches,
    services::get_api_key,
};

use super::{match_participant::create_match_participants, request::RequestOptions};

pub fn get_match_participants(match_id: &str, conn: &PgConnection) -> Vec<MatchParticipantFull> {
    use crate::diesel::BelongingToDsl;
    use crate::diesel::RunQueryDsl;

    let league_match = matches::table.find(match_id).first::<Match>(conn);

    match league_match {
        Ok(league_match) => {
            println!("{:?}", league_match);
            let participants = match_participant::MatchParticipant::belonging_to(&league_match)
                .load::<MatchParticipant>(conn)
                .expect("Error loading participants");

            let mut response = Vec::new();

            participants.iter().for_each(|participant| {
                // let summoner = Summoner::belonging_to(&participant).first::<Summoner>(conn);
                // let augments = match_participant::augments::belonging_to(&participant)
                //     .load::<match_participant::MatchParticipantAugment>(conn)
                //     .expect("Error loading augments");
                // let traits = match_participant::MatchParticipantTrait::belonging_to(&participant)
                //     .load::<match_participant::MatchParticipantTrait>(conn)
                //     .expect("Error loading traits");
                // let units = match_participant::MatchParticipantUnitFull::belonging_to(&participant)
                //     .load::<match_participant::MatchParticipantUnitFull>(conn)
                //     .expect("Error loading units");

                let augments = MatchParticipantAugment::belonging_to(participant)
                    .load::<MatchParticipantAugment>(conn)
                    .expect("Error loading participants");

                let traits = MatchParticipantTrait::belonging_to(participant)
                    .load::<MatchParticipantTrait>(conn)
                    .expect("Error loading participants");

                let units = MatchParticipantUnit::belonging_to(participant)
                    .load::<MatchParticipantUnit>(conn)
                    .expect("Error loading participants");

                let full_participant = MatchParticipantFull {
                    id: participant.id,
                    match_id: participant.match_id.to_string(),
                    summoner_id: participant.summoner_id.to_string(),
                    gold_left: participant.gold_left,
                    level: participant.level,
                    placement: participant.placement,
                    last_round: participant.last_round,
                    // summoner: summoner.unwrap(),
                    augments,
                    traits,
                    units,
                };
                response.push(full_participant);
            });

            response
        }
        Err(e) => {
            println!("Error loading league: {}", e);
            Vec::new()
        }
    }
}

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
                match_request_options.summoner_puuid = Some(puuid.clone());
                let fetched_match = fetch_match(match_request_options, conn);

                match fetched_match {
                    Ok(_) => {
                        println!("Fetched match: {}", m);
                    }
                    Err(e) => println!("Could not fetch match: {}", e),
                }
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

    // Only get the first 5 matches, or it'll fetch too much data
    match match_ids {
        Ok(ids) => {
            if ids.len() > 5 {
                let match_ids: Vec<String> = ids.iter().take(5).cloned().collect();
                Ok(match_ids)
            } else {
                Ok(ids)
            }
        }
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

            let mut league_match = NewMatch::from(league_match_dto);
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
