use std::os::unix::raw::uid_t;
use egui::{Context, Grid, Response, Ui};
use crate::app::entities::{Feature, Member, RenderMode, Role};
use crate::app::window_management::{MainAppData, RoleWindow, MemberOptions, Screen, Window, StoryOptions, FeatureOptions, ObjectiveOptions};

mod entities;
mod window_management;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct MainApp {
    main_app_data: MainAppData
}

impl Default for MainApp {
    fn default() -> Self {
        Self {
            main_app_data: MainAppData {
                // Example stuff:
                label: "Hello World!".to_owned(),
                members: Vec::new(),
                roles: Vec::new(),
                role_window: RoleWindow::new(String::new(), String::new()),
                member_creation_window: MemberOptions::new(&vec![]),
                story_creation_window: StoryOptions::new(),
                feature_creation_window: FeatureOptions {title: String::new()},
                objective_creation_window: ObjectiveOptions::new(),
                screen: Screen::SPRINTS,
                window: Window::NONE,
                features: Vec::new(),
            }
        }
    }
}

impl MainApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl eframe::App for MainApp {
    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Put your widgets into a `SidePanel`, `TopBottomPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:

            egui::menu::bar(ui, |ui| {
                // NOTE: no File->Quit on web pages!
                let is_web = cfg!(target_arch = "wasm32");
                if !is_web {
                    ui.menu_button("File", |ui| {
                        if ui.button("Quit").clicked() {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });
                    ui.add_space(16.0);
                }
                ui.menu_button("Login", |ui| {
                    todo!();
                });
                ui.menu_button("Create", |ui| {
                    if ui.button("Role").clicked() {
                        //self.main_app_data.role_window.show = true;
                        self.main_app_data.window = Window::ROLE;
                    }
                    if ui.button("Team Member").clicked() {
                        self.main_app_data.window = Window::MEMBER;
                    }
                    if ui.button("PI").clicked() {
                        self.main_app_data.window = Window::PI;
                    }
                    if ui.button("Sprint").clicked() {
                        self.main_app_data.window = Window::SPRINT;
                    }
                    if ui.button("Feature").clicked() {
                        self.main_app_data.window = Window::FEATURE;
                    }
                    if ui.button("Story").clicked() {
                        self.main_app_data.window = Window::STORY;
                    }
                    if ui.button("Objective").clicked() {
                        self.main_app_data.window = Window::OBJECTIVE;
                    }
                });

                egui::widgets::global_theme_preference_buttons(ui);
            });
        });
        egui::SidePanel::left("side_panel").show(ctx, |ui| {
            ui.heading("Side Panel");
            ui.vertical(|ui| {
                if ui.button("PI's & Sprints").clicked() {
                    self.main_app_data.screen = Screen::SPRINTS;
                }
                if ui.button("Members & Roles").clicked() {
                    self.main_app_data.screen = Screen::MEMBERS;
                }
                if ui.button("Features & Stories").clicked() {
                    self.main_app_data.screen = Screen::FEATURES;
                }
            })
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            match self.main_app_data.screen {
                Screen::SPRINTS => {
                    self.render_sprints_screen(ctx, ui)
                },
                Screen::MEMBERS => {
                    self.render_member_screen(ctx, ui)
                }
                Screen::FEATURES => {
                    self.render_features_screen(ctx, ui)
                }
            }

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                egui::warn_if_debug_build(ui);
            });
            match self.main_app_data.window {
                Window::ROLE => {
                    self.render_role_window(ctx);
                }
                Window::MEMBER => {
                    self.render_member_window(ctx);
                }
                Window::STORY => {
                    self.render_story_window(ctx);
                }
                Window::FEATURE => {
                    self.render_feature_window(ctx);
                }
                Window::OBJECTIVE => {
                    self.render_objective_window(ctx);
                }
                _ => ()
            }
        });
    }

    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }
}

impl MainApp {
    fn render_member_screen(&mut self, ctx: &Context, ui: &mut egui::Ui) {
        ui.heading("Roles");
        for role in self.main_app_data.roles.iter().to_owned() {
            ui.add(Role::new(role.name.clone(), role.velocity));
        }
        ui.heading("Members");
        for member in self.main_app_data.members.iter().to_owned() {
            ui.add(member.clone());
        }
    }

    fn render_features_screen(&mut self, ctx: &Context, ui: &mut egui::Ui) {
        ui.heading("Features");
        for feature in self.main_app_data.features.clone() {
            ui.separator();
            if ui.button("Delete Feature").clicked() {
                self.main_app_data.features.retain(|feature2: &Feature| !feature.eq(&feature2))
            }
            if ui.add(feature.clone()).clicked() {
                println!("Clicked");
                let feature_mut = self.main_app_data.get_feature_mut(&feature.name).unwrap();
                match feature_mut.render_mode {
                    RenderMode::OneLine => {
                        feature_mut.render_mode = RenderMode::Full;
                    },
                    RenderMode::Full => {
                        feature_mut.render_mode = RenderMode::OneLine;
                    }
                }
            }
        }
    }

    fn render_sprints_screen(&mut self, ctx: &Context, ui: &mut egui::Ui) {
        ui.heading("Sprints");
        for role in self.main_app_data.roles.iter().to_owned() {
            ui.add(Role::new(role.name.clone(), role.velocity));
        }
    }
}