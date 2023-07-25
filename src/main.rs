use std::error::Error;

use rtd::*;

fn main() -> Result<(), Box<dyn Error>> {
    // add_item("hello world!")?;
    delete_item(5)?;
    list_all()?;
    Ok(())
}
