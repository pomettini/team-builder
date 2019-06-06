
extern crate csv;
#[macro_use]
extern crate serde_derive;
extern crate strum;
#[macro_use]
extern crate strum_macros;

extern crate itertools;
extern crate iui;
pub mod tests;

// For some reasons rustfmt fucks up with use declarations

#[rustfmt::skip]
use csv::{ReaderBuilder};
#[rustfmt::skip]
use std::fs::*;
#[rustfmt::skip]
use std::collections::{HashMap};
#[rustfmt::skip]
use std::path::Path;
#[rustfmt::skip]
use strum::IntoEnumIterator;
#[rustfmt::skip]
use strum_macros::{EnumIter};
#[rustfmt::skip]
use iui::prelude::*;
#[rustfmt::skip]
use iui::controls::*;
#[rustfmt::skip]
use itertools::Itertools;

macro_rules! add_student {
    ($student:expr, $record:expr, $skill:expr, $id:expr) => {
        $student
            .skills
            .insert($skill, $record[$id].parse::<u8>().unwrap());
    };
}

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

            for skill in Skills::iter() {
                add_student!(student, record, skill, skill as usize);
            }

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
        // Order from lowest to greatest
        self.students.sort_by(|a, b| {
            a.average_skill_level
                .partial_cmp(&b.average_skill_level)
                .unwrap()
        });
    }

    fn assign_students_to_team(&mut self, students_per_team: usize) {
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

fn main() {
    let path = Path::new("resources/students.csv");
    let mut tb = TeamBuilder::load_file(&path).expect("File not found");
    tb.process_file().expect("Cannot process file");
    tb.calculate_teams_skill_level();
    tb.sort_teams_by_skill_level();

    let team_names = vec![
        "Alfa", "Bravo", "Charlie", "Delta", "Echo", "Foxtrot", "Golf", "Hotel", "India", "Juliett",
    ];

    let ui = UI::init().expect("Couldn't initialize UI library");
    let mut win = Window::new(&ui, "Team Builder", 640, 400, WindowType::NoMenubar);

    let mut vbox = VerticalBox::new(&ui);
    vbox.set_padded(&ui, true);

    let mut hbox = HorizontalBox::new(&ui);
    hbox.set_padded(&ui, true);

    let team_number_label = Label::new(&ui, "Team members: 2");
    let mut team_number_slider = Slider::new(&ui, 2, 10);

    hbox.append(&ui, team_number_label.clone(), LayoutStrategy::Compact);
    hbox.append(&ui, team_number_slider.clone(), LayoutStrategy::Stretchy);

    let mut students_labels: Vec<Label> = Vec::new();

    let mut students_group_vbox = VerticalBox::new(&ui);
    students_group_vbox.set_padded(&ui, true);

    // TODO: Must refactor
    let mut counter = 0;
    for _ in 0..5 {
        let mut students_group_hbox = HorizontalBox::new(&ui);
        students_group_hbox.set_padded(&ui, true);
        for _ in 0..2 {
            let mut group = Group::new(&ui, &format!("Team {}", team_names[counter]));
            let label = Label::new(&ui, "");
            students_labels.push(label.clone());
            group.set_child(&ui, label);
            students_group_hbox.append(&ui, group, LayoutStrategy::Stretchy);
            counter += 1;
        }
        students_group_vbox.append(&ui, students_group_hbox, LayoutStrategy::Stretchy);
    }

    team_number_slider.on_changed(&ui, {
        let ui = ui.clone();
        let mut team_number_label = team_number_label.clone();
        move |val| {
            team_number_label.set_text(&ui, &format!("Team members: {}", val));
        }
    });

    let mut generate_button = Button::new(&ui, "Generate Teams");

    generate_button.on_clicked(&ui, {
        let ui = ui.clone();
        let team_number_slider = team_number_slider.clone();
        move |_| {
            tb.assign_students_to_team(team_number_slider.value(&ui) as usize);

            // Cleans the value of every label
            for label in students_labels.iter_mut() {
                label.set_text(&ui, "");
            }

            let mut counter = 0;
            for team in tb.teams.iter().map(|team| &team.students) {
                let surnames: Vec<String> =
                    team.iter().map(|student| student.surname.clone()).collect();

                let surname_list = surnames.iter().join(", ");
                students_labels[counter].set_text(&ui, &surname_list);

                counter += 1;
            }
        }
    });

    hbox.append(&ui, generate_button, LayoutStrategy::Stretchy);
    vbox.append(&ui, hbox, LayoutStrategy::Compact);
    vbox.append(&ui, students_group_vbox, LayoutStrategy::Compact);

    win.set_child(&ui, vbox);
    win.show(&ui);
    ui.main();
}