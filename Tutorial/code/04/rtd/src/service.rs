use crate::storage::{self, add_item as storage_add_item, update_item};

pub fn add_item() {
    storage_add_item();
}

pub fn complete_item() {
    update_item();
}

pub fn uncomplete_item() {
    update_item();
}

pub fn delete_item() {
    update_item();
}

pub fn restore_item() {
    update_item();
}

pub fn destroy_deleted() {
    storage::delete_item();
}

pub fn destroy_item() {
    storage::delete_item();
}

pub fn clear() {
    storage::delete_item();
}

pub fn list_uncompleted() {
    storage::get_all();
}

pub fn list_completed() {
    storage::get_all();
}

pub fn list_deleted() {
    storage::get_all();
}

pub fn list_all() {
    storage::get_all();
}
