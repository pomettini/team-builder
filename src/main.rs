
extern crate csv;
extern crate serde_derive;
extern crate strum;
extern crate strum_macros;

extern crate itertools;
extern crate iui;

pub mod builder;
pub mod ui;

#[cfg(test)]
pub mod tests;

use builder::*;
use ui::*;

use std::path::Path;

fn main() {
    let path = Path::new("resources/students.csv");
    let mut tb = TeamBuilder::load_file(&path).expect("File not found");
    tb.process_file().expect("Cannot process file");
    tb.calculate_teams_skill_level();

    init_ui(&mut tb);
}