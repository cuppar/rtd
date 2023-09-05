#![allow(unused)]
use crate::storage::{self, add_item as storage_add_item, update_item};

pub fn add_item(name: &str) -> Result<String> {
    todo!();
}

pub fn complete_item(id: u32) -> Result<String> {
    todo!();
}

pub fn uncomplete_item(id: u32) -> Result<String> {
    todo!();
}

pub fn delete_item(id: u32) -> Result<String> {
    todo!();
}

pub fn restore_item(id: u32) -> Result<String> {
    todo!();
}

pub fn destroy_deleted() -> Result<String> {
    todo!();
}

pub fn destroy_item(id: u32) -> Result<String> {
    todo!();
}

pub fn clear() -> Result<String> {
    todo!();
}

pub fn list_uncompleted() -> Result<String> {
    storage::get_all();
    todo!();
}

pub fn list_completed() -> Result<String> {
    storage::get_all();
    todo!();
}

pub fn list_deleted() -> Result<String> {
    storage::get_all();
    todo!();
}

pub fn list_all() -> Result<String> {
    storage::get_all();
    todo!();
}

type Result<T> = std::result::Result<T, ServiceError>;

#[derive(Debug)]
pub enum ServiceError {
}