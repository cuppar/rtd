#![allow(unused)]
use crate::model::{self, *};
use std::fs::File;

pub fn add_item(item: Item) -> Result<()> {
    println!("add_item");
    todo!();
}

pub fn update_item(item: Item) -> Result<()> {
    println!("update_item");
    todo!();
}

pub fn delete_item(id: u32) -> Result<()> {

    todo!();
}

pub fn get_all() -> Result<Vec<Item>> {
    todo!();
}

pub fn get_item_by_id(id: u32) -> Result<Item> {
    todo!();
}

pub fn get_max_id() -> Result<u32> {
    todo!();
}

struct Csv {
    filename: String,
    file: File,
}

impl Csv {
    fn new() -> Result<Self> {
        todo!();
    }

    fn filename() -> Result<String> {
        todo!();
    }

    fn content(&mut self) -> Result<String> {
        todo!();
    }

    fn splice(&mut self, offset: u64, delete_size: u64, write_content: &str) -> Result<()> {
        todo!();
    }
}

type Result<T> = std::result::Result<T, StorageError>;

#[derive(Debug)]
pub enum StorageError {
}