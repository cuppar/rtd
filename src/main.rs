use std::error::Error;

use rtd::*;

fn main() -> Result<(), Box<dyn Error>> {
    add_item("hello world!")?;
    add_item("hello cuppar!")?;
    list_all()?;
    Ok(())
}
