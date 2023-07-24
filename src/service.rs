use std::{error::Error, fmt::Display};

use chrono::Local;

use crate::{
    model::Item,
    storage::{self, get_max_id, StorageError},
};

pub fn add_item(name: &str) -> Result<()> {
    let max_id = get_max_id()?;
    let item = Item::new(
        max_id + 1,
        name,
        false,
        false,
        Some(Local::now().timestamp()),
        None,
        None,
    );
    storage::add_item(item)?;
    Ok(())
}

pub fn list_all() -> Result<()> {
    let items = storage::get_all()?;
    for item in items.into_iter() {
        println!("{}", item.show());
    }
    Ok(())
}

type Result<T> = std::result::Result<T, ServiceError>;

#[derive(Debug)]
pub enum ServiceError {
    StorageErr(StorageError),
}

impl Error for ServiceError {}
impl Display for ServiceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use ServiceError::*;
        match self {
            StorageErr(e) => write!(f, "rtd service storage error: {}", e),
        }
    }
}

impl From<StorageError> for ServiceError {
    fn from(value: StorageError) -> Self {
        Self::StorageErr(value)
    }
}
