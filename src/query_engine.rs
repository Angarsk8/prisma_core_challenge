use crate::csv_loader::Record;
use prettytable::{Cell, Row, Table};

pub struct Query<'a> {
    records: &'a [Record],
    projection: Vec<String>,
    filter_column: String,
    filter_operation: String,
    filter_value: String,
    results: Vec<Record>,
}

impl<'a> Query<'a> {
    pub fn new(records: &'a [Record]) -> Self {
        Self {
            records,
            projection: Vec::new(),
            filter_column: String::new(),
            filter_operation: String::new(),
            filter_value: String::new(),
            results: Vec::new(),
        }
    }

    pub fn query(mut self, query_str: &str) -> Self {
        let segments: Vec<&str> = query_str.split_whitespace().collect();

        let mut i = 0;
        while i < segments.len() {
            match segments[i] {
                "PROJECT" | "project" => {
                    i += 1;
                    while i < segments.len() && !matches!(segments[i], "FILTER" | "filter") {
                        self.projection.push(segments[i].replace(",", ""));
                        i += 1;
                    }
                }
                "FILTER" | "filter" => {
                    i += 1;
                    self.filter_column = segments[i].to_string();
                    self.filter_operation = segments[i + 1].to_string();
                    self.filter_value = segments[i + 2].to_string();
                    i += 3;
                }
                _ => i += 1,
            }
        }

        self
    }

    pub fn run(mut self) -> Vec<Record> {
        self.results = self
            .records
            .iter()
            .filter(|record| self.filter_record(record))
            .map(|record| self.project_columns(record))
            .collect();

        self.results
    }

    fn filter_record(&self, record: &Record) -> bool {
        if let Some((_, value)) = record
            .columns
            .iter()
            .find(|(col, _)| col == &self.filter_column)
        {
            return self.compare(value, &self.filter_value, &self.filter_operation);
        }
        false
    }

    fn compare(&self, value: &str, filter_value: &str, operation: &str) -> bool {
        let result = (value.parse::<i32>(), filter_value.parse::<i32>());

        match result {
            (Ok(value), Ok(filter_value)) => self.apply_comparison(value, filter_value, operation),
            _ => self.apply_comparison(value, filter_value, operation),
        }
    }

    fn apply_comparison<T: PartialOrd>(&self, a: T, b: T, operation: &str) -> bool {
        match operation {
            ">" => a > b,
            "<" => a < b,
            "=" => a == b,
            _ => false,
        }
    }

    fn project_columns(&self, record: &Record) -> Record {
        let filtered_columns: Vec<(String, String)> = record
            .columns
            .iter()
            .filter(|(col, _)| self.projection.contains(&col.as_str().to_string()))
            .cloned()
            .collect();

        Record {
            columns: filtered_columns,
        }
    }
}

pub fn display_results(records: &[Record]) {
    if records.is_empty() {
        println!("No results found.");
        return;
    }

    let headers: Vec<&str> = records[0]
        .columns
        .iter()
        .map(|(col, _)| col.as_str())
        .collect();

    let mut table = Table::new();
    table.add_row(Row::new(headers.iter().map(|h| Cell::new(h)).collect()));

    for record in records {
        let row = Row::new(
            headers
                .iter()
                .map(|&header| {
                    let value = record
                        .columns
                        .iter()
                        .find(|(col, _)| col == header)
                        .map(|(_, value)| value.as_str())
                        .unwrap_or("");
                    Cell::new(value)
                })
                .collect(),
        );
        table.add_row(row);
    }

    table.printstd();
}
