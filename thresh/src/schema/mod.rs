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

joinable!(summoners -> leagues (league_id));

allow_tables_to_appear_in_same_query!(leagues, summoners,);
