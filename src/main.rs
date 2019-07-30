#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

extern crate csv;
extern crate itertools;
extern crate iui;
extern crate serde_derive;
extern crate simple_excel_writer as excel;
extern crate strum;
extern crate strum_macros;

pub mod builder;
pub mod html_exporter;
pub mod spreadsheet_exporter;
pub mod ui;

#[cfg(test)]
pub mod tests;

use builder::*;
use std::path::Path;
use ui::*;

fn main() {
    let path = Path::new("resources/students.csv");
    let mut tb = TeamBuilder::load_file(path).expect("File not found");
    tb.process_file().expect("Cannot process file");
    tb.calculate_teams_skill_level();

    init_ui(&mut tb);
}
