use actix_multipart::form::{tempfile::TempFile, text::Text};
use actix_multipart::form::MultipartForm;
use actix_web::{HttpResponse, post, Responder, web};
use actix_web::web::Json;
use log::info;
use serde::Deserialize;
use utoipa::{IntoParams, ToSchema};


use crate::config::DbPool;
use crate::EnvironmentState;
use crate::operations::{save_document, save_document_by_url};

#[derive(Debug, MultipartForm, ToSchema, IntoParams)]
pub struct SaveDocumentRequest {
    /// File to save
    #[multipart(limit = "5GB")]
    #[schema(value_type = String, format = Binary)]
    pub file: TempFile,

    /// Application Name, is audit field
    #[schema(value_type = String)]
    pub application: Text<String>,

    /// If is private document save file in database, else save in system directory
    #[schema(value_type = bool)]
    pub is_private_document: Text<bool>,

    /// Username, is audit field
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


#[derive(Debug, Deserialize, ToSchema, IntoParams)]
pub struct SaveDocumentByUrlRequest {
    pub url_file: String,
    pub application: String,
    pub is_private_document: bool,
    pub username: String,
}

#[utoipa::path(
    post,
    path = "/upload_document_by_url",
    request_body(content = SaveDocumentByUrlRequest),
    responses(
        (status = 200, description = "Successful", body = Uuid)
    ),
)]
#[post("/upload_document_by_url")]
pub async fn upload_document_by_url(body: Json<SaveDocumentByUrlRequest>,
                                    env_state: web::Data<EnvironmentState>,
                                    conn: web::Data<DbPool>,
) -> impl Responder {
    match save_document_by_url(body.into_inner(), env_state, &conn).await {
        Ok(uuid) => HttpResponse::Ok().json(uuid),
        Err(error) => {
            info!("{error}");
            HttpResponse::InternalServerError().finish()
        }
    }
}