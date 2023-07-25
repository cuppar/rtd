use std::error::Error;

use rtd::*;

fn main() -> Result<(), Box<dyn Error>> {
    // add_item("name2")?;
    // delete_item(10)?;
    // list_all()?;
    // list_uncompleted()?;
    list_completed()?;
    clear()?;
    list_all()?;

    // destroy_item(1)?;
    // destroy_deleted()?;
    Ok(())
}
