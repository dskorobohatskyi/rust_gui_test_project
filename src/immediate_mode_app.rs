// #[cfg(feature = "immediate-mode")] // This file is compiled only for `immediate-mode`
use eframe::egui::{self, Align, CentralPanel, Grid, Layout, TopBottomPanel, Window};

use crate::common::{Tab, UserInfo};

pub fn run() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Immediate Mode App",
        options,
        Box::new(|_cc| Box::new(ImmediateModeApp::default())),
    )
}

#[derive(Default)]
pub struct ImmediateModeApp {
    active_tab: Tab,
    show_main_modal: bool,
    show_save_settings_modal: bool,

    saved_user_info: Option<UserInfo>,
    temp_user_name: String,
    temp_user_age: u32,
}

impl eframe::App for ImmediateModeApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.selectable_value(&mut self.active_tab, Tab::Home, "Home");
                ui.selectable_value(&mut self.active_tab, Tab::Settings, "Settings");
                ui.selectable_value(&mut self.active_tab, Tab::About, "About");
            });
        });

        CentralPanel::default().show(ctx, |ui| match self.active_tab {
            Tab::Home => self.show_home_tab(ctx, ui),
            Tab::Settings => self.show_settings_tab(ctx, ui),
            Tab::About => self.show_about_tab(ctx, ui),
        });
    }
}

impl ImmediateModeApp {
    // Layout and content for the "Home" tab
    fn show_home_tab(&mut self, ctx: &egui::Context, ui: &mut egui::Ui) {
        ui.heading("Home Tab");
        ui.vertical_centered(|ui| {
            ui.label("This is the home tab. You can add various content here.");
            ui.separator();

            ui.horizontal(|ui| {
                if ui.button("Open User Info").clicked() {
                    self.show_main_modal = true;
                }
            });
        });

        // Show modal window when flag is set
        if self.show_main_modal {
            Window::new("Current Info")
                .collapsible(false)
                .resizable(false)
                .show(ctx, |ui| {
                    if let Some(valid_user_info) = &self.saved_user_info {
                        ui.horizontal(|ui| {
                            ui.label(format!("Name: {}", valid_user_info.name));
                        });

                        ui.horizontal(|ui| {
                            ui.label(format!("Age: {}", valid_user_info.age));
                        });
                    } else {
                        ui.label("No valid info set yet!");
                    }

                    if ui.button("Close").clicked() {
                        self.show_main_modal = false;
                    }
                });
        }
    }

    // Layout and content for the "Settings" tab
    fn show_settings_tab(&mut self, ctx: &egui::Context, ui: &mut egui::Ui) {
        ui.heading("Settings Tab");

        ui.vertical(|ui| {
            ui.label("Settings:");
            ui.separator();

            // Grid Layout for form-like structure
            Grid::new("settings_grid").striped(true).show(ui, |ui| {
                ui.label("Your name:");
                ui.text_edit_singleline(&mut self.temp_user_name);
                ui.end_row();

                ui.label("Your age:");
                ui.add(egui::DragValue::new(&mut self.temp_user_age));
                ui.end_row();
            });

            ui.horizontal(|ui| {
                if ui.button("Save Settings").clicked() {
                    self.show_save_settings_modal = true;
                    self.saved_user_info = Some(UserInfo {
                        name: self.temp_user_name.clone(),
                        age: self.temp_user_age,
                    }); // TODO what should be instead of clone?
                }
            });

            if self.show_save_settings_modal {
                Window::new("Info")
                    .collapsible(false)
                    .resizable(false)
                    .show(ctx, |ui| {
                        ui.label("Settings saved!");

                        if ui.button("Close").clicked() {
                            self.show_save_settings_modal = false;
                        }
                    });
            }
        });
    }

    // Layout and content for the "About" tab
    fn show_about_tab(&mut self, _: &egui::Context, ui: &mut egui::Ui) {
        ui.with_layout(Layout::top_down(Align::LEFT), |ui| {
            ui.heading("About Tab");
            ui.label("This is a simple demo of a more complex egui application.");
            ui.label("It has multiple tabs, a modal dialog, and various layouts.");
        });
    }
}
