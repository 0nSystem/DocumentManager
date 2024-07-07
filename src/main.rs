use actix_files as fs;
use actix_web::{App, HttpServer};
use color_eyre::eyre::Context;
use color_eyre::Result;
use env_logger::Target;

use crate::config::{config_logger, configure_storage_directory, EnvConfig, establish_connection};
use crate::endpoints::index;

mod endpoints;
mod models;
mod schema;
mod operations;
mod config;


#[actix_web::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    config_logger(Target::Stdout)?;

    HttpServer::new(
        || {
            let env_config = EnvConfig::new().unwrap();
            configure_storage_directory(&env_config.disk_storage_directory_path).unwrap();
            App::new()
                .app_data(establish_connection(env_config.database_url))
                .service(fs::Files::new(&env_config.mount_path, env_config.disk_storage_directory_path).show_files_listing())
                .service(index)
        }
    ).bind(("127.0.0.1", 8080))?
        .run()
        .await
        .with_context(|| "Error starting http server")
}