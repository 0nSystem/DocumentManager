use chrono::NaiveDateTime;
use diesel::{Identifiable, Insertable, Queryable, Selectable};
use uuid::Uuid;

use crate::schema::document;


#[derive(Queryable, Identifiable, Selectable, Debug, PartialEq)]
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
#[derive(Insertable)]
#[diesel(table_name = document)]
pub struct NewDocument<'a> {
    pub uuid: &'a Uuid,
    pub name: &'a str,
    pub extension: Option<&'a str>,
    pub application: &'a str,
    pub create_username: &'a str,
}

#[derive(Queryable, Identifiable, Selectable, Debug, PartialEq)]
#[diesel(table_name = content)]
#[diesel(belongs_to(Document, foreign_key = "id_document"))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Content {
    pub id: Uuid,
    pub id_document: Uuid,
    pub data: String,

    pub create_datetime: NaiveDateTime,
    pub create_username: String,

    pub delete_datetime: Option<NaiveDateTime>,
    pub delete_username: Option<String>,
}
#[derive(Insertable)]
#[diesel(table_name = document)]
pub struct NewContent<'a> {
    pub id_document: &'a Uuid,
    pub data: &'a str,
    pub create_username: String,
}
