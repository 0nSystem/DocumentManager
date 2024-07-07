use std::path::PathBuf;
use color_eyre::eyre::ContextCompat;
use color_eyre::Result;
use uuid::Uuid;

struct PathFile(PathBuf, String);
pub fn generate_path_by_uuid(uuid: Uuid) -> Result<PathFile> {
    let uuid = uuid.to_string();

    let index_last_separator_position = uuid.to_string().rfind("-").with_context(|| "Error find separator in uuid")?;

    let path_directory = uuid.get(0..index_last_separator_position)
        .map(|x| x.replace('-', "/")) //TODO replace slash is dependecy by system
        .with_context(|| "Error making path directory by uuid")?;
    let file_name = uuid.get(index_last_separator_position..).with_context(|| "Error making file name by uuid")?;

    Ok(PathFile(PathBuf::from(path_directory), file_name.to_owned()))
}

pub fn generate_url_by_uuid(base_path: &str, uuid: Uuid) -> Result<String> {
    let PathFile(directory, file) = generate_path_by_uuid(uuid)?;

    let path = PathBuf::from(base_path).join(directory).join(file).to_string_lossy().to_string();
    
    Ok(path)
}