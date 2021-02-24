use chrono::{DateTime, Utc};
use diesel;

use crate::author::Author;

#[table_name = "article"]
#[derive(AsChangeset, Serialize, Deserialize, Queryable, Insertable)]
pub struct Article {
    pub id: Option<i64>,
    title: string,
    body: string,
    author: Author,
    create_at: i64,
    updated_at: i64,
}