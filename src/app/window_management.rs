use egui::{Context, Response, Ui};
use crate::app::entities::{Feature, Member, Objective, Role, Story};
use crate::MainApp;

#[derive(Default)]
pub struct RoleWindow {
    pub show: bool,
    pub role_title: String,
    pub velocity: String
}

impl RoleWindow {
    pub fn new(role_title: String, velocity: String) -> Self {
        RoleWindow{show: false, role_title, velocity}
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
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct MainAppData {
    pub label: String,
    pub members: Vec<Member>,
    pub roles: Vec<Role>,
    #[serde(skip)]
    pub role_window: RoleWindow,
    pub member_creation_window: MemberOptions,
    pub story_creation_window: StoryOptions,
    pub feature_creation_window: FeatureOptions,
    pub objective_creation_window: ObjectiveOptions,
    #[serde(skip)]
    pub screen: Screen,
    pub window: Window,
    pub features: Vec<Feature>,
}

impl MainAppData {
    pub fn get_feature(&self, name: &String) -> Option<&Feature> {
        for f in self.features.iter() {
            if f.name.eq(name.as_str()) {
                return Some(f);
            }
        }
        return None;
    }

    pub fn get_feature_mut(&mut self, name: &String) -> Option<&mut Feature> {
        for f in self.features.iter_mut() {
            if f.name.eq(name.as_str()) {
                return Some(f);
            }
        }
        return None;
    }

    pub fn get_objective_mut(&mut self, feature: &String, objective: &String) -> Option<&mut Objective> {
        for f in self.features.iter_mut() {
            if f.name.eq(feature.as_str()) {
                for ob in f.objectives.iter_mut() {
                    if ob.title.eq(objective.as_str()) {
                        return Some(ob);
                    }
                }
                return None
            }
        }
        return None;
    }
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
    pub feature: Feature,
    pub objective: Objective,
}

impl StoryOptions {
    pub fn new() -> Self {
        StoryOptions {title: String::new(), description: String::new(), story_points: String::new(), feature: Feature {name: "None".to_string(), objectives: vec![]}, objective: Objective::new("None".to_string())}
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

impl MainApp {

    pub fn render_role_window(&mut self, ctx: &egui::Context) {
        egui::Window::new("Add Member")
            .fixed_pos(&[50., 50.])
            .resizable(false)
            .title_bar(false)
            .open(&mut true)
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.horizontal(|ui| {
                        ui.label("Name");
                        ui.text_edit_singleline(&mut self.main_app_data.role_window.role_title);
                    });
                    ui.horizontal(|ui| {
                        ui.label("Velocity");
                        ui.text_edit_singleline(&mut self.main_app_data.role_window.velocity);
                    });
                    ui.horizontal(|ui| {
                        if ui.button("Create Role").clicked() {
                            let vel = self.main_app_data.role_window.velocity.parse::<f64>().unwrap();
                            self.main_app_data.roles.push(Role::new(self.main_app_data.role_window.role_title.clone(), vel.clone()));
                            self.main_app_data.window = Window::NONE;
                        }
                        if ui.button("Cancel").clicked() {
                            self.main_app_data.window = Window::NONE;
                        }
                    });
                });
            });
    }

