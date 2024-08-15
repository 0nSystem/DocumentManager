use std::path::{Path, PathBuf};

use actix_multipart::form::MultipartForm;
use actix_web::web;
use bytes::Bytes;
use chrono::Local;
use color_eyre::{Report, Result};
use color_eyre::eyre::{Context, OptionExt};
use diesel::{ExpressionMethods, insert_into, update};
use diesel::prelude::*;
use diesel_async::{AsyncConnection, RunQueryDsl};
use diesel_async::scoped_futures::ScopedFutureExt;
use itertools::Itertools;
use log::{debug, error, info};
use mime_guess::from_ext;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::config::DbPool;
use crate::endpoints::delete::DeleteDocumentRequest;
use crate::endpoints::filter::DocumentFilterRequest;
use crate::endpoints::save::{SaveDocumentByUrlRequest, SaveDocumentRequest};
use crate::EnvironmentState;
use crate::models::{Content, DeleteContent, DeleteDocument, Document, NewContent, NewDocument};
use crate::operations::fs::{delete_file, generate_path_by_uuid, generate_url_by_uuid, get_extension_and_file_name, get_file_name_in_url, move_file, read_content_bytes_to_base64, read_content_file_to_base64, save_file};
use crate::schema::{content, document};

mod fs;

pub async fn save_document(
    MultipartForm(form): MultipartForm<SaveDocumentRequest>,
    env_state: web::Data<EnvironmentState>,
    conn: web::Data<DbPool>,
) -> Result<Uuid> {
    let conn = &mut conn.get().await?;

    conn.transaction::<Uuid, Report, _>(|conn| {
        async move {
            let uuid_document = Uuid::new_v4();
            debug!("Generate UUID: {uuid_document}, to save document");

            let default_filename_to_scan = &form.file.file_name.unwrap_or("".to_string());
            let (filename, extension) = get_extension_and_file_name(default_filename_to_scan);

            let new_document = NewDocument {
                id_document: &uuid_document,
                name: filename,
                extension: extension.map(|x| x.to_string()),
                content_type: form.file.content_type.map(|m| m.to_string()),
                application: &form.application,
                create_username: &form.username,
            };
            insert_into(document::table)
                .values(new_document)
                .execute(conn)
                .await
                .with_context(|| "Error create document")?;

            let temp_file_path = form.file.file.path();
            debug!("Document tempfile path: {:?}", temp_file_path);
            if *form.is_private_document {
                debug!("Document is private saving in database");
                let content_file = read_content_file_to_base64(temp_file_path).await?;
                let content = NewContent {
                    id_document: &uuid_document,
                    data: &content_file,
                    create_username: &form.username,
                };
                insert_into(content::table)
                    .values(content)
                    .execute(conn)
                    .await
                    .with_context(|| "Error save row in table content")?;
            } else {
                let new_path = PathBuf::from(generate_path_by_uuid(
                    env_state.disk_storage_directory_path.clone(),
                    extension.unwrap_or(""),
                    uuid_document,
                )?);
                debug!("Document is public saving in {:?}", new_path);
                move_file(temp_file_path, new_path)?;
            }
            debug!("Finish procces save document");
            Ok(uuid_document)
        }
            .scope_boxed()
    })
        .await
}

pub async fn save_document_by_url(params: SaveDocumentByUrlRequest,
                                  env_state: web::Data<EnvironmentState>,
                                  conn: &DbPool) -> Result<Uuid> {
    let conn = &mut conn.get().await?;
    let uuid_document = Uuid::new_v4();
    debug!("Generate UUID: {uuid_document}, to save document");


    let r = conn.transaction::<Uuid, Report, _>(|conn| {
        async move {
            let url_file = params.url_file.clone();
            let file_name_in_url = get_file_name_in_url(&url_file)
                .ok_or_eyre(format!("Error not match filename in url: {:?} ", url_file))?;
            let (filename, extension) = get_extension_and_file_name(file_name_in_url);

            let mime_type = extension
                .map(|x| from_ext(x).first_or_octet_stream().to_string());
            let info_download = download_file(&params.url_file).await?;
            let new_document = NewDocument {
                id_document: &uuid_document,
                name: filename,
                extension: extension.map(|x| x.to_string()),
                content_type: mime_type,
                application: &params.application,
                create_username: &params.username,
            };
            insert_into(document::table)
                .values(new_document)
                .execute(conn)
                .await
                .with_context(|| "Error create document")?;

            if params.is_private_document {
                debug!("Document is private saving in database");
                let content_file = read_content_bytes_to_base64(&info_download.content).await?;
                let content = NewContent {
                    id_document: &uuid_document,
                    data: &content_file,
                    create_username: &params.username,
                };
                insert_into(content::table)
                    .values(content)
                    .execute(conn)
                    .await
                    .with_context(|| "Error save row in table content")?;
            } else {
                let new_path = PathBuf::from(generate_path_by_uuid(
                    env_state.disk_storage_directory_path.clone(),
                    extension.unwrap_or(""),
                    uuid_document,
                )?);
                debug!("Document is public saving in {:?}", new_path);
                save_file(new_path, &info_download.content).await?;
            }
            debug!("Finish procces save document");
            Ok(uuid_document)
        }
            .scope_boxed()
    })
        .await;


    match r {
        Ok(e) => Ok(e),
        Err(e) => {
            //delete_file(&clone_new_path).await?;

            Err(e)
        }
    }
}


