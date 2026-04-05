use egui::{Context};
use crate::app::entities::{Feature, Member, Objective, Role, Story};
use crate::app::model::pi::PI;
use crate::app::window_data::{StoryOptions, Window};
use crate::MainApp;

impl MainApp {

    pub fn render_role_window(&mut self, ctx: &Context) {
        egui::Window::new("Add Member")
            .fixed_pos(&[50., 50.])
            .resizable(false)
            .title_bar(false)
            .open(&mut true)
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.horizontal(|ui| {
                        ui.label("Name");
                        ui.text_edit_singleline(&mut self.window_data.role_window.role_title);
                    });
                    ui.horizontal(|ui| {
                        ui.label("Velocity");
                        ui.text_edit_singleline(&mut self.window_data.role_window.velocity);
                    });
                    ui.horizontal(|ui| {
                        if ui.button("Create Role").clicked() {
                            let vel = self.window_data.role_window.velocity.parse::<f64>().unwrap();
                            self.main_app_data.roles.push(Role::new(self.window_data.role_window.role_title.clone(), vel.clone()));
                            self.window_data.window = Window::NONE;
                        }
                        if ui.button("Cancel").clicked() {
                            self.window_data.window = Window::NONE;
                        }
                    });
                });
            });
    }

    pub fn render_story_window(&mut self, ctx: &Context) {
        egui::Window::new("Add Story")
            .default_pos(&[50., 50.])
            .resizable(true)
            .title_bar(false)
            .open(&mut true)
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.horizontal(|ui| {
                        ui.label("Name");
                        ui.text_edit_singleline(&mut self.window_data.story_creation_window.title);
                    });
                    ui.horizontal(|ui| {
                        ui.label("Description");
                        ui.text_edit_multiline(&mut self.window_data.story_creation_window.description);
                    });
                    ui.horizontal(|ui| {
                        ui.label("Story Points");
                        ui.text_edit_singleline(&mut self.window_data.story_creation_window.story_points);
                    });
                    ui.horizontal(|ui| {
                        egui::ComboBox::from_label("Feature")
                            .selected_text(format!("{:?}", self.window_data.story_creation_window.feature))
                            .show_ui(ui, |ui| {
                                for feature in self.main_app_data.features.clone() {
                                    ui.selectable_value(&mut self.window_data.story_creation_window.feature, feature.clone(), feature.name.clone());
                                }
                            });
                    });
                    ui.horizontal(|ui| {
                        egui::ComboBox::from_label("Objective")
                            .selected_text(format!("{:?}", self.window_data.story_creation_window.objective))
                            .show_ui(ui, |ui| {
                                for objective in self.main_app_data.get_feature(&self.window_data.story_creation_window.feature.name).unwrap().objectives.clone() {
                                    ui.selectable_value(&mut self.window_data.story_creation_window.objective, objective.clone(), objective.title.clone());
                                }
                            });
                    });
                    ui.horizontal(|ui| {
                        ui.label("PI");
                        for pi in self.main_app_data.pis.clone() {
                            ui.radio_value(&mut self.window_data.story_creation_window.pi, pi.clone(), pi.name.clone());
                        }
                    });
                    ui.horizontal(|ui| {
                        ui.label("Sprint");
                        for sprint in self.window_data.story_creation_window.pi.sprints.clone() {
                            ui.radio_value(&mut self.window_data.story_creation_window.sprint, sprint.clone(), sprint.name.clone());
                        }
                    });
                    ui.horizontal(|ui| {
                        if ui.button("Create").clicked() {
                            let story_points = self.window_data.story_creation_window.story_points.parse::<f64>().unwrap();

                            let story = Story::new(self.window_data.story_creation_window.title.clone(),
                                                   story_points,
                                                   self.window_data.story_creation_window.description.clone(),
                                                   self.window_data.story_creation_window.sprint.clone());

                            let feature = self.window_data.story_creation_window.feature.name.clone();
                            let ob = self.window_data.story_creation_window.objective.title.clone();
                            self.main_app_data.add_story_to_objective(&feature, &ob, story);
                            self.window_data.story_creation_window = StoryOptions::new();
                            self.window_data.window = Window::NONE;
                            }
                        if ui.button("Cancel").clicked() {
                            self.window_data.window = Window::NONE;
                        }
                    });
                    });
                });
            }

    pub fn render_feature_window(&mut self, ctx: &Context) {
        egui::Window::new("Add Feature")
            .default_pos(&[50., 50.])
            .resizable(true)
            .title_bar(false)
            .open(&mut true)
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.horizontal(|ui| {
                        ui.label("Name");
                        ui.text_edit_singleline(&mut self.window_data.feature_creation_window.title);
                    });
                    ui.horizontal(|ui| {
                        if ui.button("Create").clicked() {
                            let feature = Feature::new(self.window_data.feature_creation_window.title.clone());
                            self.main_app_data.features.push(feature);
                            self.window_data.window = Window::NONE;
                        }
                        if ui.button("Cancel").clicked() {
                            self.window_data.window = Window::NONE;
                        }
                    });
                });
            });
    }

    pub fn render_objective_window(&mut self, ctx: &Context) {
        egui::Window::new("Add Objective")
            .default_pos(&[50., 50.])
            .resizable(true)
            .title_bar(false)
            .open(&mut true)
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.horizontal(|ui| {
                        ui.label("Name");
                        ui.text_edit_singleline(&mut self.window_data.objective_creation_window.title);
                    });
                    egui::ComboBox::from_label("Feature")
                        .selected_text(format!("{:?}", self.window_data.objective_creation_window.feature))
                        .show_ui(ui, |ui| {
                            for feature in self.main_app_data.features.clone() {
                                ui.selectable_value(&mut self.window_data.objective_creation_window.feature, feature.clone(), feature.name.clone());
                            }
                        });
                    ui.horizontal(|ui| {
                        if ui.button("Create").clicked() {
                            let objective = Objective::new(self.window_data.objective_creation_window.title.clone());
                            let feature_name = self.window_data.objective_creation_window.feature.name.clone();
                            self.main_app_data.get_feature_mut(&feature_name).unwrap().add_objective(objective);
                            self.window_data.window = Window::NONE;
                        }
                        if ui.button("Cancel").clicked() {
                            self.window_data.window = Window::NONE;
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
                        ui.text_edit_singleline(&mut self.window_data.member_creation_window.name);
                    });
                    ui.horizontal(|ui| {
                        ui.label("Role");
                        ui.horizontal(|ui| {
                            for role in self.main_app_data.roles.clone() {
                                ui.radio_value(&mut self.window_data.member_creation_window.selected, role.clone(), role.name.clone());
                            }
                        })
                    });
                    ui.horizontal(|ui| {
                        ui.label("Capacity");
                        ui.text_edit_singleline(&mut self.window_data.member_creation_window.capacity);
                    });
                    ui.horizontal(|ui| {
                        if ui.button("Create").clicked() {
                            let cap = self.window_data.member_creation_window.capacity.parse::<f64>().unwrap();
                            let member = Member::new(self.window_data.member_creation_window.name.clone(),
                                                     self.window_data.member_creation_window.selected.clone(), cap);
                            self.main_app_data.members.push(member);
                            self.window_data.window = Window::NONE;
                        }
                        if ui.button("Cancel").clicked() {
                            self.window_data.window = Window::NONE;
                        }
                    });
                });
            });
    }

    pub fn render_pi_window(&mut self, ctx: &Context) {
        egui::Window::new("Add PI")
            .default_pos(&[50., 50.])
            .resizable(true)
            .title_bar(false)
            .open(&mut true)
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.add(&mut self.window_data.pi_creation_window);
                    ui.horizontal(|ui| {
                        if ui.button("Create").clicked() {
                            let pi = PI::create(&self.window_data.pi_creation_window);
                            self.main_app_data.pis.push(pi);
                            self.window_data.window = Window::NONE;
                        }
                        if ui.button("Cancel").clicked() {
                            self.window_data.window = Window::NONE;
                        }
                    })
                })
            });
    }

    pub fn render_capacity_window(&mut self, ctx: &Context) {
        egui::Window::new("Add PI")
            .default_pos(&[50., 50.])
            .resizable(true)
            .title_bar(false)
            .open(&mut true)
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.add(&mut self.window_data.capacity_window);
                        if ui.button("Cancel").clicked() {
                            self.window_data.window = Window::NONE;
                        }
                    })
                });
    }
}