mod first;
mod second;

use anyhow::Result;

fn main() -> Result<()> {
    // first::main()?;
    second::main()?;

    Ok(())
}
