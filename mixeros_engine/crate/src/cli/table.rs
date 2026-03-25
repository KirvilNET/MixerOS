use comfy_table::{Table, Cell, Color, Attribute, ContentArrangement};
use crossterm::{
    cursor,
    terminal::{self, ClearType},
    ExecutableCommand,
};
use std::io::{stdout, Write};

pub struct LiveTable {
    headers: Vec<String>,
    rows: Vec<Vec<String>>,
    title: Option<String>,
    row_count: u16, // track how many lines to erase
}

impl LiveTable {
    pub fn new(headers: Vec<&str>) -> Self {
        Self {
            headers: headers.iter().map(|s| s.to_string()).collect(),
            rows: vec![],
            title: None,
            row_count: 0,
        }
    }

    pub fn title(mut self, title: &str) -> Self {
        self.title = Some(title.to_string());
        self
    }

    pub fn set_rows(&mut self, rows: Vec<Vec<String>>) {
        self.rows = rows;
    }

    pub fn update_row(&mut self, index: usize, row: Vec<String>) {
        if index < self.rows.len() {
            self.rows[index] = row;
        }
    }

    pub fn add_row(&mut self, row: Vec<String>) {
        self.rows.push(row);
    }

    pub fn clear_rows(&mut self) {
        self.rows.clear();
    }

    fn build_table(&self) -> Table {
        let mut table = Table::new();
        table.set_content_arrangement(ContentArrangement::Dynamic);

        if let Some(title) = &self.title {
            table.set_header(vec![
                Cell::new(title)
                    .add_attribute(Attribute::Bold)
                    .fg(Color::Cyan),
            ]);
        }

        // Header row
        table.set_header(
            self.headers.iter().map(|h| {
                Cell::new(h)
                    .add_attribute(Attribute::Bold)
                    .fg(Color::Yellow)
            })
        );

        // Data rows
        for row in &self.rows {
            table.add_row(row.iter().map(|c| Cell::new(c)));
        }

        table
    }

    /// Draw the table, erasing the previous draw first
    pub fn draw(&mut self) {
        let mut out = stdout();

        // Erase previous draw by moving up and clearing
        if self.row_count > 0 {
            out.execute(cursor::MoveUp(self.row_count)).ok();
            out.execute(terminal::Clear(ClearType::FromCursorDown)).ok();
        }

        let table = self.build_table();
        let rendered = table.to_string();

        // Count lines so we know how far to move up next time
        self.row_count = rendered.lines().count() as u16;

        println!("{}", rendered);
        out.flush().ok();
    }
}