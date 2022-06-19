#[derive(Queryable)]
pub struct League {
    pub league_id: String,
    pub tier: String,
    pub name: String,
    pub queue: String,
}
