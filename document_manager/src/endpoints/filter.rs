use actix_web::{get, HttpResponse, Responder, web};
use actix_web::http::StatusCode;
use actix_web_lab::extract as exlab;
use log::info;
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use uuid::Uuid;

use crate::config::DbPool;
use crate::EnvironmentState;
use crate::operations::{DocumentContent, filter_documents, FoundContent, FoundDocument};

#[derive(Deserialize, ToSchema, IntoParams)]
pub struct DocumentFilterRequest {
    pub username: Option<String>,
    pub application: String,
    #[serde(default)]
    pub extensions: Vec<String>,
    #[serde(default)]
    pub content_type: Vec<String>,
}


#[derive(Serialize, Deserialize, ToSchema)]
pub struct FoundDocumentResponse {
    pub id_document: Uuid,
    pub name: String,
    pub extension: String,
    pub application: String,
    pub content: FoundContentResponse,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct FoundContentResponse {
    pub content: DocumentContentResponse,
}
impl From<FoundContent> for FoundContentResponse {
    fn from(value: FoundContent) -> Self {
        Self {
            content: DocumentContentResponse::from(value.content),
        }
    }
}

#[derive(Serialize, Deserialize, ToSchema)]
pub enum DocumentContentResponse {
    Data(String),
    Url(String),
    None, //Never option
}
impl From<DocumentContent> for DocumentContentResponse {
    fn from(value: DocumentContent) -> Self {
        match value {
            DocumentContent::Data(v) => DocumentContentResponse::Data(v),
            DocumentContent::Url(v) => DocumentContentResponse::Url(v),
            DocumentContent::None => DocumentContentResponse::None
        }
    }
}

impl From<FoundDocument> for FoundDocumentResponse {
    fn from(value: FoundDocument) -> Self {
        Self {
            id_document: value.id_document,
            name: value.name,
            extension: value.extension,
            application: value.application,
            content: FoundContentResponse::from(value.content),
        }
    }
}


#[utoipa::path(
    get,
    path = "/",
    params(
        DocumentFilterRequest
    ),
    responses(
        (status = 200, description = "Files", body = Vec < FoundDocumentResponse >)
    )
)]
#[get("/")]
pub async fn find_documents(
    document_filter: exlab::Query<DocumentFilterRequest>,
    env_state: web::Data<EnvironmentState>,
    conn: web::Data<DbPool>,
) -> impl Responder {
    match filter_documents(document_filter.0, env_state, conn).await {
        Ok(r) => {
            let mut result = vec![];
            for i in r {
                result.push(FoundDocumentResponse::from(i))
            }
            HttpResponse::Ok().json(result)
        }
        Err(e) => {
            info!("{e}");
            HttpResponse::new(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}
