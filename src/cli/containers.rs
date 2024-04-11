use comfy_table::{Cell, Table};
use container_rack_lib::rack::Container;
use container_rack_lib::supported_containers;

/// Print containers to CLI
pub fn print_containers() {
    let containers = supported_containers();
    let mut table = Table::new();
    table.set_header(table_header());

    containers.into_iter().for_each(|container| {
        table.add_row(container.to_table_row());
    });

    println!("{table}");
}

fn table_header() -> Vec<Cell> {
    vec![
        Cell::new("Brand"),
        Cell::new("Model"),
        Cell::new("Description"),
        Cell::new("Links"),
    ]
}

trait ToTableRow {
    fn to_table_row(&self) -> Vec<Cell>;
}

impl ToTableRow for Container {
    fn to_table_row(&self) -> Vec<Cell> {
        vec![
            Cell::new(&self.vendor),
            Cell::new(&self.model),
            Cell::new(&self.description).truncate(40),
            Cell::new(&self.links.join("\n")),
        ]
    }
}

trait TruncatedCell {
    fn truncate(&self, max_length: usize) -> Cell;
}

impl TruncatedCell for Cell {
    fn truncate(&self, max_length: usize) -> Cell {
        let content = self.content();

        if content.len() > max_length {
            Cell::new(content.truncate_words(max_length).join("\n"))
        } else {
            self.clone()
        }
    }
}

trait TruncateWords {
    fn truncate_words(self, max_length: usize) -> Vec<String>;
}
impl TruncateWords for &str {
    fn truncate_words(self, chunk_size: usize) -> Vec<String> {
        let mut chunks = Vec::new();
        let mut chunk = String::new();

        for word in self.split_whitespace() {
            if chunk.len() + word.len() > chunk_size {
                chunks.push(chunk);
                chunk = String::from(word);
            } else {
                if !chunk.is_empty() {
                    chunk.push(' ');
                }
                chunk.push_str(word);
            }
        }

        if !chunk.is_empty() {
            chunks.push(chunk);
        }

        chunks
    }
}
