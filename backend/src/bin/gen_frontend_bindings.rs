use std::{fs, path::Path};

use color_eyre::eyre::Result;

fn main() -> Result<()> {
    color_eyre::install()?;

    let bindings = town_hall_backend::ts_bindgen()?;

    let path = Path::new("../frontend/src/lib/generated.ts"); // fragile?
    fs::write(path, bindings)?;

    println!("Bindings exported to: {:?}", fs::canonicalize(path)?);
    Ok(())
}