struct DownloadFileInfo {
    pub content: Bytes,
}
async fn download_file(url: &str) -> Result<DownloadFileInfo> {
    let client = Client::new();
    let response = client.get(url).send().await?;

    if response.status().is_success() {
        let content = response.bytes().await?;
        info!("File downloaded");
        Ok(
            DownloadFileInfo {
                content,
            }
        )
    } else {
        error!("Error download file: {}", response.status());
        Err(Report::msg("Error download file"))
    }
}


pub async fn delete_document_and_content(
    document_delete: web::Query<DeleteDocumentRequest>,
    conn: web::Data<DbPool>,
) -> Result<()> {
    let conn = &mut conn.get().await?;
    conn.transaction::<(), Report, _>(|conn| {
        async move {
            let current_datetime = Local::now().naive_local();

            let delete_document = DeleteDocument {
                delete_datetime: current_datetime,
                delete_username: &document_delete.username,
            };
            update(document::table)
                .filter(document::dsl::id_document.eq(document_delete.id_document))
                .set(delete_document)
                .execute(conn)
                .await?; //TODO remove await and control end

            let delete_content = DeleteContent {
                delete_datetime: current_datetime,
                delete_username: &document_delete.username,
            };
            update(content::table)
                .filter(content::dsl::id_document.eq(document_delete.id_document))
                .set(delete_content)
                .execute(conn)
                .await?; //TODO remove await and control end

            Ok(())
        }
            .scope_boxed()
    })
        .await
}

#[derive(Serialize, Deserialize)]
pub struct FoundDocument {
    pub id_document: Uuid,
    pub name: String,
    pub extension: String,
    pub application: String,
    pub content: FoundContent,
}

#[derive(Serialize, Deserialize)]
pub struct FoundContent {
    pub content: DocumentContent,
}

#[derive(Serialize, Deserialize)]
pub enum DocumentContent {
    Data(String),
    Url(String),
    None, //Never option
}

pub async fn filter_documents(
    filter: DocumentFilterRequest,
    env_state: web::Data<EnvironmentState>,
    conn: web::Data<DbPool>,
) -> Result<Vec<FoundDocument>> {
    let conn = &mut conn.get().await?;

    let mut query = document::dsl::document
        .into_boxed()
        .filter(document::dsl::delete_datetime.is_null())
        .filter(document::dsl::application.eq(filter.application));

    if let Some(username) = filter.username {
        query = query.filter(document::dsl::create_username.eq(username));
    }

    if !filter.extensions.is_empty() {
        query = query.filter(document::dsl::extension.eq_any(filter.extensions));
    }

    if !filter.content_type.is_empty() {
        query = query.filter(document::dsl::extension.eq_any(filter.content_type));
    }

    let documents = query.select(Document::as_select()).load(conn).await?;

    let content_found = Content::belonging_to(&documents)
        .filter(content::dsl::delete_datetime.is_null())
        .select(Content::as_select())
        .load(conn)
        .await?;
    let group_content_by_document = content_found.iter().into_group_map_by(|f| f.id_document);

    let mut documents_founds = Vec::new();
    for doc in documents {
        let resolved_extension = doc.extension.unwrap_or("".to_string());

        let content = group_content_by_document.get(&doc.id_document);
        let document_content = if let Some(c) = content {
            if !c.is_empty() {
                //never empty
                let content_file = c.as_slice().first().unwrap();
                DocumentContent::Data(content_file.data.clone())
            } else {
                DocumentContent::None
            }
        } else {
            let content_url =
                generate_url_by_uuid(env_state.mount_path.clone(), doc.id_document, &resolved_extension)?;
            DocumentContent::Url(content_url)
        };

        let found_document = FoundDocument {
            id_document: doc.id_document,
            name: doc.name,
            extension: resolved_extension,
            application: doc.application,
            content: FoundContent {
                content: document_content,
            },
        };

        documents_founds.push(found_document);
    }

    Ok(documents_founds)
}
