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
    storage::add_item(item.clone())?;
    println!("Added [{}]: {}\n", item.id, item.name);
    Ok(())
}

pub fn destroy_item(id: u32) -> Result<()> {
    storage::delete_item(id)?;
    println!("Destroyed [{}]\n", id);
    Ok(())
}

pub fn destroy_deleted() -> Result<()> {
    let items = storage::get_all()?
        .into_iter()
        .filter(|item| item.deleted)
        .collect::<Vec<_>>();
    if items.is_empty() {
        println!("Nothing to destory.");
        return Ok(());
    }
    for item in items {
        destroy_item(item.id)?;
    }
    println!("All deleted todos were destroyed.\n");
    Ok(())
}

pub fn clear() -> Result<()> {
    let items = storage::get_all()?;
    if items.is_empty() {
        println!("Nothing to clear.");
        return Ok(());
    }
    for item in items {
        destroy_item(item.id)?;
    }
    println!("All todos were destroyed.\n");
    Ok(())
}

pub fn uncomplete_item(id: u32) -> Result<()> {
    let item = get_item_by_id(id)?;
    update_item(Item {
        completed: false,
        completed_at: None,
        ..item.clone()
    })?;
    println!("Uncompleted [{}]: {}\n", item.id, item.name);
    Ok(())
}

pub fn complete_item(id: u32) -> Result<()> {
    let item = get_item_by_id(id)?;
    update_item(Item {
        completed: true,
        completed_at: Some(Local::now().timestamp()),
        ..item.clone()
    })?;
    println!("Completed [{}]: {}\n", item.id, item.name);
    Ok(())
}

pub fn delete_item(id: u32) -> Result<()> {
    let item = get_item_by_id(id)?;
    update_item(Item {
        deleted: true,
        deleted_at: Some(Local::now().timestamp()),
        ..item.clone()
    })?;
    println!("Deleted [{}]: {}\n", item.id, item.name);
    Ok(())
}


pub fn list_uncompleted() -> Result<()> {
    let items = storage::get_all()?
        .into_iter()
        .filter(|item| !item.completed)
        .collect::<Vec<_>>();
    if items.is_empty() {
        println!("Nothing need to do.");
        return Ok(());
    }
    println!("Uncompleted todos:\n");
    for item in items {
        println!("{}", item.show());
    }
    Ok(())
}

pub fn list_completed() -> Result<()> {
    let items = storage::get_all()?
        .into_iter()
        .filter(|item| item.completed)
        .collect::<Vec<_>>();
    if items.is_empty() {
        println!("Nothing completed.");
        return Ok(());
    }
    println!("Completed todos:\n");
    for item in items {
        println!("{}", item.show());
    }
    Ok(())
}
pub fn list_deleted() -> Result<()> {
    let items = storage::get_all()?
        .into_iter()
        .filter(|item| item.deleted)
        .collect::<Vec<_>>();
    if items.is_empty() {
        println!("Nothing deleted.");
        return Ok(());
    }
    println!("Deleted todos:\n");
    for item in items {
        println!("{}", item.show());
    }
    Ok(())
}

pub fn list_all() -> Result<()> {
    let items = storage::get_all()?;
    if items.is_empty() {
        println!("Nothing need to do.");
        return Ok(());
    }
    println!("All todos:\n");
    for item in items {
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
