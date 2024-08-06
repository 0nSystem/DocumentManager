use actix_multipart::form::{tempfile::TempFile, text::Text};
use actix_multipart::form::MultipartForm;
use actix_web::{HttpResponse, post, Responder, web};
use log::info;
use utoipa::{IntoParams, ToSchema};


use crate::config::DbPool;
use crate::EnvironmentState;
use crate::operations::save_document;

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