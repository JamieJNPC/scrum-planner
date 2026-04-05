use egui::{Response, Ui, Widget};
use crate::app::creation_windows::capacity_creation_window::CapacityCreationWindow;
use crate::app::date_picker::DatePicker;
use crate::app::entities::{Feature, Objective, Role};
use crate::app::model::pi::{Sprint, PI};

pub struct WindowData {
    pub role_window: RoleWindow,
    pub member_creation_window: MemberOptions,
    pub story_creation_window: StoryOptions,
    pub feature_creation_window: FeatureOptions,
    pub objective_creation_window: ObjectiveOptions,
    pub pi_creation_window: PiOptions,
    pub capacity_window: CapacityCreationWindow,
    pub screen: Screen,
    pub window: Window,
}

impl WindowData {

}

#[derive(Default)]
pub struct RoleWindow {
    pub role_title: String,
    pub velocity: String
}

impl RoleWindow {
    pub fn new(role_title: String, velocity: String) -> Self {
        RoleWindow{role_title, velocity}
    }
}

#[derive(serde::Deserialize, serde::Serialize, Default)]
pub enum Screen {
    #[default]
    MEMBERS,
    SPRINTS,
    FEATURES,
}

#[derive(serde::Deserialize, serde::Serialize, Default)]
pub enum Window {
    #[default]
    NONE,
    ROLE,
    MEMBER,
    STORY,
    FEATURE,
    SPRINT,
    PI,
    OBJECTIVE,
    CAPACITY
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct MemberOptions {
    pub roles: Vec<Role>,
    pub selected: Role,
    pub name: String,
    pub capacity: String
}

impl MemberOptions {
    pub fn new(roles: &Vec<Role>) -> Self {
        let role: Role = match roles.get(0) {
            Some(role) => {
                role.clone()
            }
            None => {
                Role::new(String::new(), 0.0)
            }
        };
        MemberOptions{roles: roles.clone(), selected: role, name: String::new(), capacity: String::new()}
    }
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct StoryOptions {
    pub title: String,
    pub description: String,
    pub story_points: String,
    pub pi: PI,
    pub sprint: Sprint,
    pub feature: Feature,
    pub objective: Objective,
}

impl StoryOptions {
    pub fn new() -> Self {
        StoryOptions {
            title: String::new(),
            description: String::new(),
            story_points: String::new(),
            pi: PI::new2(),
            sprint: Sprint::new2(),
            feature: Feature::new(String::from("None")),
            objective: Objective::new(String::from("None"))
        }
    }
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct PiOptions {
    pub title: String,
    pub start_date: DatePicker,
    pub end_date: DatePicker,
    pub weeks_in_sprint: String,
    pub number_of_sprints: String,
}

impl PiOptions {
    pub fn new(title: String, start_date: DatePicker, end_date: DatePicker, weeks_in_sprint: String, number_of_sprints: String) -> Self {
        PiOptions {title, start_date, end_date, weeks_in_sprint, number_of_sprints}
    }

    pub fn empty() -> Self {
        PiOptions {title: String::new(), start_date: DatePicker::new(), end_date: DatePicker::new(),
            weeks_in_sprint: String::new(), number_of_sprints: String::new()}
    }
}

impl Widget for &mut PiOptions {
    fn ui(mut self, ui: &mut Ui) -> Response {
        ui.vertical_centered(|ui| {
            ui.horizontal(|ui| {
                ui.label("Name");
                ui.text_edit_singleline(&mut self.title);
            });
            ui.horizontal(|ui| {
                ui.label("Start Date");
                ui.add(&mut self.start_date);
            });
            ui.horizontal(|ui| {
                ui.label("End Date");
                ui.add(&mut self.end_date);
            });
            ui.horizontal(|ui| {
                ui.label("Number of Sprints");
                ui.text_edit_singleline(&mut self.number_of_sprints);
            });
            ui.horizontal(|ui| {
                ui.label("Weeks in Sprint");
                ui.text_edit_singleline(&mut self.weeks_in_sprint);
            });
        }).response
    }
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct FeatureOptions {
    pub title: String,
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct ObjectiveOptions {
    pub title: String,
    pub feature: Feature,
}

impl ObjectiveOptions {
    pub fn new() -> Self {
        ObjectiveOptions {title: String::new(), feature: Feature::new(String::new())}
    }
}