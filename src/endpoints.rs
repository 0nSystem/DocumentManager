use actix_multipart::form::{tempfile::TempFile, text::Text};
use actix_multipart::form::MultipartForm;
use actix_web::{delete, get, HttpRequest, HttpResponse, post, put, Responder, web};
use actix_web::http::StatusCode;
use log::info;
use serde::Deserialize;

use crate::config::DbPool;
use crate::EnvironmentState;
use crate::operations::{filter_documents, save_document};

#[derive(Debug, MultipartForm)]
pub struct SaveDocumentRequest {
    #[multipart(limit = "100MB")]
    pub file: TempFile,
    pub application: Text<String>,
    pub is_private_document: Text<bool>,
    pub username: Text<String>,
}

#[post("/")]
pub async fn upload_document(
    form: MultipartForm<SaveDocumentRequest>, env_state: web::Data<EnvironmentState>, conn: web::Data<DbPool>,
) -> impl Responder {
    match save_document(form, env_state, conn).await {
        Ok(uuid) => HttpResponse::Ok().json(uuid),
        Err(error) => {
            info!("{error}");
            HttpResponse::new(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

#[put("/")]
pub async fn update_document(
    MultipartForm(form): MultipartForm<SaveDocumentRequest>, conn: web::Data<DbPool>,
) -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[delete("/")]
pub async fn delete_document(
    conn: web::Data<DbPool>
) -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
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
