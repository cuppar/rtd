use clap::{Parser, ValueEnum};
use rtd_tutorial::*;

/// Rust To Do, tutorial: https://github.com/cuppar/rtd
#[derive(Parser, Debug)]
#[command(
    author = "Cuppar He <cuppar.hzy@gmail.com>",
    version = "0.1.2",
    long_about = "A simple todo app write by Rust.\nYou can use it to make life pleasant or use it to learn the Rust language!\nLearn Rust in 500 lines of code tutorial: https://github.com/cuppar/rtd"
)]
struct Args {
    /// Name of todo item to add
    #[arg(short, long, value_name = "item-name")]
    add: Option<String>,

    /// Id of item to complete
    #[arg(short, long, value_name = "item-id")]
    complete: Option<u32>,

    /// Id of item to uncomplete
    #[arg(short, long, value_name = "item-id")]
    uncomplete: Option<u32>,

    /// Id of item to delete, rtd use lazy delete,
    /// this just mark the item to `deleted`,
    /// it will not destroy data record,
    /// if you want to destory a item, use `--destroy` option.
    #[arg(short, long, value_name = "item-id")]
    delete: Option<u32>,

    /// restore a deleted todo item
    #[arg(short, long, value_name = "item-id")]
    restore: Option<u32>,

    /// Id of item to destroy, this will real destroy data record,
    /// use `--delete` to logic delete a todo item
    #[arg(long, value_name = "item-id")]
    destroy: Option<u32>,

    /// destory all `deleted` marked todo items,
    /// this will real destroy data records,
    /// use `--delete` to logic delete a todo item
    #[arg(long)]
    destroy_deleted: bool,

    /// Clear all records, make all list empty
    #[arg(long, value_name = "item-id")]
    clear: bool,

    /// List todo items
    #[arg(short, long, value_name = "list-type")]
    list: Option<Option<ListType>>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum ListType {
    /// All todo items
    All,

    /// All completed todo tiems [default]
    Completed,

    /// All uncompleted todo tiems
    Uncompleted,

    /// All deleted todo tiems
    Deleted,
}

fn main() {
    let args = Args::parse();

    if let Some(name) = args.add {
        match add_item(&name) {
            Ok(s) => println!("{s}"),
            Err(e) => eprintln!("Add '{name}' fail: {e}"),
        }
    }

    if let Some(id) = args.complete {
        match complete_item(id) {
            Ok(s) => println!("{s}"),
            Err(e) => eprintln!("Complete todo '{id}' fail: {e}"),
        }
    }

    if let Some(id) = args.uncomplete {
        match uncomplete_item(id) {
            Ok(s) => println!("{s}"),
            Err(e) => eprintln!("Uncomplete todo '{id}' fail: {e}"),
        }
    }

    if let Some(id) = args.delete {
        match delete_item(id) {
            Ok(s) => println!("{s}"),
            Err(e) => eprintln!("Delete todo '{id}' fail: {e}"),
        }
    }

    if let Some(id) = args.restore {
        match restore_item(id) {
            Ok(s) => println!("{s}"),
            Err(e) => eprintln!("Restore todo '{id}' fail: {e}"),
        }
    }

    // destroy operation need be after all operation which need a todo exist
    // or it can NOT find a todo when user execute the operation and destroy it at same time
    if let Some(id) = args.destroy {
        match destroy_item(id) {
            Ok(s) => println!("{s}"),
            Err(e) => eprintln!("Destroy todo '{id}' fail: {e}"),
        };
    }

    // destroy operation need be after all operation which need a todo exist
    // or it can NOT find a todo when user execute the operation and destroy it at same time
    if args.destroy_deleted {
        match destroy_deleted() {
            Ok(s) => println!("{s}"),
            Err(e) => eprintln!("Destroy all deleted todos fail: {e}"),
        };
    }

    // destroy operation need be after all operation which need a todo exist
    // or it can NOT find a todo when user execute the operation and destroy it at same time
    if args.clear {
        match clear() {
            Ok(s) => println!("{s}"),
            Err(e) => eprintln!("Clear all todos fail: {e}"),
        };
    }

    let mut already_listed = false;

    if let Some(None) = args.list {
        default_list();
        already_listed = true;
    }

    if let Some(Some(list_type)) = args.list {
        use ListType::*;
        match list_type {
            All => match list_all() {
                Ok(s) => println!("{s}"),
                Err(e) => eprint!("List all todos fail: {e}"),
            },
            Completed => match list_completed() {
                Ok(s) => println!("{s}"),
                Err(e) => eprint!("List completed todos fail: {e}"),
            },
            Uncompleted => default_list(),
            Deleted => match list_deleted() {
                Ok(s) => println!("{s}"),
                Err(e) => eprint!("List deleted todos fail: {e}"),
            },
        }
        already_listed = true;
    }

    if !already_listed {
        default_list();
    }
}

fn default_list() {
    match list_uncompleted() {
        Ok(s) => println!("{s}"),
        Err(e) => eprint!("List uncompleted todos fail: {e}"),
    }
}
