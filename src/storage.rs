use std::env::{self, VarError};
use std::error::Error;
use std::fmt::Display;
use std::fs::File;
use std::fs::{self, OpenOptions};
use std::io::{BufReader, BufWriter, Read, Seek, Write};
use std::path::Path;

use crate::model;

use crate::model::*;

pub fn add_item(item: Item) -> Result<()> {
    let mut csv = Csv::new()?;
    csv.file.seek(std::io::SeekFrom::End(0))?;
    writeln!(csv.file, "{}", item.to_string())?;

    Ok(())
}

pub fn update_item(item: Item) -> Result<()> {
    let mut csv = Csv::new()?;
    let content = csv.content()?;

    let old_item_str = content.lines().find(|line| {
        if let Ok(it) = line.parse::<Item>() {
            it.id() == item.id()
        } else {
            false
        }
    });

    let old_item_str = if let Some(str) = old_item_str {
        str
    } else {
        return Err(StorageError::ItemNoExist(item.id()));
    };

    let prev_lines = content.lines().take_while(|line| {
        if let Ok(it) = line.parse::<Item>() {
            it.id() != item.id()
        } else {
            true
        }
    });
    let offset: usize = prev_lines.map(|line| line.len() + 1).sum();
    csv.splice(
        offset as u64,
        old_item_str.len() as u64,
        item.to_string().as_str(),
    )
}

pub fn delete_item(id: u32) -> Result<()> {
    let mut csv = Csv::new()?;
    let content = csv.content()?;

    let old_item_str = content.lines().find(|line| {
        if let Ok(it) = line.parse::<Item>() {
            it.id() == id
        } else {
            false
        }
    });

    let old_item_str = if let Some(str) = old_item_str {
        str
    } else {
        return Err(StorageError::ItemNoExist(id));
    };

    let prev_lines = content.lines().take_while(|line| {
        if let Ok(it) = line.parse::<Item>() {
            it.id() != id
        } else {
            true
        }
    });
    let offset: usize = prev_lines.map(|line| line.len() + 1).sum();
    csv.splice(offset as u64, old_item_str.len() as u64 + 1, "")
}

pub fn get_all() -> Result<Vec<Item>> {
    Ok(Csv::new()?
        .content()?
        .lines()
        .filter_map(|line| line.parse::<Item>().ok())
        .collect())
}

pub fn get_item_by_id(id: u32) -> Result<Item> {
    let content = Csv::new()?.content()?;
    let item_str = content.lines().find(|line| {
        if let Ok(item) = line.parse::<Item>() {
            item.id() == id
        } else {
            false
        }
    });

    if let Some(item_str) = item_str {
        Ok(item_str.parse().unwrap())
    } else {
        Err(StorageError::ItemNoExist(id))
    }
}

pub fn get_max_id() -> Result<u32> {
    let max_id = get_all()?.iter().map(|item| item.id()).reduce(u32::max);

    if let Some(max_id) = max_id {
        Ok(max_id)
    } else {
        Ok(0)
    }
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
        let csv = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(path)?;
        Ok(csv)
    }

    fn open(path: &Path) -> Result<fs::File> {
        let csv = OpenOptions::new().read(true).write(true).open(path)?;
        Ok(csv)
    }
    fn filename() -> Result<String> {
        let home = env::var("HOME")?;
        let filename = home + "/" + CSV_FILE_NAME;
        Ok(filename)
    }

    pub fn content(&mut self) -> Result<String> {
        let mut contents = String::new();
        self.file.read_to_string(&mut contents)?;
        Ok(contents)
    }

    pub fn splice(&mut self, offset: u64, delete_size: u64, write_content: &str) -> Result<()> {
        use std::io::SeekFrom;
        let file = &self.file;
        let mut reader = BufReader::new(file);
        reader.seek(SeekFrom::Start(offset + delete_size))?;
        let mut rest_content = String::new();
        reader.read_to_string(&mut rest_content)?;

        let write_content = write_content.to_owned() + &rest_content;
        let mut writer = BufWriter::new(file);
        writer.seek(SeekFrom::Start(offset))?;
        writer.write_all(write_content.as_bytes())?;
        file.set_len(offset + write_content.len() as u64)?;

        Ok(())
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
    ItemNoExist(u32),
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
            ItemNoExist(id) => write!(f, "Todo item no exist: {}", id),
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
            ItemNoExist(_) => None,
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
