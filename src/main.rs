use std::error::Error;

use rtd::*;

fn main() -> Result<(), Box<dyn Error>> {
    add_item("hello world!")?;
    list_all()?;
    Ok(())
}
