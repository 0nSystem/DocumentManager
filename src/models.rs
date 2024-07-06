use chrono::{DateTime, NaiveDateTime};
use diesel::{Insertable, Queryable, Selectable};
use uuid::Uuid;
use crate::schema::document;

#[derive(Insertable)]
#[diesel(table_name = document)]
pub struct NewDocument<'a> {
    pub name: &'a str,
    pub extension: &'a str,
    pub application: &'a str,
}


#[derive(Queryable, Selectable)]
#[diesel(table_name = document)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Document {
    pub id_document: Uuid,

    pub name: String,
    pub extension: String,
    pub application: String,

    pub create_datetime: NaiveDateTime,
    pub create_username: String,

    pub update_datetime: Option<NaiveDateTime>,
    pub update_username: Option<String>,

    pub delete_datetime: Option<NaiveDateTime>,
    pub delete_username: Option<String>,
}