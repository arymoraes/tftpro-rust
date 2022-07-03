use std::{thread, time::Duration};

use colored::Colorize;
use diesel::PgConnection;

use crate::{
    models::{
        item::Item,
        league_match::{
            augments::NewMatchParticipantAugment, dto::MatchDtoParticipant,
            match_participant::NewMatchParticipant,
        },
        summoner::Summoner,
    },
    services::summoner::create_summoner_from_puuid,
};

use super::request::RequestOptions;

pub async fn create_match_participants(
    participants: Vec<MatchDtoParticipant>,
    options: RequestOptions,
    conn: &PgConnection,
) {
    for participant_dto in participants {
        let participant_augments = participant_dto.augments.clone();

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
                for augment in participant_augments {
                    let augment_id = Item::from_name_id(&augment, conn);

                    let match_participant_augment = NewMatchParticipantAugment {
                        match_participant_id: p.id,
                        augment_id: augment_id.id,
                    };

                    match_participant_augment.create(conn);
                }
            }
            Err(e) => {
                println!("{}", e);
                continue;
            }
        }
    }
}
