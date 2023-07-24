use std::env::{self, VarError};
use std::error::Error;
use std::fmt::Display;
use std::fs::File;
use std::fs::{self, OpenOptions};
use std::io::{Read, Write};
use std::path::Path;

use crate::model;

use super::model::*;

pub fn add_item(item: Item) -> Result<()> {
    let mut csv = Csv::new()?;
    writeln!(csv.file, "{}", item.to_string())?;
    Ok(())
}

pub fn get_all() -> Result<Vec<Item>> {
    let mut csv = Csv::new()?;
    let mut contents = String::new();
    csv.file.read_to_string(&mut contents)?;

    let items = contents
        .lines()
        .filter_map(|line| line.parse::<Item>().ok())
        .collect::<Vec<Item>>();

    Ok(items)
}

pub fn get_max_id() -> Result<u32> {
    Ok(1)
}

#[allow(unused)]
struct Csv {
    filename: String,
    file: File,
}

impl Csv {
    fn new() -> Result<Self> {
        let filename = Csv::filename()?;
        let path = Path::new(&filename);
        if !path.exists() {
            let mut file = Csv::create(path)?;
            file.write_all(b"id,name,completed,deleted,createdAt,completedAt,deletedAt\n")?;
            Ok(Self {
                filename: filename.to_string(),
                file,
            })
        } else {
            Ok(Self {
                filename: filename.to_string(),
                file: Csv::open(path)?,
            })
        }
    }

    fn create(path: &Path) -> Result<fs::File> {
        let csv = OpenOptions::new().write(true).create(true).open(path)?;
        Ok(csv)
    }

    fn open(path: &Path) -> Result<fs::File> {
        let csv = OpenOptions::new()
            .read(true)
            .write(true)
            .append(true)
            .open(path)?;
        Ok(csv)
    }
    fn filename() -> Result<String> {
        let home = env::var("HOME")?;
        let filename = home + "/" + CSV_FILE_NAME;
        Ok(filename)
    }
}

#[allow(unused)]
mod tests {
    use super::*;
    use std::io::Read;

    #[test]
    fn create_file() -> Result<()> {
        let csv = Csv::new()?;
        Ok(())
    }

    #[test]
    fn title_line() -> Result<()> {
        let mut csv = Csv::new()?;
        let mut content = String::new();
        csv.file.read_to_string(&mut content)?;
        let lines: Vec<&str> = content.lines().collect();
        assert!(!lines.is_empty());
        assert_eq!(
            lines[0],
            "id,name,completed,deleted,createdAt,completedAt,deletedAt"
        );

        Ok(())
    }
}

const CSV_FILE_NAME: &str = ".rtd.csv";

type Result<T> = std::result::Result<T, StorageError>;

#[derive(Debug)]
pub enum StorageError {
    FileErr { source: FileHandleError },
    ParseErr(ItemError),
}

#[derive(Debug)]
pub enum FileHandleError {
    VarErr(VarError),
    IoErr(std::io::Error),
}

impl Display for StorageError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use FileHandleError::*;
        use StorageError::*;
        match &self {
            FileErr { source } => match source {
                VarErr(e) => write!(f, "rtd storage file handle env var error: {}", e),
                IoErr(e) => write!(f, "rtd storage file handle io error: {}", e),
            },
            ParseErr(e) => write!(f, "{}", e),
        }
    }
}

impl Error for StorageError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        use FileHandleError::*;
        use StorageError::*;
        match self {
            FileErr { source } => match source {
                VarErr(source) => Some(source),
                IoErr(source) => Some(source),
            },
            ParseErr(e) => Some(e),
        }
    }
}

impl From<VarError> for StorageError {
    fn from(value: VarError) -> Self {
        Self::FileErr {
            source: FileHandleError::VarErr(value),
        }
    }
}

impl From<std::io::Error> for StorageError {
    fn from(value: std::io::Error) -> Self {
        Self::FileErr {
            source: FileHandleError::IoErr(value),
        }
    }
}

impl From<model::ItemError> for StorageError {
    fn from(value: model::ItemError) -> Self {
        Self::ParseErr(value)
    }
}
