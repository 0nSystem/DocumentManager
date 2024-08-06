use actix_web::{delete, HttpResponse, Responder, web};
use actix_web::http::StatusCode;
use serde::Deserialize;
use utoipa::{IntoParams, ToSchema};
use uuid::Uuid;
use crate::config::DbPool;
use crate::operations::delete_document_and_content;

#[derive(Deserialize, ToSchema, IntoParams)]
pub struct DeleteDocumentRequest {
    /// Document Identify
    #[schema(value_type = Uuid)]
    pub id_document: Uuid,

    /// Audit Fields
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