    pub fn render_story_window(&mut self, ctx: &egui::Context) {
        egui::Window::new("Add Story")
            .default_pos(&[50., 50.])
            .resizable(true)
            .title_bar(false)
            .open(&mut true)
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.horizontal(|ui| {
                        ui.label("Name");
                        ui.text_edit_singleline(&mut self.main_app_data.story_creation_window.title);
                    });
                    ui.horizontal(|ui| {
                        ui.label("Description");
                        ui.text_edit_multiline(&mut self.main_app_data.story_creation_window.description);
                    });
                    ui.horizontal(|ui| {
                        ui.label("Story Points");
                        ui.text_edit_singleline(&mut self.main_app_data.story_creation_window.story_points);
                    });
                    ui.horizontal(|ui| {
                        egui::ComboBox::from_label("Feature")
                            .selected_text(format!("{:?}", self.main_app_data.story_creation_window.feature))
                            .show_ui(ui, |ui| {
                                for feature in self.main_app_data.features.clone() {
                                    ui.selectable_value(&mut self.main_app_data.story_creation_window.feature, feature.clone(), feature.name.clone());
                                }
                            });
                    });
                    ui.horizontal(|ui| {
                        egui::ComboBox::from_label("Objective")
                            .selected_text(format!("{:?}", self.main_app_data.story_creation_window.objective))
                            .show_ui(ui, |ui| {
                                for objective in self.main_app_data.get_feature(&self.main_app_data.story_creation_window.feature.name).unwrap().objectives.clone() {
                                    ui.selectable_value(&mut self.main_app_data.story_creation_window.objective, objective.clone(), objective.title.clone());
                                }
                            });
                    });
                    ui.horizontal(|ui| {
                        if ui.button("Create").clicked() {
                            let story_points = self.main_app_data.story_creation_window.story_points.parse::<f64>().unwrap();
                            let story = Story::new(self.main_app_data.story_creation_window.title.clone(), story_points,
                                                     self.main_app_data.story_creation_window.description.clone());
                            let feature = self.main_app_data.story_creation_window.feature.name.clone();
                            let ob = self.main_app_data.story_creation_window.objective.title.clone();
                            let objective = self.main_app_data.get_objective_mut(&feature, &ob).unwrap();
                            objective.add_story(story);
                            self.main_app_data.story_creation_window = StoryOptions::new()
                            }});
                        if ui.button("Cancel").clicked() {
                            self.main_app_data.window = Window::NONE;
                        }
                    });
                });
            }

    pub fn render_feature_window(&mut self, ctx: &egui::Context) {
        egui::Window::new("Add Feature")
            .default_pos(&[50., 50.])
            .resizable(true)
            .title_bar(false)
            .open(&mut true)
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.horizontal(|ui| {
                        ui.label("Name");
                        ui.text_edit_singleline(&mut self.main_app_data.feature_creation_window.title);
                    });
                    ui.horizontal(|ui| {
                        if ui.button("Create").clicked() {
                            let feature = Feature::new(self.main_app_data.feature_creation_window.title.clone());
                            self.main_app_data.features.push(feature);
                            self.main_app_data.window = Window::NONE;
                        }
                        if ui.button("Cancel").clicked() {
                            self.main_app_data.window = Window::NONE;
                        }
                    });
                });
            });
    }

    pub fn render_objective_window(&mut self, ctx: &egui::Context) {
        egui::Window::new("Add Objective")
            .default_pos(&[50., 50.])
            .resizable(true)
            .title_bar(false)
            .open(&mut true)
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.horizontal(|ui| {
                        ui.label("Name");
                        ui.text_edit_singleline(&mut self.main_app_data.objective_creation_window.title);
                    });
                    egui::ComboBox::from_label("Feature")
                        .selected_text(format!("{:?}", self.main_app_data.objective_creation_window.feature))
                        .show_ui(ui, |ui| {
                            for feature in self.main_app_data.features.clone() {
                                ui.selectable_value(&mut self.main_app_data.objective_creation_window.feature, feature.clone(), feature.name.clone());
                            }
                        });
                    ui.horizontal(|ui| {
                        if ui.button("Create").clicked() {
                            let objective = Objective::new(self.main_app_data.objective_creation_window.title.clone());
                            let feature_name = self.main_app_data.objective_creation_window.feature.name.clone();
                            self.main_app_data.get_feature_mut(&feature_name).unwrap().add_objective(objective);
                            self.main_app_data.window = Window::NONE;
                        }
                        if ui.button("Cancel").clicked() {
                            self.main_app_data.window = Window::NONE;
                        }
                    });
                });
            });
    }

    pub fn render_member_window(&mut self, ctx: &Context) {
        egui::Window::new("Add Member")
            .default_pos(&[50., 50.])
            .resizable(true)
            .title_bar(false)
            .open(&mut true)
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.horizontal(|ui| {
                        ui.label("Name");
                        ui.text_edit_singleline(&mut self.main_app_data.member_creation_window.name);
                    });
                    ui.horizontal(|ui| {
                        ui.label("Role");
                        ui.horizontal(|ui| {
                            for role in self.main_app_data.roles.clone() {
                                ui.radio_value(&mut self.main_app_data.member_creation_window.selected, role.clone(), role.name.clone());
                            }
                        })
                    });
                    ui.horizontal(|ui| {
                        ui.label("Capacity");
                        ui.text_edit_singleline(&mut self.main_app_data.member_creation_window.capacity);
                    });
                    ui.horizontal(|ui| {
                        if ui.button("Create").clicked() {
                            let cap = self.main_app_data.member_creation_window.capacity.parse::<f64>().unwrap();
                            let member = Member::new(self.main_app_data.member_creation_window.name.clone(),
                                                     self.main_app_data.member_creation_window.selected.clone(), cap);
                            self.main_app_data.members.push(member);
                            self.main_app_data.window = Window::NONE;
                        }
                        if ui.button("Cancel").clicked() {
                            self.main_app_data.window = Window::NONE;
                        }
                    });
                });
            });
    }
}