use std::env;
use std::path::PathBuf;
use color_eyre::{Report, Result};
use color_eyre::eyre::Context;
use diesel_async::AsyncPgConnection;
use diesel_async::pooled_connection::deadpool::Pool;
use diesel_async::pooled_connection::{AsyncDieselConnectionManager, ManagerConfig};
use env_logger::{Builder, Target};
use log::LevelFilter;

pub fn configure_storage_directory(disk_storage_directory_path: &str) -> Result<()> {
    let disk = PathBuf::from(disk_storage_directory_path);
    if disk.exists() && disk.is_file() {
        return Err(Report::msg("Error oath for files services isn´t directory"));
    }

    if !disk.exists() {
        std::fs::create_dir_all(disk_storage_directory_path)?;
    }

    return Ok(());
}

pub fn config_logger(target: Target) -> Result<()> {
    Builder::from_default_env()
        .target(target)
        .default_format()
        .format_indent(Some(4))
        .format_module_path(false)
        .format_timestamp_millis()
        .write_style(env_logger::WriteStyle::Always)
        .filter(None, LevelFilter::Debug)
        .try_init()
        .with_context(|| "Wasn't unable to set up the logger")
}

pub type DbPool = Pool<AsyncPgConnection>;
pub async fn establish_connection(database_url: String) -> DbPool {
    let config = ManagerConfig::default();
    let manager = AsyncDieselConnectionManager::<AsyncPgConnection>::new_with_config(database_url, config);

    Pool::builder(manager)
        .build().unwrap()
}

pub struct EnvConfig {
    pub database_url: String,
    pub mount_path: String,
    pub disk_storage_directory_path: String,
}
impl EnvConfig {
    pub fn new() -> color_eyre::Result<Self> {
        Ok(
            Self {
                database_url: env::var("DATABASE_URL")?,
                mount_path: env::var("MOUNT_PATH")?,
                disk_storage_directory_path: env::var("DISK_STORAGE_DIRECTORY")?,
            }
        )
    }
}