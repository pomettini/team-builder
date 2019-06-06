use csv::ReaderBuilder;
use std::collections::HashMap;
use std::fs::*;
use std::path::Path;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

macro_rules! add_student {
    ($student:expr, $record:expr, $skill:expr, $id:expr) => {
        $student
            .skills
            .insert($skill, $record[$id].parse::<u8>().unwrap());
    };
}

pub static TEAM_NAMES: [&str; 10] = [
    "Alfa", "Bravo", "Charlie", "Delta", "Echo", "Foxtrot", "Golf", "Hotel", "India", "Juliett",
];

#[derive(Debug, Eq, PartialEq, Hash, Clone, EnumIter, Copy)]
pub enum Skills {
    GameDesign = 1,
    LevelDesign = 2,
    Programming = 3,
    Narrative = 4,
    Graphics = 5,
    Teamwork = 6,
}

#[derive(Default, Debug)]
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
    pub students_file: String,
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

            for skill in Skills::iter() {
                add_student!(student, record, skill, skill as usize);
            }

            students.push(student);
        }

        self.students = students;

        Ok(())
    }

    pub fn check_number_of_teams(&self, students_per_team: usize) -> Option<(usize, usize)> {
        if students_per_team >= self.students.len() {
            return None;
        }

        let quotient = self.students.len() / students_per_team;
        let remainder = self.students.len() % students_per_team;

        Some((quotient, remainder))
    }

    pub fn calculate_teams_skill_level(&mut self) {
        for student in &mut self.students {
            student.average_skill_level = student.get_average_skills();
        }
    }

    pub fn sort_teams_by_skill_level(&mut self, sort_by: Option<Skills>) {
        // Order from lowest to greatest
        match sort_by {
            None => {
                self.students.sort_by(|a, b| {
                    a.average_skill_level
                        .partial_cmp(&b.average_skill_level)
                        .unwrap()
                });
            }
            Some(skill) => {
                self.students
                    .sort_by(|a, b| a.skills[&skill].partial_cmp(&b.skills[&skill]).unwrap());
            }
        }
    }

    pub fn assign_students_to_team(&mut self, students_per_team: usize) {
        let number_of_teams = self.check_number_of_teams(students_per_team).unwrap();
        let mut teams: Vec<Team> = Vec::new();
        let mut students = self.students.clone();

        for _ in 0..number_of_teams.0 {
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