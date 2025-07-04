use egui::{Response, Ui};
use crate::app::entities::{Member, Role};

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
    SPRINTS
}

#[derive(serde::Deserialize, serde::Serialize, Default)]
pub enum Window {
    #[default]
    NONE,
    ROLE,
    MEMBER,
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct MainAppData {
    pub label: String,
    pub members: Vec<Member>,
    pub roles: Vec<Role>,
    #[serde(skip)]
    pub role_window: RoleWindow,
    #[serde(skip)]
    pub screen: Screen,
    pub window: Window,
}

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub struct MembersScreenInfo {
    pub members: Vec<Member>,
    pub roles: Vec<Role>,
}
pub fn render_role_window(mut app: &mut MainAppData, ctx: &egui::Context, ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        egui::Window::new("Add Member")
            .fixed_pos(&[50., 50.])
            .resizable(false)
            .title_bar(false)
            .open(&mut true)
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.horizontal(|ui| {
                        ui.label("Name");
                        ui.text_edit_singleline(&mut app.role_window.role_title);
                    });
                    ui.horizontal(|ui| {
                        ui.label("Velocity");
                        ui.text_edit_singleline(&mut app.role_window.velocity);
                    });
                    ui.horizontal(|ui| {
                        if ui.button("Create Role").clicked() {
                            let vel = app.role_window.velocity.parse::<f64>().unwrap();
                            app.roles.push(Role::new(app.role_window.role_title.clone(), vel.clone()));
                            app.window = Window::NONE;
                        }
                        if ui.button("Cancel").clicked() {
                            app.window = Window::NONE;
                        }
                    });
                });
            });
    });
}

pub fn render_member_window(mut app: &mut MainAppData, ctx: &egui::Context, ui: &mut egui::Ui) {
    let selected_member: Role = Role::new("".parse().unwrap(), 0.0);
    let member_selector = &mut MemberOptions::new(app.roles.clone(), selected_member);
    let name: &mut String = &mut String::new();
    let capacity: &mut String = &mut String::new();
    ui.horizontal(|ui| {
        egui::Window::new("Add Member")
            .fixed_pos(&[50., 50.])
            .resizable(false)
            .title_bar(false)
            .open(&mut true)
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.horizontal(|ui| {
                        ui.label("Name");
                        ui.text_edit_singleline(name);
                    });
                    ui.horizontal(|ui| {
                        ui.label("Role");
                        let response = ui.add(member_selector);
                        response.output_event()
                    });
                    ui.horizontal(|ui| {
                        ui.label("Capacity");
                        ui.text_edit_singleline(capacity);
                    });
                    ui.horizontal(|ui| {
                        if ui.button("Create").clicked() {
                            let cap = capacity.parse::<f64>().unwrap();
                            app.members.push(Member::new(name.clone(), member_selector.selected.clone(), cap));
                            app.window = Window::NONE;
                        }
                        if ui.button("Cancel").clicked() {
                            app.window = Window::NONE;
                        }
                    });
                });
            });
    });
}

struct MemberOptions {
    roles: Vec<Role>,
    selected: Role,
    name: String,
    capacity: f64
}

impl MemberOptions {
    pub fn new(roles: Vec<Role>, selected_member: Role) -> Self {
        MemberOptions{roles, selected: selected_member, name: String::new(), capacity: 0.0}
    }
}
impl egui::Widget for MemberOptions {
    fn ui(mut self, ui: &mut Ui) -> Response {
        ui.horizontal(|ui| {
            for role in self.roles {
               if  ui.button(role.name.clone()).clicked() {
                   self.selected = role.clone();
               }
            }
        }).response
    }
}