use actix_multipart::form::{tempfile::TempFile, text::Text};
use actix_multipart::form::MultipartForm;
use actix_web::{delete, get, HttpResponse, post, Responder, web};
use actix_web::http::StatusCode;
use log::info;
use serde::Deserialize;
use utoipa::{IntoParams, ToSchema};
use uuid::Uuid;
use actix_web_lab::extract as exlab;
use crate::config::DbPool;
use crate::EnvironmentState;
use crate::operations::{delete_document_and_content, filter_documents, save_document};

#[derive(Debug, MultipartForm, ToSchema, IntoParams)]
pub struct SaveDocumentRequest {
    #[multipart(limit = "5GB")]
    #[schema(value_type = String, format = Binary)]
    pub file: TempFile,
    #[schema(value_type = String)]
    pub application: Text<String>,
    #[schema(value_type = bool)]
    pub is_private_document: Text<bool>,
    #[schema(value_type = String)]
    pub username: Text<String>,
}

#[utoipa::path(
    post,
    path = "/",
    request_body(content = SaveDocumentRequest, content_type = "multipart/form-data"),
    responses(
        (status = 200, description = "Successful upload document", body = Uuid)
    )
)]
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
            HttpResponse::InternalServerError().finish()
        }
    }
}


#[derive(Deserialize, ToSchema, IntoParams)]
pub struct DeleteDocumentRequest {
    pub id_document: Uuid,
    pub username: String,
}


#[utoipa::path(
    delete,
    path = "/",
    responses(
        (status = 200, description = "Successfully delete"),
        (status = 500, description = "Error")
    ),
    params(
        DeleteDocumentRequest
    )
)]
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

#[derive(Deserialize, ToSchema, IntoParams)]
pub struct DocumentFilterRequest {
    pub username: Option<String>,
    pub application: String,
    #[serde(default)]
    pub extensions: Vec<String>,
    #[serde(default)]
    pub content_type: Vec<String>,
}


#[utoipa::path(
    get,
    path = "/",
    params(
        DocumentFilterRequest
    ),
    responses(
        (status = 200, description = "Files")
    )
)]
#[get("/")]
pub async fn find_documents(
    document_filter: exlab::Query<DocumentFilterRequest>,
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
