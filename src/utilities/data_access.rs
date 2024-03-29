use std::{fs, fs::File, io::Write, path::Path};

use ron::{
    de::from_reader,
    ser::{to_string_pretty, PrettyConfig},
};
use serde::{de, ser};

pub fn read_and_deserialize<T>(file_path: &str) -> Option<T>
where
    T: de::DeserializeOwned,
{
    if let Ok(file) = File::open(data_path(file_path)) {
        match from_reader(file) {
            Ok(x) => return Some(x),
            Err(e) => println!("Failed to deserialize {} : {}", file_path, e),
        }
    };
    None
}

pub fn save_to_file<T>(data: &T, file_path: &str) -> Result<(), DataAccessError>
where
    T: ser::Serialize,
{
    let data_path = data_path(file_path);
    let path = Path::new(&data_path);
    match create_parents_directories_if_not_exist(&path) {
        Err(e) => Err(e),
        _ => {
            let file_already_exist = path.exists();
            if !file_already_exist || fs::remove_file(path).is_ok() {
                if let Ok(mut target_file) = File::create(path) {
                    let string_data = to_string_pretty(data, PrettyConfig::new()).unwrap();
                    return target_file
                        .write_all(string_data.as_bytes())
                        .map_err(|_e| DataAccessError::new());
                }
            }
            Err(DataAccessError::new())
        }
    }
}

pub fn create_parents_directories_if_not_exist(file_path: &Path) -> Result<(), DataAccessError> {
    if let Some(directory) = file_path.parent() {
        if fs::read_dir(directory).is_err() && fs::create_dir_all(directory).is_err() {
            return Err(DataAccessError::new())
        }
    }
    Ok(())
}

fn data_path(file_path: &str) -> String {
    format!("{}/data/{}", env!("CARGO_MANIFEST_DIR"), file_path)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DataAccessError {}

impl DataAccessError {
    fn new() -> DataAccessError {
        DataAccessError {}
    }
}
