extern crate csv;
#[macro_use]
extern crate serde_derive;

use csv::{Reader, ReaderBuilder};
use std::cmp::Eq;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::*;
use std::path::Path;

macro_rules! ADD_STUDENT {
    ($student:expr, $record:expr, $skill:expr, $id:expr) => {
        $student
            .skills
            .insert($skill, $record[$id].parse::<u8>().unwrap());
    };
}

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub enum Skills {
    GameDesign,
    LevelDesign,
    Programming,
    Narrative,
    Graphics,
    Teamwork,
}

#[derive(Default)]
pub struct Team {
    pub students: Vec<Student>,
}

#[derive(Default, Debug, PartialEq, Clone)]
pub struct Student {
    pub surname: String,
    pub skills: HashMap<Skills, u8>,
    pub average_skill_level: f32,
}

impl Student {
    pub fn get_average_skills(&self) -> f32 {
        let mut sum: u8 = 0;

        for skill in &self.skills {
            sum += skill.1;
        }

        f32::from(sum) / self.skills.len() as f32
    }
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
            ADD_STUDENT!(student, record, Skills::GameDesign, 1);
            ADD_STUDENT!(student, record, Skills::LevelDesign, 2);
            ADD_STUDENT!(student, record, Skills::Programming, 3);
            ADD_STUDENT!(student, record, Skills::Narrative, 4);
            ADD_STUDENT!(student, record, Skills::Graphics, 5);
            ADD_STUDENT!(student, record, Skills::Teamwork, 6);

