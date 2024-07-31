use chrono::NaiveDateTime;
use diesel::{AsChangeset, Associations, Identifiable, Insertable, Queryable, Selectable};
use serde::Serialize;
use uuid::Uuid;

use crate::schema;

#[derive(Queryable, Selectable, Identifiable, Debug, PartialEq)]
#[diesel(primary_key(id_document))]
#[diesel(table_name = schema::document)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Document {
    pub id_document: Uuid,

    pub name: String,
    pub application: String,
    pub extension: Option<String>,
    pub content_type: Option<String>,

    pub create_datetime: NaiveDateTime,
    pub create_username: String,

    pub update_datetime: Option<NaiveDateTime>,
    pub update_username: Option<String>,

    pub delete_datetime: Option<NaiveDateTime>,
    pub delete_username: Option<String>,
}

#[derive(Insertable)]
#[diesel(table_name = schema::document)]
pub struct NewDocument<'a> {
    pub id_document: &'a Uuid,
    pub name: &'a str,
    pub extension: Option<String>,
    pub content_type: Option<String>,
    pub application: &'a str,
    pub create_username: &'a str,
}

//TODO reference
#[derive(Serialize, AsChangeset)]
#[diesel(table_name = schema::document)]
pub struct DeleteDocument<'a> {
    pub delete_datetime: NaiveDateTime,
    pub delete_username: &'a str,
}

#[derive(Queryable, Selectable, Identifiable, Associations, Debug, PartialEq)]
#[diesel(table_name = schema::content)]
#[diesel(belongs_to(Document, foreign_key = id_document))]
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
#[diesel(table_name = schema::content)]
pub struct NewContent<'a> {
    pub id_document: &'a Uuid,
    pub data: &'a str,
    pub create_username: &'a str,
}

#[derive(Serialize, AsChangeset)]
#[diesel(table_name = schema::content)]
pub struct DeleteContent<'a> {
    pub delete_datetime: NaiveDateTime,
    pub delete_username: &'a str,
}
