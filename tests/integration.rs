use prisma_core_challenge::{csv_loader::load_csv, csv_loader::Record, query_engine::Query};
use std::collections::HashMap;
use std::error::Error;
use std::path::Path;

fn load_test_data() -> Result<Vec<Record>, Box<dyn Error>> {
    let path = Path::new("./data.csv");
    let records = load_csv(path.to_str().unwrap())?;
    Ok(records)
}

fn record_to_hashmap(record: &Record) -> HashMap<String, String> {
    record.columns.iter().cloned().collect()
}

#[test]
fn test_query_filter_gt() -> Result<(), Box<dyn Error>> {
    let records = load_test_data()?;

    let query_str = "PROJECT col1, col2 FILTER col3 > 5";
    let result = Query::new(&records).query(query_str).run();

    assert_eq!(result.len(), 2);

    let record0_map = record_to_hashmap(&result[0]);
    assert_eq!(record0_map.get("col1"), Some(&"2".to_string()));
    assert_eq!(record0_map.get("col2"), Some(&"B".to_string()));

    let record1_map = record_to_hashmap(&result[1]);
    assert_eq!(record1_map.get("col1"), Some(&"3".to_string()));
    assert_eq!(record1_map.get("col2"), Some(&"C".to_string()));

    Ok(())
}

#[test]
fn test_query_filter_lt() -> Result<(), Box<dyn Error>> {
    let records = load_test_data()?;

    let query_str = "PROJECT col1, col2 FILTER col3 < 5";
    let result = Query::new(&records).query(query_str).run();

    assert_eq!(result.len(), 2);

    let record0_map = record_to_hashmap(&result[0]);
    assert_eq!(record0_map.get("col1"), Some(&"1".to_string()));
    assert_eq!(record0_map.get("col2"), Some(&"A".to_string()));

    let record1_map = record_to_hashmap(&result[1]);
    assert_eq!(record1_map.get("col1"), Some(&"4".to_string()));
    assert_eq!(record1_map.get("col2"), Some(&"D".to_string()));

    Ok(())
}

#[test]
fn test_query_filter_equal() -> Result<(), Box<dyn Error>> {
    let records = load_test_data()?;

    let query_str = "PROJECT col1 FILTER col3 = 6";
    let result = Query::new(&records).query(query_str).run();

    assert_eq!(result.len(), 1);

    let record_map = record_to_hashmap(&result[0]);
    assert_eq!(record_map.get("col1"), Some(&"2".to_string()));

    Ok(())
}

#[test]
fn test_query_filter_string_comparisson() -> Result<(), Box<dyn Error>> {
    let records = load_test_data()?;

    let query_str = "PROJECT col1, col2 FILTER col2 = A";
    let result = Query::new(&records).query(query_str).run();

    assert_eq!(result.len(), 1);

    let record_map = record_to_hashmap(&result[0]);
    assert_eq!(record_map.get("col1"), Some(&"1".to_string()));
    assert_eq!(record_map.get("col2"), Some(&"A".to_string()));

    Ok(())
}