            students.push(student);
        }

        self.students = students;

        Ok(())
    }

    fn check_number_of_teams(&self, students_per_team: usize) -> Option<(usize, usize)> {
        if students_per_team >= self.students.len() {
            return None;
        }

        let quotient = self.students.len() / students_per_team;
        let remainder = self.students.len() % students_per_team;

        Some((quotient, remainder))
    }

    fn calculate_teams_skill_level(&mut self) {
        for student in &mut self.students {
            student.average_skill_level = student.get_average_skills();
        }
    }

    fn sort_teams_by_skill_level(&mut self) {
        self.students.sort_by(|a, b| {
            b.average_skill_level
                .partial_cmp(&a.average_skill_level)
                .unwrap()
        });
    }

    fn assign_students_to_team(&mut self, students_per_team: usize) {
        let number_of_teams = self.check_number_of_teams(students_per_team).unwrap();
        let mut teams: Vec<Team> = Vec::new();
        let mut students = self.students.clone();

        for i in 0..number_of_teams.0 {
            teams.push(Default::default());
        }

        while !students.is_empty() {
            for team in &mut teams {
                match students.pop() {
                    Some(s) => team.students.push(s),
                    None => break,
                }
            }
        }

        self.teams = teams;
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

    static TEST_FILE_EVEN: &str = "resources/test_even.csv";
    static TEST_FILE_UNEVEN: &str = "resources/test_uneven.csv";
    static TEST_FILE_WRONG: &str = "resources/test.csv";

    #[allow(unused_macros)]
    macro_rules! SETUP_TEAMBUILDER_TEST {
        ($file:expr, $path:ident, $tb:ident) => {
            let $path = Path::new($file);
            #[allow(unused_mut)]
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
        SETUP_TEAMBUILDER_TEST!(TEST_FILE_EVEN, path, tb);
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
        SETUP_TEAMBUILDER_TEST_AND_INIT!(TEST_FILE_EVEN, path, tb);
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
        SETUP_TEAMBUILDER_TEST_AND_INIT!(TEST_FILE_EVEN, path, tb);
        assert_eq!(tb.students.len(), 6);
    }

    #[test]
    #[should_panic]
    fn test_get_students_number_red() {
        SETUP_TEAMBUILDER_TEST_AND_INIT!(TEST_FILE_EVEN, path, tb);
        assert_eq!(tb.students.len(), 0);
    }

    #[test]
    fn test_get_students_skills_average_green() {
        SETUP_TEAMBUILDER_TEST_AND_INIT!(TEST_FILE_EVEN, path, tb);
        assert_eq!(tb.students[0].get_average_skills(), 2.0);
        assert_eq!(tb.students[1].get_average_skills(), 2.1666667);
    }

    #[test]
    #[should_panic]
    fn test_get_students_skills_average_red() {
        SETUP_TEAMBUILDER_TEST_AND_INIT!(TEST_FILE_EVEN, path, tb);
        assert_eq!(tb.students[0].get_average_skills(), 3.0);
        assert_eq!(tb.students[1].get_average_skills(), 3.0);
    }

    #[test]
    fn test_check_number_of_teams_divisible_first() {
        SETUP_TEAMBUILDER_TEST_AND_INIT!(TEST_FILE_EVEN, path, tb);
        assert_eq!(tb.check_number_of_teams(2), Some((3, 0)));
    }

    #[test]
    fn test_check_number_of_teams_divisible_second() {
        SETUP_TEAMBUILDER_TEST_AND_INIT!(TEST_FILE_EVEN, path, tb);
        assert_eq!(tb.check_number_of_teams(3), Some((2, 0)));
    }

    #[test]
    fn test_check_number_of_teams_not_divisible() {
        SETUP_TEAMBUILDER_TEST_AND_INIT!(TEST_FILE_EVEN, path, tb);
        assert_eq!(tb.check_number_of_teams(4), Some((1, 2)));
    }

    #[test]
    fn test_check_number_of_teams_same_size() {
        SETUP_TEAMBUILDER_TEST_AND_INIT!(TEST_FILE_EVEN, path, tb);
        assert_eq!(tb.check_number_of_teams(6), None);
    }

    #[test]
    fn test_check_number_of_teams_exceed() {
        SETUP_TEAMBUILDER_TEST_AND_INIT!(TEST_FILE_EVEN, path, tb);
        assert_eq!(tb.check_number_of_teams(7), None);
    }

    #[test]
    fn test_calculate_skill_level_green() {
        SETUP_TEAMBUILDER_TEST_AND_INIT!(TEST_FILE_EVEN, path, tb);

        tb.calculate_teams_skill_level();

        let student = tb
            .students
            .iter()
            .find(|&x| x.surname == "Pomettini")
            .unwrap();

        assert_eq!(student.average_skill_level, 1.8333334);
    }

    #[test]
    #[should_panic]
    fn test_calculate_skill_level_red() {
        SETUP_TEAMBUILDER_TEST_AND_INIT!(TEST_FILE_EVEN, path, tb);

        tb.calculate_teams_skill_level();

        let student = tb
            .students
            .iter()
            .find(|&x| x.surname == "Pomettini")
            .unwrap();

        assert_eq!(student.average_skill_level, 2.0);
    }

    #[test]
    fn test_sort_by_skill_level_best_green() {
        SETUP_TEAMBUILDER_TEST_AND_INIT!(TEST_FILE_EVEN, path, tb);

        tb.calculate_teams_skill_level();
        tb.sort_teams_by_skill_level();

        assert_eq!(tb.students.first().unwrap().surname, "Bonanni");
    }

    #[test]
    #[should_panic]
    fn test_sort_by_skill_level_best_red() {
        SETUP_TEAMBUILDER_TEST_AND_INIT!(TEST_FILE_EVEN, path, tb);

        tb.calculate_teams_skill_level();
        tb.sort_teams_by_skill_level();

        assert_eq!(tb.students.first().unwrap().surname, "Pomettini");
    }

    #[test]
    fn test_sort_by_skill_level_worst_green() {
        SETUP_TEAMBUILDER_TEST_AND_INIT!(TEST_FILE_EVEN, path, tb);

        tb.calculate_teams_skill_level();
        tb.sort_teams_by_skill_level();

        assert_eq!(tb.students.last().unwrap().surname, "Reclus");
    }

    #[test]
    #[should_panic]
    fn test_sort_by_skill_level_worst_red() {
        SETUP_TEAMBUILDER_TEST_AND_INIT!(TEST_FILE_EVEN, path, tb);

        tb.calculate_teams_skill_level();
        tb.sort_teams_by_skill_level();

        assert_eq!(tb.students.last().unwrap().surname, "Pomettini");
    }

    #[test]
    fn test_assign_students_to_team_even() {
        SETUP_TEAMBUILDER_TEST_AND_INIT!(TEST_FILE_EVEN, path, tb);

        tb.calculate_teams_skill_level();
        tb.sort_teams_by_skill_level();
        tb.assign_students_to_team(3);

        let first_team: Vec<String> = tb.teams[0]
            .students
            .iter()
            .map(|x| x.surname.clone())
            .collect();

        let second_team: Vec<String> = tb.teams[1]
            .students
            .iter()
            .map(|x| x.surname.clone())
            .collect();

        assert_eq!(tb.teams.len(), 2);
        assert_eq!(first_team, vec!["Reclus", "Pomettini", "Ricchiuti"]);
        assert_eq!(second_team, vec!["Leotta", "De Dominicis", "Bonanni"]);
    }

    #[test]
    fn test_assign_students_to_team_uneven() {
        SETUP_TEAMBUILDER_TEST_AND_INIT!(TEST_FILE_UNEVEN, path, tb);

        tb.calculate_teams_skill_level();
        tb.sort_teams_by_skill_level();
        tb.assign_students_to_team(2);

        let first_team: Vec<String> = tb.teams[0]
            .students
            .iter()
            .map(|x| x.surname.clone())
            .collect();

        let second_team: Vec<String> = tb.teams[1]
            .students
            .iter()
            .map(|x| x.surname.clone())
            .collect();

        assert_eq!(tb.teams.len(), 2);
        assert_eq!(first_team, vec!["Leotta", "De Dominicis", "Bonanni"]);
        assert_eq!(second_team, vec!["Pomettini", "Ricchiuti"]);
    }
}