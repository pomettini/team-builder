#[rustfmt::skip]
use itertools::Itertools;
#[rustfmt::skip]
use iui::controls::*;
#[rustfmt::skip]
use iui::prelude::*;
#[rustfmt::skip]
use crate::builder::*;
#[rustfmt::skip]
use std::cell::RefCell;
#[rustfmt::skip]
use std::rc::Rc;
#[rustfmt::skip]
use crate::html_exporter::*;
#[rustfmt::skip]
use crate::csv_exporter::*;

pub fn init_ui(tb: &mut TeamBuilder) {
    // Wrapped with Interior Mutability Pattern
    // Because I need to pass the state around between UI controls
    let sort_by: Rc<RefCell<Option<usize>>> = Rc::new(RefCell::new(None));
    let skills = tb.skills.clone();

    let ui = UI::init().expect("Couldn't initialize UI library");
    let mut window = Window::new(&ui, "Team Builder", 640, 400, WindowType::NoMenubar);

    let mut program_vbox = VerticalBox::new(&ui);
    program_vbox.set_padded(&ui, true);

    let mut selectors_hbox = HorizontalBox::new(&ui);
    selectors_hbox.set_padded(&ui, true);

    let team_number_label = Label::new(&ui, "Team members: 2");
    let mut team_number_slider = Slider::new(&ui, 2, 10);

    selectors_hbox.append(&ui, team_number_label.clone(), LayoutStrategy::Compact);
    selectors_hbox.append(&ui, team_number_slider.clone(), LayoutStrategy::Stretchy);

    let mut students_labels: Vec<Label> = Vec::new();

    let mut students_group_vbox = VerticalBox::new(&ui);
    students_group_vbox.set_padded(&ui, true);

    // TODO: Must refactor
    // Creates two columns and five rows for the teams
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

    // Updates the number of teams based on slider's value
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
            // Do stuff with teams data
            tb.sort_teams_by_skill_level(*sort_by.borrow());
            tb.assign_students_to_team(team_number_slider.value(&ui) as usize);

            // Cleans the value of every label
            for label in students_labels.iter_mut() {
                label.set_text(&ui, "");
            }

            // Assigns the teams on each label
            let mut counter = 0;
            for team in tb.teams.iter().map(|team| &team.students) {
                let surnames: Vec<String> = team
                    .iter()
                    .map(|student| {
                        format!(
                            "{} [{:.1}]",
                            student.surname.clone(),
                            student.average_skill_level
                        )
                    })
                    .collect();

                let surname_list = surnames.iter().join(", ");
                students_labels[counter].set_text(&ui, &surname_list);

                counter += 1;
            }
        }
    });

    selectors_hbox.append(&ui, generate_button, LayoutStrategy::Stretchy);
    program_vbox.append(&ui, selectors_hbox, LayoutStrategy::Compact);

    let sort_by_skill_cb = Combobox::new(&ui);
    sort_by_skill_cb.append(&ui, "Sort by Average");

    // Add each skill to the ComboBox
    for skill in skills {
        print!("{}", skill);
        sort_by_skill_cb
            .clone()
            .append(&ui, &format!("Sort by {}", skill));
    }

    // Updates the value of the sorting variable
    sort_by_skill_cb.clone().set_selected(&ui, 0);
    sort_by_skill_cb.clone().on_selected(&ui, {
        move |index| {
            // TODO: Need refactor
            if index == 0 {
                *sort_by.borrow_mut() = None;
            } else {
                *sort_by.borrow_mut() = Some((index - 1) as usize);
            }
        }
    });

    program_vbox.append(&ui, sort_by_skill_cb, LayoutStrategy::Compact);

    program_vbox.append(&ui, HorizontalSeparator::new(&ui), LayoutStrategy::Compact);

    program_vbox.append(&ui, students_group_vbox, LayoutStrategy::Compact);

    program_vbox.append(&ui, HorizontalSeparator::new(&ui), LayoutStrategy::Compact);

    let mut exporters_hbox = HorizontalBox::new(&ui);
    exporters_hbox.set_padded(&ui, true);

    let mut generate_html_table_button = Button::new(&ui, "Export HTML Table");
    generate_html_table_button.on_clicked(&ui, {
        let ui = ui.clone();
        let window = window.clone();
        move |_| {
            window.modal_err(&ui, "Warning", "Not yet implemented, come back later!");
        }
    });
    exporters_hbox.append(&ui, generate_html_table_button, LayoutStrategy::Stretchy);

    let mut generate_csv_table_button = Button::new(&ui, "Export CSV Table");
    generate_csv_table_button.on_clicked(&ui, {
        let ui = ui.clone();
        let window = window.clone();
        move |_| {
            window.modal_err(&ui, "Warning", "Not yet implemented, come back later!");
        }
    });
    exporters_hbox.append(&ui, generate_csv_table_button, LayoutStrategy::Stretchy);

    program_vbox.append(&ui, exporters_hbox, LayoutStrategy::Compact);

    window.set_child(&ui, program_vbox);
    window.show(&ui);
    ui.main();
}
