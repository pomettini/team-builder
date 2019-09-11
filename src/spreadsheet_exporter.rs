use crate::builder::*;

use excel::*;

pub fn generate_spreadsheet(teams: &[Team]) {
    // TODO: Remove hardcoded values
    let mut workbook = excel::Workbook::create("test.xlsx");
    let mut sheet = workbook.create_sheet("test_sheet");

    // TODO: Add column padding
    workbook
        .write_sheet(&mut sheet, |data| {
            for team in teams {
                let mut row = Row::new();

                // TODO: Remove hardcoded team name
                row.add_cell("Team name");

                for person in &team.people {
                    row.add_cell(person.surname.as_str());
                }

                data.append_row(row).unwrap();
            }
            Ok(())
        })
        .expect("Write Excel error!");

    workbook.close().expect("Close Excel error!");
}
