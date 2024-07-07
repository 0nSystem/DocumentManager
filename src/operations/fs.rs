use std::fs::{copy, create_dir_all};
use std::io::Read;
use std::path::{Path, PathBuf};

use actix_files::NamedFile;
use color_eyre::eyre::ContextCompat;
use color_eyre::{Report, Result};
use log::debug;
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
pub async fn read_content_file(path: &Path) -> Result<Vec<u8>> {
    let mut file_open = NamedFile::open_async(path).await?;

    let mut buffer_read_content_file = vec![];
    file_open.read_to_end(&mut buffer_read_content_file)?;

    Ok(buffer_read_content_file)
}

pub fn move_file(from: &Path, to: PathBuf) -> Result<()> {
    debug!("Moving file from: {:?}, to: {:?}", from, to);

    if from.is_dir() {
        return Err(Report::msg("From path is a directory"));
    }

    if let Some(path_directory) = to.parent() {
        debug!(
            "Creating directories required to path: {:?}",
            path_directory
        );
        create_dir_all(path_directory)?
    }
    //Error using fs::rename, because error by temp files
    copy(from, to)?;
    Ok(())
}

pub fn get_extension_in_file_name(file_name: &str) -> &str {
    if let Some(index) = file_name.rfind('.') {
        &file_name[index..]
    } else {
        ""
    }
}
