use std::{error::Error, fmt::Display};

use chrono::Local;

use crate::{
    model::Item,
    storage::{self, get_item_by_id, get_max_id, update_item, StorageError},
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

pub fn destory_item(id: u32) -> Result<()> {
    storage::delete_item(id)?;
    Ok(())
}

pub fn delete_item(id: u32) -> Result<()> {
    let item = get_item_by_id(id)?;
    update_item(Item {
        deleted: true,
        ..item
    })?;
    Ok(())
}

pub fn list_all() -> Result<()> {
    let items = storage::get_all()?;
    if items.is_empty() {
        println!("Nothing need to do.");
        return Ok(());
    }
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
