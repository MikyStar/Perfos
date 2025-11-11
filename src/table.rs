use std::io::stdout;

use ascii_table_rs::{AsciiTable, CellValue};
use crossterm::{cursor::MoveToColumn, execute, style::Print};

use crate::{console_ui::ColoredText, file::write};

////////////////////////////////////////

pub fn print_table(
    desc: String,
    titles: Vec<String>,
    data: Vec<Vec<ColoredText>>,
    file_path: Option<String>,
) {
    let mut ascii_table = AsciiTable::new(desc);
    ascii_table.set_headers(titles);

    for row in data.into_iter() {
        ascii_table.add_row(row.iter().map(|f| CellValue::Str(f.to_string())).collect());
    }

    let table = ascii_table.render_to_string();

    if let Some(ref path) = file_path {
        write(path.to_string(), vec![table.clone()]);
    }

    execute!(stdout(), Print(table), Print("\n"), MoveToColumn(0),).unwrap();
}
