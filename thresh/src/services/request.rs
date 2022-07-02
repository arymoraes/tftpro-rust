#[derive(Debug, Clone)]
pub struct RequestOptions {
    pub region: Option<String>,
    pub sub_region: Option<String>,
    pub league_id: Option<String>,
    pub summoner_puuid: Option<String>,
    pub match_id: Option<String>,
}
