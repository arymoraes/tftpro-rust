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

joinable!(summoners -> leagues (league_id));

allow_tables_to_appear_in_same_query!(characters, items, leagues, summoners, traits,);
