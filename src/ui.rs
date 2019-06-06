use itertools::Itertools;
use iui::controls::*;
use iui::prelude::*;
use strum::IntoEnumIterator;

use crate::builder::*;

use std::cell::RefCell;
use std::rc::Rc;

pub fn init_ui(tb: &mut TeamBuilder) {
    let sort_by: Rc<RefCell<Option<Skills>>> = Rc::new(RefCell::new(None));

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
            let mut group = Group::new(&ui, &format!("Team {}", TEAM_NAMES[counter]));
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
        let sort_by = sort_by.clone();
        move |_| {
            tb.sort_teams_by_skill_level(*sort_by.borrow());
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

    let combo_box = Combobox::new(&ui);
    combo_box.append(&ui, "Sort by Average");

    for skill in Skills::iter() {
        combo_box
            .clone()
            .append(&ui, &format!("Sort by {:?}", skill));
    }

    combo_box.clone().set_selected(&ui, 0);
    combo_box.clone().on_selected(&ui, {
        move |index| {
            println!("{:?}", index);
            match index {
                // TODO: Must refactor
                0 => *sort_by.borrow_mut() = None,
                1 => *sort_by.borrow_mut() = Some(Skills::GameDesign),
                2 => *sort_by.borrow_mut() = Some(Skills::LevelDesign),
                3 => *sort_by.borrow_mut() = Some(Skills::Programming),
                4 => *sort_by.borrow_mut() = Some(Skills::Narrative),
                5 => *sort_by.borrow_mut() = Some(Skills::Graphics),
                6 => *sort_by.borrow_mut() = Some(Skills::Teamwork),
                _ => *sort_by.borrow_mut() = None,
            }
        }
    });

    vbox.append(&ui, combo_box, LayoutStrategy::Compact);
    vbox.append(&ui, students_group_vbox, LayoutStrategy::Compact);

    win.set_child(&ui, vbox);
    win.show(&ui);
    ui.main();
}