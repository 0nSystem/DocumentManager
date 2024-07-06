use actix_multipart::form::{json::Json as MPJson, tempfile::TempFile};
use actix_multipart::form::MultipartForm;
use actix_web::{delete, get, HttpRequest, HttpResponse, post, put, Responder, web};
use serde::Deserialize;

use crate::DbPool;

#[get("/index")]
pub async fn index(_req: HttpRequest) -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[derive(Debug, Deserialize)]
pub struct MetadataDocument {
    pub application: String,
    pub is_private_document: bool,
}

#[derive(Debug, MultipartForm)]
pub struct DocumentRequest {
    #[multipart(limit = "100MB")]
    pub file: TempFile,
    pub json: MPJson<MetadataDocument>,
}

#[post("")]
pub async fn upload_document(
    MultipartForm(form): MultipartForm<DocumentRequest>, conn: web::Data<DbPool>,
) -> impl Responder {}

#[put("")]
pub async fn update_document(
    MultipartForm(form): MultipartForm<DocumentRequest>, conn: web::Data<DbPool>,
) -> impl Responder {}

#[delete("")]
pub async fn delete_document(
    conn: web::Data<DbPool>
) -> impl Responder {}

#[get("")]
pub async fn find_documents(
    conn: web::Data<DbPool>
) -> impl Responder {}
