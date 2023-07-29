use crate::{
    model::Item,
    storage::{self, get_item_by_id, get_max_id, update_item, StorageError},
};
use chrono::Local;
use std::{error::Error, fmt::Display};

pub fn add_item(name: &str) -> Result<String> {
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
    storage::add_item(item.clone())?;
    Ok(format!("Added [{}]: {}\n", item.id, item.name))
}

pub fn complete_item(id: u32) -> Result<String> {
    let item = get_item_by_id(id)?;
    update_item(Item {
        completed: true,
        completed_at: Some(Local::now().timestamp()),
        ..item.clone()
    })?;
    Ok(format!("Completed [{}]: {}\n", item.id, item.name))
}

pub fn uncomplete_item(id: u32) -> Result<String> {
    let item = get_item_by_id(id)?;
    update_item(Item {
        completed: false,
        completed_at: None,
        ..item.clone()
    })?;
    Ok(format!("Uncompleted [{}]: {}\n", item.id, item.name))
}

pub fn delete_item(id: u32) -> Result<String> {
    let item = get_item_by_id(id)?;
    update_item(Item {
        deleted: true,
        deleted_at: Some(Local::now().timestamp()),
        ..item.clone()
    })?;
    Ok(format!("Deleted [{}]: {}\n", item.id, item.name))
}

pub fn restore_item(id: u32) -> Result<String> {
    let item = get_item_by_id(id)?;
    update_item(Item {
        deleted: false,
        deleted_at: None,
        ..item.clone()
    })?;
    Ok(format!("Restored [{}]: {}\n", item.id, item.name))
}

pub fn destroy_deleted() -> Result<String> {
    let items = storage::get_all()?
        .into_iter()
        .filter(|item| item.deleted)
        .collect::<Vec<_>>();
    if items.is_empty() {
        return Ok("Nothing to destory.".to_string());
    }
    for item in items {
        destroy_item(item.id)?;
    }
    Ok("All deleted todos were destroyed.\n".to_string())
}

pub fn destroy_item(id: u32) -> Result<String> {
    storage::delete_item(id)?;
    Ok(format!("Destroyed [{}]\n", id))
}

pub fn clear() -> Result<String> {
    let items = storage::get_all()?;
    if items.is_empty() {
        return Ok("Nothing to clear.".to_string());
    }
    for item in items {
        destroy_item(item.id)?;
    }
    Ok("All todos were destroyed.\n".to_string())
}

pub fn list_uncompleted() -> Result<String> {
    let items = storage::get_all()?
        .into_iter()
        .filter(|item| !item.deleted && !item.completed)
        .collect::<Vec<_>>();
    if items.is_empty() {
        return Ok("Nothing need to do.".to_string());
    }
    let mut result = "Uncompleted todos:\n\n".to_string();
    for item in items {
        result += &item.to_prettier_string();
    }
    Ok(result)
}

pub fn list_completed() -> Result<String> {
    let items = storage::get_all()?
        .into_iter()
        .filter(|item| !item.deleted && item.completed)
        .collect::<Vec<_>>();
    if items.is_empty() {
        return Ok("Nothing completed.".to_string());
    }
    let mut result = "Completed todos:\n\n".to_string();
    for item in items {
        result += &item.to_prettier_string();
    }
    Ok(result)
}

pub fn list_deleted() -> Result<String> {
    let items = storage::get_all()?
        .into_iter()
        .filter(|item| item.deleted)
        .collect::<Vec<_>>();
    if items.is_empty() {
        return Ok("Nothing deleted.".to_string());
    }
    let mut result = "Deleted todos:\n\n".to_string();
    for item in items {
        result += &item.to_prettier_string();
    }
    Ok(result)
}

pub fn list_all() -> Result<String> {
    let items = storage::get_all()?;
    if items.is_empty() {
        return Ok("Nothing need to do.".to_string());
    }
    let mut result = "All todos:\n\n".to_string();
    for item in items {
        result += &item.to_prettier_string();
    }
    Ok(result)
}

type Result<T> = std::result::Result<T, ServiceError>;

#[derive(Debug)]
pub enum ServiceError {
    Storage(StorageError),
}

impl Error for ServiceError {}

impl Display for ServiceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use ServiceError::*;
        match self {
            Storage(e) => writeln!(f, "Rtd service storage error: {}", e),
        }
    }
}

impl From<StorageError> for ServiceError {
    fn from(value: StorageError) -> Self {
        Self::Storage(value)
    }
}
