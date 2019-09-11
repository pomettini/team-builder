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
    pub people: Vec<Person>,
}

#[derive(Default, Debug, PartialEq, Clone)]
pub struct Person {
    pub surname: String,
    pub skill_levels: Vec<u32>,
    pub average_skill_level: f32,
}

impl Person {
    pub fn get_average_skills(&self) -> f32 {
        let mut sum: u32 = 0;

        for skill in &self.skill_levels {
            sum += skill;
        }

        sum as f32 / self.skill_levels.len() as f32
    }
}

#[derive(Default, Clone)]
pub struct TeamBuilder {
    pub teams: Vec<Team>,
    pub skills: Vec<String>,
    pub people: Vec<Person>,
    pub people_file: String,
}

impl TeamBuilder {
    pub fn new() -> Self {
        Self {
            teams: Vec::new(),
            skills: Vec::new(),
            people: Vec::new(),
            people_file: String::new(),
        }
    }

    pub fn load_file(&mut self, path: &Path) -> Result<(), ()> {
        let file_contents = read_to_string(&path);

        match file_contents {
            Ok(contents) => {
                self.people_file = contents;
                Ok(())
            }
            Err(_) => Err(()),
        }
    }

    pub fn process_file(&mut self) -> Result<(), io::Error> {
        // Reset values first
        self.teams = Vec::new();
        self.skills = Vec::new();

        let mut people: Vec<Person> = Vec::new();

        let mut reader = ReaderBuilder::new()
            .delimiter(b';')
            .from_reader(self.people_file.as_bytes());

        for skill in reader.headers()?.iter().skip(1) {
            self.skills.push(skill.to_string());
        }

        for record in reader.records() {
            let record = record.expect("Cannot process file");
            let mut person: Person = Person::default();

            person.surname = record[0].to_string();

            for index in 0..self.skills.len() {
                person.skill_levels.push(
                    record[index + 1]
                        .parse::<u32>()
                        .expect("Cannot push record"),
                );
            }

            people.push(person);
        }

        self.people = people;

        Ok(())
    }

    pub fn check_number_of_teams(&self, people_per_team: usize) -> Option<(usize, usize)> {
        if people_per_team >= self.people.len() {
            return None;
        }

        let quotient = self.people.len() / people_per_team;
        let remainder = self.people.len() % people_per_team;

        Some((quotient, remainder))
    }

    pub fn calculate_teams_skill_level(&mut self) {
        for person in &mut self.people {
            person.average_skill_level = person.get_average_skills();
        }
    }

    pub fn sort_teams_by_skill_level(&mut self, sort_by: Option<usize>) {
        // Order from lowest to greatest
        match sort_by {
            None => {
                self.people.sort_by(|a, b| {
                    a.average_skill_level
                        .partial_cmp(&b.average_skill_level)
                        .expect("Cannot compare people by average skill level")
                });
            }
            Some(skill) => {
                self.people.sort_by(|a, b| {
                    a.skill_levels[skill]
                        .partial_cmp(&b.skill_levels[skill])
                        .expect("Cannot compare people by average skill level")
                });
            }
        }
    }

    pub fn assign_people_to_team(&mut self, people_per_team: usize) {
        let number_of_teams = self
            .check_number_of_teams(people_per_team)
            .expect("Cannot calculate number of people per team");
        let mut teams: Vec<Team> = Vec::new();
        let mut people = self.people.clone();

        for _ in 0..number_of_teams.0 {
            teams.push(Team::default());
        }

        let mut direction = Direction::Forward;
        let mut team_index = 0;

        while !people.is_empty() {
            let person = match people.pop() {
                Some(person) => person,
                None => break,
            };

            teams[team_index].people.push(person);

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
