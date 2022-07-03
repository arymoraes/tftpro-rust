use diesel::PgConnection;

use crate::models::league_match::{
    dto::MatchDtoParticipantUnits, items::NewMatchParticipantUnitItem,
    units::NewMatchParticipantUnit,
};

pub fn create_participant_units(
    units: Vec<MatchDtoParticipantUnits>,
    participant_id: i32,
    conn: &PgConnection,
) -> () {
    for unit in units {
        let unit_items_ids = unit.items;

        let match_participant_unit = NewMatchParticipantUnit {
            match_participant_id: participant_id,
            unit_id: unit.character_id,
            rarity: unit.rarity,
            tier: unit.tier,
        };

        let new_unit = match_participant_unit.create(conn);

        match new_unit {
            Ok(u) => {
                create_participant_unit_items(unit_items_ids, u.id, conn);
            }
            Err(e) => {
                println!(
                    "Problem while creating match participant unit: {}. \n Participant ID: {},\n",
                    e, participant_id
                );
            }
        }
    }
}

pub fn create_participant_unit_items(items: Vec<i32>, unit_id: i32, conn: &PgConnection) -> () {
    for item in items {
        let match_participant_unit_item = NewMatchParticipantUnitItem {
            match_participant_unit_id: unit_id,
            item_id: item,
        };

        match_participant_unit_item.create(conn);
    }
}
