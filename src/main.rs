// #![warn(clippy::all, clippy::pedantic, clippy::nursery)]

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

use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    let tb = Rc::new(RefCell::new(TeamBuilder::new()));
    init_ui(&tb);
}
