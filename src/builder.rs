use csv::ReaderBuilder;
use std::fs::*;
use std::io;
use std::path::Path;

enum Direction {
    Forward,
    Backward,
}

pub static TEAM_NAMES: [&str; 10] = [
    "Alfa", "Bravo", "Charlie", "Delta", "Echo", "Foxtrot", "Golf", "Hotel", "India", "Juliett",
];

#[derive(Default, Debug, Clone)]
pub struct Team {
    pub students: Vec<Student>,
}

#[derive(Default, Debug, PartialEq, Clone)]
pub struct Student {
    pub surname: String,
    pub skill_levels: Vec<u8>,
    pub average_skill_level: f32,
}

impl Student {
    pub fn get_average_skills(&self) -> f32 {
        let mut sum: u8 = 0;

        for skill in &self.skill_levels {
            sum += skill;
        }

        f32::from(sum) / self.skill_levels.len() as f32
    }
}

#[derive(Default, Clone)]
pub struct TeamBuilder {
    pub teams: Vec<Team>,
    pub skills: Vec<String>,
    pub students: Vec<Student>,
    pub students_file: String,
}

impl TeamBuilder {
    pub fn new() -> Self {
        Self {
            teams: Vec::new(),
            skills: Vec::new(),
            students: Vec::new(),
            students_file: String::new(),
        }
    }

    pub fn load_file(&mut self, path: &Path) -> Result<(), ()> {
        let file_contents = read_to_string(&path);

        match file_contents {
            Ok(contents) => {
                self.students_file = contents;
                Ok(())
            }
            Err(_) => Err(()),
        }
    }

    pub fn process_file(&mut self) -> Result<(), io::Error> {
        let mut students: Vec<Student> = Vec::new();

        let mut reader = ReaderBuilder::new()
            .delimiter(b';')
            .from_reader(self.students_file.as_bytes());

        for skill in reader.headers()?.iter().skip(1) {
            self.skills.push(skill.to_string());
        }

        for record in reader.records() {
            let record = record.expect("Cannot process file");
            let mut student: Student = Student::default();

            student.surname = record[0].to_string();

            for index in 0..self.skills.len() {
                student
                    .skill_levels
                    .push(record[index + 1].parse::<u8>().expect("Cannot push record"));
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

    pub fn sort_teams_by_skill_level(&mut self, sort_by: Option<usize>) {
        // Order from lowest to greatest
        match sort_by {
            None => {
                self.students.sort_by(|a, b| {
                    a.average_skill_level
                        .partial_cmp(&b.average_skill_level)
                        .expect("Cannot compare students by average skill level")
                });
            }
            Some(skill) => {
                self.students.sort_by(|a, b| {
                    a.skill_levels[skill]
                        .partial_cmp(&b.skill_levels[skill])
                        .expect("Cannot compare students by average skill level")
                });
            }
        }
    }

    pub fn assign_students_to_team(&mut self, students_per_team: usize) {
        let number_of_teams = self
            .check_number_of_teams(students_per_team)
            .expect("Cannot calculate number of students per team");
        let mut teams: Vec<Team> = Vec::new();
        let mut students = self.students.clone();

        for _ in 0..number_of_teams.0 {
            teams.push(Team::default());
        }

        let mut direction = Direction::Forward;
        let mut team_index = 0;

        while !students.is_empty() {
            let student = match students.pop() {
                Some(student) => student,
                None => break,
            };

            teams[team_index].students.push(student);

            // FIXME: Needs refactor
            match direction {
                Direction::Forward => {
                    if team_index < teams.len() - 1 {
                        team_index += 1;
                    } else {
                        direction = Direction::Backward;
                    }
                }
                Direction::Backward => {
                    if team_index > 0 {
                        team_index -= 1;
                    } else {
                        direction = Direction::Forward;
                    }
                }
            }
        }

        self.teams = teams;
    }
}
