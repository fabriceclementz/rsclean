use rsclean::Config;
use std::io::Result;

fn main() -> Result<()> {
    let config = Config::parse();

    config.run()?;

    Ok(())
}
