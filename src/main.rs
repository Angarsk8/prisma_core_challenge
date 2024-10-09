mod csv_loader;
mod query_engine;

use csv_loader::load_csv;
use query_engine::{display_results, Query};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let records = load_csv("./data.csv")?;

    let results = Query::new(&records)
        .query("PROJECT col1, col2, col3 FILTER col3 > 5")
        .run();

    display_results(&results);

    Ok(())
}
