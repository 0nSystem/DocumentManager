use std::io::Read;
use std::path::PathBuf;

use actix_multipart::form::MultipartForm;
use actix_web::web;
use color_eyre::{Report, Result};
use color_eyre::eyre::Context;
use diesel::insert_into;
use diesel_async::{AsyncConnection, RunQueryDsl};
use diesel_async::scoped_futures::ScopedFutureExt;
use log::debug;
use uuid::Uuid;

use crate::config::DbPool;
use crate::models::{NewContent, NewDocument};
use crate::operations::fs::{generate_path_by_uuid_and_extension, move_file, path_by_uuid, read_content_file};
use crate::{EnvironmentState, schema};

mod fs;

pub async fn save_document(
    MultipartForm(form): MultipartForm<crate::endpoints::DocumentRequest>,
    env_state: web::Data<EnvironmentState>, conn: web::Data<DbPool>,
) -> Result<Uuid> {
    let conn = &mut conn.get().await?;

    conn.transaction::<Uuid, Report, _>(|conn| async move {
        let uuid_document = Uuid::new_v4();
        debug!("Generate UUID: {uuid_document}, to save document");
        let new_document = NewDocument {
            id_document: &uuid_document,
            name: &form.file.file_name.unwrap_or(uuid_document.to_string()),
            extension: form.file.content_type.map(|m| m.to_string()),
            application: &form.application,
            create_username: &form.username,
        };
        insert_into(schema::document::table)
            .values(new_document)
            .execute(conn).await
            .with_context(|| "Error create ")?;

        let temp_file_path = form.file.file.path();
        debug!("Document tempfile path: {:?}",temp_file_path);
        if *form.is_private_document {
            debug!("Document is private saving in database");
            let content_file = read_content_file(temp_file_path).await?;
            let content = NewContent {
                id_document: &uuid_document,
                data: &content_file,
                create_username: &form.username,
            };
            insert_into(schema::content::table)
                .values(content)
                .execute(conn).await
                .with_context(|| "Error save row in table content")?;
        } else {
            let new_path = PathBuf::from(generate_path_by_uuid_and_extension(env_state.disk_storage_directory_path.clone(), temp_file_path.extension(), uuid_document)?);
            debug!("Document is public saving in {:?}",new_path);
            move_file(temp_file_path, new_path)?;
        }
        debug!("Finish procces save document");
        Ok(uuid_document)
    }.scope_boxed()).await
}


pub async fn update_document(
    conn: DbPool
) -> Result<()> {
    todo!()
}

pub async fn delete_document(conn: DbPool) -> Result<()> {
    todo!()
}

pub async fn find_documents(conn: DbPool) -> Result<()> {
    todo!()
}




