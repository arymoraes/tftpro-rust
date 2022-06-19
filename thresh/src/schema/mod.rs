table! {
  leagues (league_id) {
      league_id -> Varchar,
      tier -> Nullable<Varchar>,
      queue -> Nullable<Varchar>,
      name -> Nullable<Varchar>,
  }
}

table! {
  summoners (id) {
      id -> Varchar,
      account_id -> Nullable<Varchar>,
      puuid -> Nullable<Varchar>,
      name -> Nullable<Varchar>,
      summoner_level -> Nullable<Int4>,
      revision_date -> Nullable<Int8>,
      profile_icon_id -> Nullable<Int4>,
      region -> Nullable<Varchar>,
      league_id -> Nullable<Varchar>,
  }
}

joinable!(summoners -> leagues (league_id));

allow_tables_to_appear_in_same_query!(leagues, summoners,);
