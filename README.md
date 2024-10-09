# Prisma Test

Implements a basic query engine written in Rust. The engine processes in-memory CSV data and supports basic query operations, such as selecting specific columns and filtering by a single column using equality and partial ordering operators.

## Features

- **CSV Loading**: loads CSV file into memory.
- **Query Processing**: The query engine supports projections of columns and filtering by equality, greater than, or less than operators.
- **Results Display**: print query results in a formatted table.

## Example (as in [main.rs](./src/main.rs))

```rust
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
```

Prints:

```
+------+------+------+
| col1 | col2 | col3 |
+------+------+------+
| 2    | B    | 6    |
+------+------+------+
| 3    | C    | 10   |
+------+------+------+
```

## Questions and Answers

### What were some of the trade-offs you made when building this application? Why were these acceptable trade-offs?

One of the first decisions I made was to use a Vec<(String, String)> to store the columns of each record, rather than a HashMap (or even better an IndexMap). The reason for this was that a Vec preserves the order of columns. However, this choice means that accessing a specific column by name is a little less efficient since I have to iterate over the vector rather than using a direct lookup. I think this trade-off is fine considering the constraints.

Another trade-off was to limit the query engine to only handling one filter condition per query. Adding support for multiple conditions, with different types of operations is in principle a much bigger task, and would require more time.

Finally, I went with an in-memory approach for handling the data. While this works well for smaller datasets, it won’t scale well for large ones. But since the dataset here is very small, loading everything into memory at once simplifies the design and is an acceptable trade-off in the meantime.

###  Given more time, what improvements or optimizations would you want to add later?

the first thing I’d focus on is supporting more complex queries with multiple filters. Right now, you can only filter based on a single condition, which renders it useless :D. Adding support for AND/OR logic would allow for much more flexible querying too.

I’d also like to expand the types of data the engine can handle, and have the query engine adapt to those types accordingly.

Another area that could use improvement is error handling/logging. There's basically none.

### How would you accommodate changes to support other data types, multiple filters, or ordering of results?

For supporting more data types, I’d refactor how the columns are stored. Right now, everything is treated as either a string or a number. I’d introduce a more flexible system where each value is typed as part of an enum, like Int(i32), String(String), Bool(bool), etc.

To support multiple filters, I’d extend the query parser to handle AND and OR conditions. This would involve tweaking the filtering logic to evaluate multiple conditions and combine them logically. It would also require parsing and managing more complex expressions, which is doable although challenging.

For ordering results, I’d add support for an "ORDER BY" clause or similar. This could be implemented by sorting the records based on the specified column before retrieving the result.

4. How would you process extremely large datasets?
For extremely large datasets, I’d definitely move away from the in-memory approach. Instead, I’d implement a streaming model where only chunks of data are loaded into memory at a time. This could be done by reading the CSV file in steps and applying the filters incrementally as the data is streamed in. This way, the engine would only keep what’s necessary in memory (potentially allowing it to handle datasets as large as the system’s memory or larger).

