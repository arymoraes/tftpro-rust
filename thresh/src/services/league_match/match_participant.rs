use std::{thread, time::Duration};

use colored::Colorize;
use diesel::PgConnection;

use crate::{
    models::{
        league_match::{dto::MatchDtoParticipant, match_participant::NewMatchParticipant},
        summoner::Summoner,
    },
    services::{
        league_match::{
            augment::create_participant_augments, traits::create_participant_traits,
            unit::create_participant_units,
        },
        summoner::create_summoner_from_puuid,
    },
};

use super::request::RequestOptions;

pub async fn create_match_participants(
    participants: Vec<MatchDtoParticipant>,
    options: RequestOptions,
    conn: &PgConnection,
) {
    for participant_dto in participants {
        let participant_augments = participant_dto.augments.clone();
        let participant_traits = participant_dto.traits.clone();
        let participant_units = participant_dto.units.clone();

        let mut participant = NewMatchParticipant::from(participant_dto);

        let summoner = Summoner::find_by_puuid(&participant.summoner_id, conn);
        println!("{:?}", summoner);
        let summoner_puuid = participant.summoner_id.clone();

        if summoner.unwrap() == 0 {
            let new_summoner = create_summoner_from_puuid(
                summoner_puuid,
                &options.sub_region.as_ref().unwrap(),
                None,
                conn,
            )
            .await;

            thread::sleep(Duration::from_millis(1000));

            match new_summoner {
                Ok(_) => {
                    println!("{}", String::from("Created summoner").bright_yellow());
                }
                Err(e) => {
                    println!("{}", e);
                    continue;
                }
            }
        }

        participant.match_id = options.match_id.as_ref().unwrap().to_string();
        let new_participant = participant.create(conn);

        match new_participant {
            Ok(p) => {
                println!("{}", String::from("Created participant").bright_yellow());
                create_participant_augments(participant_augments, p.id, conn);
                create_participant_traits(participant_traits, p.id, conn);
                create_participant_units(participant_units, p.id, conn);
            }
            Err(e) => {
                println!("{}", e);
                continue;
            }
        }
    }
}
