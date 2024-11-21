use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::blog::schema::posts)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Post {
    pub slug: String,
    pub author: i32,
    pub title: String,
    pub body: String,
    pub created_at: i64,
    pub updated_at: i64,
}
