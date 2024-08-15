use std::fs::{copy, create_dir_all};
use std::io::Read;
use std::path::{Path, PathBuf};

use actix_files::NamedFile;
use base64::alphabet::STANDARD;
use base64::Engine;
use base64::engine::{GeneralPurpose, GeneralPurposeConfig};
use color_eyre::{Report, Result};
use color_eyre::eyre::ContextCompat;
use log::debug;
use tokio::fs::{File, remove_file};
use tokio::io::AsyncWriteExt;
use uuid::Uuid;

pub struct PathFile(PathBuf, String);
pub fn path_by_uuid(uuid: Uuid) -> Result<PathFile> {
    let uuid = uuid.to_string();

    let index_last_separator_position = uuid
        .to_string()
        .rfind('-')
        .with_context(|| "Error find separator in uuid")?;

    let path_directory = uuid
        .get(0..index_last_separator_position)
        .map(|x| x.replace('-', "/")) //TODO replace slash is dependecy by system
        .with_context(|| "Error making path directory by uuid")?;
    let file_name = uuid
        .get(index_last_separator_position + 1..) //+1 to Remove Separator
        .with_context(|| "Error making file name by uuid")?;

    Ok(PathFile(
        PathBuf::from(path_directory),
        file_name.to_owned(),
    ))
}

pub fn generate_path_by_uuid(base_path: PathBuf, extension: &str, uuid: Uuid) -> Result<String> {
    let PathFile(directory, file) = path_by_uuid(uuid)?;

    let path = base_path.join(directory).join(file + extension);
    Ok(path.to_string_lossy().to_string())
}

pub fn url_by_uuid(uuid: Uuid) -> Result<String> {
    let uuid = uuid.to_string();

    let index_last_separator_position = uuid
        .to_string()
        .rfind('-')
        .with_context(|| "Error find separator in uuid")?;

    let path_directory = uuid
        .get(0..index_last_separator_position)
        .map(|x| x.replace('-', "/"))
        .with_context(|| "Error making path directory by uuid")?;
    let file_name = uuid
        .get(index_last_separator_position + 1..) //+1 to Remove Separator
        .with_context(|| "Error making file name by uuid")?;

    Ok(path_directory + "/" + file_name)
}
pub fn generate_url_by_uuid(base_path: String, uuid: Uuid, extension: &str) -> Result<String> {
    //TODO pending url server?
    let file = url_by_uuid(uuid)?;
    let mut absolute_path = String::new();
    absolute_path.push_str(&base_path);
    absolute_path.push('/');
    absolute_path.push_str(&file);
    absolute_path.push_str(extension);
    Ok(absolute_path)
}

#[allow(unused_mut)]
pub async fn read_content_file_to_base64(path: &Path) -> Result<String> {
    let mut file_open = NamedFile::open_async(path).await?;

    let mut buffer_read_content_file = vec![];
    file_open.read_to_end(&mut buffer_read_content_file)?;

    let engine = GeneralPurpose::new(&STANDARD, GeneralPurposeConfig::default());
    Ok(engine.encode(buffer_read_content_file))
}
pub async fn read_content_bytes_to_base64(b: &[u8]) -> Result<String> {
    let engine = GeneralPurpose::new(&STANDARD, GeneralPurposeConfig::default());
    Ok(engine.encode(b))
}

fn create_parent_directories(to: &Path) -> Result<()> {
    if let Some(path_directory) = to.parent() {
        debug!(
            "Creating directories required to path: {:?}",
            path_directory
        );
        create_dir_all(path_directory)?;
    }
    Ok(())
}

pub fn move_file(from: &Path, to: PathBuf) -> Result<()> {
    debug!("Moving file from: {:?}, to: {:?}", from, to);

    if from.is_dir() {
        return Err(Report::msg("From path is a directory"));
    }

    create_parent_directories(&to)?;
    //Error using fs::rename, because error by temp files
    copy(from, to)?;
    Ok(())
}


pub async fn save_file(to: PathBuf, content: &[u8]) -> Result<()> {
    if to.is_dir() {
        return Err(Report::msg("Path is a directory"));
    }
    create_parent_directories(&to)?;

    debug!("Saving file in: {:?}", to);
    let mut f = File::create(to).await?;
    let _ = f.write(content).await?;

    Ok(())
}

pub async fn delete_file(p: &Path) -> Result<()> {
    remove_file(p).await?;
    Ok(())
}

pub fn get_extension_and_file_name(file_name: &str) -> (&str, Option<&str>) {
    if let Some(index) = file_name.rfind('.') {
        (&file_name[..index], Some(&file_name[index..]))
    } else {
        (file_name, None)
    }
}


pub fn get_file_name_in_url(url: &str) -> Option<&str> {
    if let Some(index) = url.rfind('/') {
        Some(&url[index + 1..])
    } else {
        None
    }
}
