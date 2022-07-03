use diesel::PgConnection;

use crate::models::{item::Item, league_match::augments::NewMatchParticipantAugment};

pub fn create_participant_augments(
    augments: Vec<String>,
    participant_id: i32,
    conn: &PgConnection,
) -> () {
    for augment in augments {
        let augment_id = Item::from_name_id(&augment, conn);

        if augment_id.is_none() {
            continue;
        }

        let match_participant_augment = NewMatchParticipantAugment {
            match_participant_id: participant_id,
            augment_id: augment_id.unwrap(),
        };

        match_participant_augment.create(conn);
    }
}
