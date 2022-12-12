use serde::Deserialize;
use serde::Serialize;
use std::fs::OpenOptions;
use std::path::PathBuf;
use std::io::{Result, Seek, SeekFrom};

#[derive(Debug, Deserialize, Serialize)]
pub struct OpenItem{
    pub name: String,
    pub path: String,
    pub args: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Appsettings{
    settings:Vec<OpenItem>
}

pub fn get_appsettings(file_path: PathBuf) -> Result<Vec<OpenItem>> {
    let file = OpenOptions::new().read(true).open(file_path)?;
    let mut file_operate = &file;
    file_operate.seek(SeekFrom::Start(0))?; // Rewind the file before.

    let appsettings: Appsettings = match serde_json::from_reader(file_operate) {
        Ok(settings) => settings,
        Err(e) => Err(e)?,
    };

    file_operate.seek(SeekFrom::Start(0))?; // Rewind the file after.

    Ok(appsettings.settings)
}