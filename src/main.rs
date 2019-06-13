extern crate csv;
#[rustfmt::skip]
extern crate serde_derive;
#[rustfmt::skip]
extern crate strum;
#[rustfmt::skip]
extern crate strum_macros;
#[rustfmt::skip]
extern crate itertools;
#[rustfmt::skip]
extern crate iui;

#[rustfmt::skip]
pub mod builder;
#[rustfmt::skip]
pub mod html_exporter;
#[rustfmt::skip]
pub mod csv_exporter;
#[rustfmt::skip]
pub mod ui;

#[cfg(test)]
pub mod tests;

#[rustfmt::skip]
use builder::*;
#[rustfmt::skip]
use html_exporter::*;
#[rustfmt::skip]
use csv_exporter::*;
#[rustfmt::skip]
use ui::*;
#[rustfmt::skip]
use std::path::Path;

// TODO: Add exporter for teams
// TODO: Make some kind of graph

fn main() {
    let path = Path::new("resources/students.csv");
    let mut tb = TeamBuilder::load_file(&path).expect("File not found");
    tb.process_file().expect("Cannot process file");
    tb.calculate_teams_skill_level();

    init_ui(&mut tb);
}
