use chrono::NaiveDateTime;
use crate::schema::article;
use diesel;
use diesel::prelude::*;
use diesel::mysql::MysqlConnection;

#[table_name = "article"]
#[derive(AsChangeset, Serialize, Deserialize, Queryable, Insertable)]
pub struct Article {
    pub id: Option<i32>,
    pub title: String,
    pub body: String,
    pub author: i32,
    pub create_at: NaiveDateTime,
    pub update_at: NaiveDateTime,
}


impl Article {
    pub fn create(article: Article, connection: &MysqlConnection) -> Article {
        diesel::insert_into(article::table)
            .values(&article)
            .execute(connection)
            .expect("Error creating new Article");

        article::table
            .order(article::id.desc()).first(connection)
            .unwrap()
    }

    pub fn read(connection: &MysqlConnection) -> Vec<Article> {
        article::table
            .order(article::id)
            .load::<Article>(connection)
            .unwrap()
    }

    pub fn update(id: i32, article: Article, connection: &MysqlConnection) -> bool {
        diesel::
        update(article::table.find(id))
            .set(&article)
            .execute(connection)
            .is_ok()
    }

    pub fn delete(id: i32, connection: &MysqlConnection) -> bool {
        diesel::
        delete(article::table.find(id))
            .execute(connection)
            .is_ok()
    }
}
