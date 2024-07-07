use actix_multipart::form::{tempfile::TempFile, text::Text};
use actix_multipart::form::MultipartForm;
use actix_web::{delete, get, HttpRequest, HttpResponse, post, put, Responder, web};
use actix_web::http::StatusCode;
use log::info;
use crate::config::DbPool;
use crate::EnvironmentState;
use crate::operations::save_document;

#[get("/index")]
pub async fn index(_req: HttpRequest) -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[derive(Debug, MultipartForm)]
pub struct DocumentRequest {
    #[multipart(limit = "100MB")]
    pub file: TempFile,
    pub application: Text<String>,
    pub is_private_document: Text<bool>,
    pub username: Text<String>,
}

#[post("/")]
pub async fn upload_document(
    form: MultipartForm<DocumentRequest>, env_state: web::Data<EnvironmentState>, conn: web::Data<DbPool>,
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
    MultipartForm(form): MultipartForm<DocumentRequest>, conn: web::Data<DbPool>,
) -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[delete("/")]
pub async fn delete_document(
    conn: web::Data<DbPool>
) -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/")]
pub async fn find_documents(
    conn: web::Data<DbPool>
) -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}
