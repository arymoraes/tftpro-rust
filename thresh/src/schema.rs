table! {
    characters (character_id) {
        character_id -> Varchar,
        img -> Nullable<Varchar>,
        tier -> Nullable<Int4>,
        rarity -> Nullable<Int4>,
        display_name -> Varchar,
        square_icon_path -> Varchar,
    }
}

table! {
    items (id) {
        id -> Int4,
        name_id -> Varchar,
        img -> Nullable<Varchar>,
        loadouts_icon -> Varchar,
        guid -> Varchar,
    }
}

table! {
    leagues (league_id) {
        league_id -> Varchar,
        tier -> Varchar,
        queue -> Varchar,
        name -> Varchar,
        region -> Nullable<Varchar>,
    }
}

table! {
    matches (match_id) {
        match_id -> Varchar,
        game_datetime -> Int4,
        game_length -> Int4,
        tft_set_core_name -> Varchar,
        region -> Nullable<Varchar>,
    }
}

table! {
    matches_participants (id) {
        id -> Int4,
        match_id -> Varchar,
        summoner_id -> Varchar,
        gold_left -> Int4,
        level -> Int4,
        placement -> Int4,
        last_round -> Int4,
    }
}

table! {
    matches_participants_augments (id) {
        id -> Int4,
        match_participant_id -> Int4,
        augment_id -> Int4,
    }
}

table! {
    matches_participants_traits (id) {
        id -> Int4,
        match_participant_id -> Int4,
        trait_id -> Varchar,
        num_units -> Int4,
        tier_current -> Int4,
        tier_total -> Int4,
        style -> Int4,
    }
}

table! {
    matches_participants_unit_items (id) {
        id -> Int4,
        match_participant_unit_id -> Int4,
        item_id -> Int4,
    }
}

table! {
    matches_participants_units (id) {
        id -> Int4,
        match_participant_id -> Int4,
        unit_id -> Varchar,
        rarity -> Int4,
        tier -> Int4,
    }
}

table! {
    summoners (id) {
        id -> Varchar,
        account_id -> Varchar,
        puuid -> Varchar,
        name -> Varchar,
        summoner_level -> Int4,
        revision_date -> Int8,
        profile_icon_id -> Int4,
        region -> Nullable<Varchar>,
        league_id -> Varchar,
        revision_id -> Int8,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    traits (trait_id) {
        trait_id -> Varchar,
        display_name -> Varchar,
        set -> Varchar,
        icon_path -> Varchar,
        img -> Nullable<Varchar>,
    }
}

joinable!(matches_participants -> matches (match_id));
joinable!(matches_participants -> summoners (summoner_id));
joinable!(matches_participants_augments -> items (augment_id));
joinable!(matches_participants_augments -> matches_participants (match_participant_id));
joinable!(matches_participants_traits -> matches_participants (match_participant_id));
joinable!(matches_participants_traits -> traits (trait_id));
joinable!(matches_participants_unit_items -> items (item_id));
joinable!(matches_participants_unit_items -> matches_participants_units (match_participant_unit_id));
joinable!(matches_participants_units -> characters (unit_id));
joinable!(matches_participants_units -> matches_participants (match_participant_id));
joinable!(summoners -> leagues (league_id));

allow_tables_to_appear_in_same_query!(
    characters,
    items,
    leagues,
    matches,
    matches_participants,
    matches_participants_augments,
    matches_participants_traits,
    matches_participants_unit_items,
    matches_participants_units,
    summoners,
    traits,
);
