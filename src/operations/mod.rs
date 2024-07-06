use actix_multipart::form::MultipartForm;
use color_eyre::Result;
use uuid::Uuid;

use crate::DbPool;
use crate::models::NewDocument;

async fn save_document(
    MultipartForm(form): MultipartForm<crate::endpoints::DocumentRequest>, conn: DbPool,
) -> Result<Uuid> {
    let uuid = Uuid::new_v4();

    let new_document = NewDocument {
        name: &form.file.file_name.unwrap_or(uuid.to_string()),
        extension: form.file.content_type.map(|m| m.to_string().as_str()),
        application: &form.json.application,
    };




    todo!()
}


async fn update_document(
    conn: DbPool
) -> Result<()> {
    todo!()
}

async fn delete_document(conn: DbPool) -> Result<()> {
    todo!()
}

async fn find_documents(conn: DbPool) -> Result<()> {
    todo!()
}