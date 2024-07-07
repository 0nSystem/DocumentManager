use actix_multipart::form::MultipartForm;
use actix_multipart::form::{tempfile::TempFile, text::Text};
use actix_web::http::StatusCode;
use actix_web::{delete, get, post, web, HttpResponse, Responder};
use log::info;
use serde::Deserialize;
use uuid::Uuid;

use crate::config::DbPool;
use crate::operations::{delete_document_and_content, filter_documents, save_document};
use crate::EnvironmentState;

#[derive(Debug, MultipartForm)]
pub struct SaveDocumentRequest {
    #[multipart(limit = "5GB")]
    pub file: TempFile,
    pub application: Text<String>,
    pub is_private_document: Text<bool>,
    pub username: Text<String>,
}

#[post("/")]
pub async fn upload_document(
    form: MultipartForm<SaveDocumentRequest>,
    env_state: web::Data<EnvironmentState>,
    conn: web::Data<DbPool>,
) -> impl Responder {
    match save_document(form, env_state, conn).await {
        Ok(uuid) => HttpResponse::Ok().json(uuid),
        Err(error) => {
            info!("{error}");
            HttpResponse::new(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

//TODO reference
#[derive(Deserialize)]
pub struct DeleteDocumentRequest {
    pub id_document: Uuid,
    pub username: String,
}
#[delete("/")]
pub async fn delete_document(
    params: web::Query<DeleteDocumentRequest>,
    conn: web::Data<DbPool>,
) -> impl Responder {
    match delete_document_and_content(params, conn).await {
        Ok(_) => HttpResponse::new(StatusCode::OK),
        Err(_) => HttpResponse::new(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

#[derive(Deserialize)]
pub struct DocumentFilterRequest {
    pub username: String,
    pub application: String,
    #[serde(default)]
    pub extensions: Vec<String>,
    #[serde(default)]
    pub content_type: Vec<String>,
}

#[get("/")]
pub async fn find_documents(
    document_filter: web::Query<DocumentFilterRequest>,
    env_state: web::Data<EnvironmentState>,
    conn: web::Data<DbPool>,
) -> impl Responder {
    match filter_documents(document_filter.0, env_state, conn).await {
        Ok(r) => HttpResponse::Ok().json(r),
        Err(e) => {
            info!("{e}");
            HttpResponse::new(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}
