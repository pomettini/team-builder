extern crate csv;
#[macro_use]
extern crate serde_derive;

use csv::{Reader, ReaderBuilder};
use std::collections::HashMap;

use std::fs::*;
use std::path::Path;

#[derive(Eq, PartialEq, Hash)]
pub enum Skills {
    GameDesign,
    LevelDesign,
    Programming,
    Narrative,
    Graphics,
    Teamwork,
}

pub struct Team {}

#[derive(Default)]
pub struct Student {
    pub surname: String,
    pub skills: HashMap<Skills, u8>,
}

#[derive(Default)]
pub struct TeamBuilder {
    pub teams: Vec<Team>,
    pub students: Vec<Student>,
    students_file: String,
}

impl TeamBuilder {
    pub fn load_file(path: &Path) -> Result<Self, ()> {
        let file_contents = read_to_string(&path);

        match file_contents {
            Ok(contents) => {
                let result = Self {
                    teams: Vec::new(),
                    students: Vec::new(),
                    students_file: contents,
                };
                Ok(result)
            }
            Err(_) => Err(()),
        }
    }

    pub fn process_file(&mut self) -> Result<(), ()> {
        let mut students: Vec<Student> = Vec::new();

        let mut reader = ReaderBuilder::new()
            .delimiter(b';')
            .from_reader(self.students_file.as_bytes());

        for record in reader.records() {
            let record = record.unwrap();
            let mut student: Student = Default::default();

            student.surname = record[0].to_string();

            // TODO: Must refactor
            student
                .skills
                .insert(Skills::GameDesign, record[1].parse::<u8>().unwrap());
            student
                .skills
                .insert(Skills::LevelDesign, record[2].parse::<u8>().unwrap());
            student
                .skills
                .insert(Skills::Programming, record[3].parse::<u8>().unwrap());
            student
                .skills
                .insert(Skills::Narrative, record[4].parse::<u8>().unwrap());
            student
                .skills
                .insert(Skills::Graphics, record[5].parse::<u8>().unwrap());
            student
                .skills
                .insert(Skills::Teamwork, record[6].parse::<u8>().unwrap());

            students.push(student);
        }

        self.students = students;

        Ok(())
    }
}

fn main() {
    let path = Path::new("resources/test.csv");
    let mut tb = TeamBuilder::load_file(&path).expect("File not found");
    tb.process_file().expect("Cannot process file");
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_FILE: &str = "resources/test.csv";
    static TEST_FILE_WRONG: &str = "resources/tests.csv";

    #[allow(unused_macros)]
    macro_rules! SETUP_TEAMBUILDER_TEST {
        ($file:expr, $path:ident, $tb:ident) => {
            let $path = Path::new($file);
            let mut $tb = TeamBuilder::load_file(&$path).expect("File not found");
        };
    }

    #[allow(unused_macros)]
    macro_rules! SETUP_TEAMBUILDER_TEST_AND_INIT {
        ($file:expr, $path:ident, $tb:ident) => {
            SETUP_TEAMBUILDER_TEST!($file, $path, $tb);
            $tb.process_file().expect("Cannot process file");
        };
    }

    #[test]
    fn test_load_csv_correct_path() {
        SETUP_TEAMBUILDER_TEST!(TEST_FILE, path, tb);
        assert!(tb.students_file.len() > 0);
    }

    #[test]
    #[should_panic]
    fn test_load_csv_wrong_path() {
        SETUP_TEAMBUILDER_TEST!(TEST_FILE_WRONG, path, tb);
        assert!(tb.students_file.len() > 0);
    }

    #[test]
    fn test_load_csv_valid_content() {
        SETUP_TEAMBUILDER_TEST_AND_INIT!(TEST_FILE, path, tb);
        assert!(tb.students.len() > 0);
    }

    #[test]
    #[should_panic]
    fn test_load_csv_not_valid_content() {
        SETUP_TEAMBUILDER_TEST_AND_INIT!(TEST_FILE_WRONG, path, tb);
        assert!(tb.students.len() > 0);
    }

    #[test]
    fn test_get_students_number_green() {
        unimplemented!();
    }

    #[test]
    fn test_get_students_number_red() {
        unimplemented!();
    }

    #[test]
    fn test_get_students_skills_average_green() {
        unimplemented!();
    }

    #[test]
    fn test_get_students_skills_average_red() {
        unimplemented!();
    }

    #[test]
    fn test_distribute_to_one_team() {
        unimplemented!();
    }

    #[test]
    fn test_distribute_to_two_teams() {
        unimplemented!();
    }

    #[test]
    fn test_distribute_to_three_teams() {
        unimplemented!();
    }

    #[test]
    fn test_distribute_to_four_teams() {
        unimplemented!();
    }
}