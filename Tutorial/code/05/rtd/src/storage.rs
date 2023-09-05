#![allow(unused)]

use std::fs::File;

pub fn add_item() {
    println!("add_item");
}

pub fn update_item() {
    println!("update_item");
}

pub fn delete_item() {}

pub fn get_all() {}

pub fn get_item_by_id() {}

pub fn get_max_id() {}

struct Csv {
    filename: String,
    file: File,
}

impl Csv {
    fn new() {}

    fn filename() {}

    fn content() {}

    fn splice() {}
}
