#[table_name = "author"]
#[derive(AsChangeset, Serialize, Deserialize, Queryable, Insertable)]
pub struct Author {
    pub id: Option<i64>,
    pub name: String,
    pub username: String,
    pub resume: String,
    pub company: String,
    pub years_experience: i32,
    pub country: String,
}
