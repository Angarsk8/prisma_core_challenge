use csv::ReaderBuilder;
use serde::Deserialize;
use std::error::Error;

#[derive(Debug, Deserialize)]
pub struct Record {
    pub columns: Vec<(String, String)>,
}

pub fn load_csv(filename: &str) -> Result<Vec<Record>, Box<dyn Error>> {
    let mut reader = ReaderBuilder::new().flexible(true).from_path(filename)?;
    let headers = reader.headers()?.clone();

    let records = reader
        .records()
        .map(|result| {
            let record = result?;
            let columns = headers
                .iter()
                .zip(record.iter())
                .map(|(header, field)| (header.to_string(), field.to_string())) // Create tuples
                .collect::<Vec<(String, String)>>();
            Ok(Record { columns })
        })
        .collect();

    records
}
