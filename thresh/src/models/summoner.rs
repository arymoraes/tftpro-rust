#[derive(Queryable)]
pub struct Summoner {
    pub id: String,
    pub name: String,
    pub region: String,
    pub league_id: String,
    pub profile_icon_id: i32,
    pub summoner_level: i32,
    pub revision_date: i64,
    pub account_id: String,
    pub puuid: String,
}
