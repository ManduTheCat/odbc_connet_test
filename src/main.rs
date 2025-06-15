// ...existing code...
use prettytable::{Table, Row, Cell}; // 추가
use anyhow::Error;
use odbc_api::{buffers::TextRowSet, Cursor, Environment, ConnectionOptions, ResultSetMetadata};
use std::{
    ffi::CStr,
    io::{stdout, Write},
    path::PathBuf,
};
const BATCH_SIZE: usize = 5000;
fn main() -> Result<(), Error> {
    // 표 생성
    let mut table = Table::new();

    let environment = Environment::new()?;
    let mut connection = environment.connect(
        "Tibero7",
        "sys",
        "tibero",
        ConnectionOptions::default(),
    )?;

    match connection.execute("SELECT NAME FROM v$database", (), None)? {
        Some(mut cursor) => {
            // 컬럼명 표 첫 줄에 추가
            let headline: Vec<String> = cursor.column_names()?.collect::<Result<_,_>>()?;
            table.add_row(Row::new(headline.iter().map(|h| Cell::new(h)).collect()));

            let mut buffers = TextRowSet::for_cursor(BATCH_SIZE, &mut cursor, Some(4096))?;
            let mut row_set_cursor = cursor.bind_buffer(&mut buffers)?;

            while let Some(batch) = row_set_cursor.fetch()? {
                for row_index in 0..batch.num_rows() {
                    let record: Vec<String> = (0..batch.num_cols())
                        .map(|col_index| {
                            let bytes = batch.at(col_index, row_index).unwrap_or(&[]);
                            String::from_utf8_lossy(bytes).to_string()
                        })
                        .collect();
                    table.add_row(Row::new(record.iter().map(|v| Cell::new(v)).collect()));
                }
            }
            table.printstd(); // 표 출력
        }
        None => {
            eprintln!("Query came back empty. No output has been created.");
        }
    }

    Ok(())
}
// ...existing code...
