use std::io;

use ufodb_v0::Ufdb;

pub fn save(ufdb: &Ufdb, db_name: &str) -> io::Result<()> {
    std::fs::create_dir_all("./ufo_data")?;

    let json = serde_json::to_string_pretty(ufdb)?;

    let path = format!("./ufo_data/{db_name}.json");

    std::fs::write(path, json)?;

    Ok(())
}
