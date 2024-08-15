use std::path::{Path, PathBuf};

use actix_files as fs;
use actix_web::{App, HttpServer, web};
use color_eyre::{Report, Result};
use color_eyre::eyre::Context;
use env_logger::Target;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::config::{config_logger, configure_storage_directory, EnvConfig, establish_connection};
use crate::endpoints::{delete::delete_document, filter::find_documents, save::upload_document};
use crate::endpoints::{delete::DeleteDocumentRequest, filter::{DocumentContentResponse, DocumentFilterRequest, FoundContentResponse, FoundDocumentResponse}, save::SaveDocumentRequest};

mod config;
mod models;
mod operations;
mod schema;
mod endpoints;

#[derive(Clone)]
pub struct EnvironmentState {
    pub mount_path: String,
    pub disk_storage_directory_path: PathBuf,
}
impl TryFrom<EnvConfig> for EnvironmentState {
    type Error = Report;

    fn try_from(value: EnvConfig) -> std::result::Result<Self, Self::Error> {
        Ok(Self {
            mount_path: value.mount_path,
            disk_storage_directory_path: Path::new(&value.disk_storage_directory_path)
                .canonicalize()?,
        })
    }
}

#[derive(OpenApi)]
#[openapi(
    paths(
        endpoints::save::upload_document,
        endpoints::delete::delete_document,
        endpoints::filter::find_documents
    ),
    components(schemas(
        SaveDocumentRequest,
        DeleteDocumentRequest,
        DocumentFilterRequest,
        FoundDocumentResponse, FoundContentResponse, DocumentContentResponse
    ))
)]
struct ApiDoc;


#[actix_web::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    config_logger(Target::Stdout)?;
    HttpServer::new(|| {
        //TODO move
        let env_config = EnvConfig::new().unwrap();
        configure_storage_directory(&env_config.disk_storage_directory_path).unwrap();
        let env_state = EnvironmentState::try_from(env_config.clone()).unwrap();

        App::new()
            .app_data(web::Data::new(establish_connection(env_config.database_url)))
            .app_data(web::Data::new(env_state))
            .service(
                fs::Files::new(
                    &env_config.mount_path,
                    env_config.disk_storage_directory_path,
                ).show_files_listing(),
            )
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}")
                    .url("/api-docs/openapi.json", ApiDoc::openapi()),
            )
            .service(upload_document)
            .service(delete_document)
            .service(find_documents)
    })
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
        .with_context(|| "Error starting http server")
}
