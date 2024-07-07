use std::env;

use actix_files as fs;
use actix_web::{App, HttpServer};
use color_eyre::eyre::Context;
use color_eyre::Result;
use diesel_async::AsyncPgConnection;
use diesel_async::pooled_connection::{AsyncDieselConnectionManager, deadpool::Pool, ManagerConfig};
use diesel_async::pooled_connection::deadpool::Object;
use dotenvy::dotenv;
use env_logger::{Builder, Target};
use log::LevelFilter;

use crate::endpoints::index;

mod endpoints;
mod models;
mod schema;
mod operations;

fn config_logger(target: Target) -> Result<()> {
    Builder::from_default_env()
        .target(target)
        .default_format()
        .format_indent(Some(4))
        .format_module_path(false)
        .format_timestamp_millis()
        .write_style(env_logger::WriteStyle::Always)
        .filter(None, LevelFilter::Info)
        .try_init()
        .with_context(|| "Zork++ wasn't unable to set up the logger")
}

pub type DbConnection = Object<AsyncDieselConnectionManager<AsyncPgConnection>>;
pub type DbPool = Pool<AsyncPgConnection>;
async fn establish_connection() -> DbPool {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let config = ManagerConfig::default();
    let manager = AsyncDieselConnectionManager::<AsyncPgConnection>::new_with_config(database_url, config);

    Pool::builder(manager)
        .build().unwrap()
}


#[actix_web::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    config_logger(Target::Stdout)?;

    HttpServer::new(
        || App::new()
            .app_data(establish_connection())
            .service(fs::Files::new("/static", ".").show_files_listing())
            .service(index)
    ).bind(("127.0.0.1", 8080))?
        .run()
        .await
        .with_context(|| "Error starting http server")
}