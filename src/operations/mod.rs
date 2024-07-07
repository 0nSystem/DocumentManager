mod fs;

use actix_multipart::form::MultipartForm;
use actix_web::web;
use color_eyre::{Report, Result};
use color_eyre::eyre::Context;
use diesel::insert_into;
use diesel_async::{AsyncConnection, RunQueryDsl};
use diesel_async::scoped_futures::ScopedFutureExt;
use uuid::Uuid;

use crate::DbPool;
use crate::models::{NewContent, NewDocument};
use crate::schema;

pub async fn save_document(
    MultipartForm(form): MultipartForm<crate::endpoints::DocumentRequest>, conn: web::Data<DbPool>,
) -> Result<Uuid> {
    let conn = &mut conn.get().await?;

    conn.transaction::<Uuid, Report, _>(|conn| async move {
        let uuid_document = Uuid::new_v4();
        let new_document = NewDocument {
            id_document: &uuid_document,
            name: &form.file.file_name.unwrap_or(uuid_document.to_string()),
            extension: form.file.content_type.map(|m| m.to_string()),
            application: &form.json.application,
            create_username: &form.json.username,
        };
        insert_into(schema::document::table)
            .values(new_document)
            .execute(conn).await
            .with_context(|| "Error create ")?;

        if form.json.is_private_document {
            let content = NewContent {
                id_document: &uuid_document,
                data: "", //TODO
                create_username: &form.json.username,
            };
            insert_into(schema::content::table)
                .values(content)
                .execute(conn).await
                .with_context(|| "Error create ")?;
        } else {
            //TODO generate path to save content
            todo!()
        }
        Ok(uuid_document)
    }.scope_boxed()).await
        .with_context(|| "Error create document")
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




