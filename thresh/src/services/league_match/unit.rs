use diesel::PgConnection;

use crate::models::league_match::{dto::MatchDtoParticipantUnits, units::NewMatchParticipantUnit};

pub fn create_participant_units(
    units: Vec<MatchDtoParticipantUnits>,
    participant_id: i32,
    conn: &PgConnection,
) -> () {
    for unit in units {
        let match_participant_unit = NewMatchParticipantUnit {
            match_participant_id: participant_id,
            unit_id: unit.character_id,
            rarity: unit.rarity,
            tier: unit.tier,
        };

        let new_unit = match_participant_unit.create(conn);

        match new_unit {
            Ok(_) => (),
            Err(e) => {
                println!(
                    "Problem while creating match participant unit: {}. \n Participant ID: {},\n",
                    e, participant_id
                );
            }
        }
    }
}
