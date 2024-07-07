use std::path::{Path, PathBuf};

use actix_files as fs;
use actix_web::{App, HttpServer, web};
use color_eyre::{Report, Result};
use color_eyre::eyre::Context;
use env_logger::Target;

use crate::config::{config_logger, EnvConfig, establish_connection};
use crate::endpoints::{find_documents, upload_document};

mod endpoints;
mod models;
mod schema;
mod operations;
mod config;

#[derive(Clone)]
pub struct EnvironmentState {
    pub mount_path: String,
    pub disk_storage_directory_path: PathBuf,
}
impl TryFrom<EnvConfig> for EnvironmentState {
    type Error = Report;

    fn try_from(value: EnvConfig) -> std::result::Result<Self, Self::Error> {
        Ok(
            Self {
                mount_path: value.mount_path,
                disk_storage_directory_path: Path::new(&value.disk_storage_directory_path).canonicalize()?,
            }
        )
    }
}


#[actix_web::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    config_logger(Target::Stdout)?;

    HttpServer::new(
        || {
            let env_config = EnvConfig::new().unwrap();
            let env_state = EnvironmentState::try_from(env_config.clone()).unwrap();
            App::new()
                .app_data(web::Data::new(establish_connection(env_config.database_url)))
                .app_data(web::Data::new(env_state))
                .service(fs::Files::new(&env_config.mount_path, env_config.disk_storage_directory_path).show_files_listing())
                .service(upload_document)
                .service(find_documents)
        }
    ).bind(("127.0.0.1", 8080))?
        .run()
        .await
        .with_context(|| "Error starting http server")
}
