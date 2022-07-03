use diesel::PgConnection;

use crate::models::league_match::{
    dto::MatchDtoParticipantTraits, traits::NewMatchParticipantTrait,
};

pub fn create_participant_traits(
    traits: Vec<MatchDtoParticipantTraits>,
    participant_id: i32,
    conn: &PgConnection,
) -> () {
    for unit_trait in traits {
        let match_participant_trait = NewMatchParticipantTrait {
            match_participant_id: participant_id,
            trait_id: unit_trait.name,
            num_units: unit_trait.num_units,
            tier_current: unit_trait.tier_current,
            tier_total: unit_trait.tier_total,
            style: unit_trait.style,
        };

        match_participant_trait.create(conn);
    }
}
