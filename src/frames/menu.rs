use bevy::prelude::*;
use bevy_egui::{egui::{self, Align2}, EguiContext};
use egui_file::FileDialog;

use crate::utils::data_loader as datal;
use datal::Data;

use super::{GuiState, logger::{LogType, Log}};

pub fn show(mut cmd: Commands, mut gst: ResMut<GuiState>, mut data: ResMut<Data>, mut ctx: ResMut<EguiContext>) {
	match gst.as_mut() {
        GuiState::Normal => {
            egui::Window::new("MENU").anchor(Align2::LEFT_TOP, egui::vec2(0.0, 0.0)).show(ctx.ctx_mut(), |ui| {
                if ui.button("Clear Data").clicked() {
                    data.clear();
					cmd.spawn(Log::new(LogType::Info, "Data cleared"));
                }
                if ui.button("Open File").clicked() {
                    *gst = GuiState::OpenFile(FileDialog::open_file(None));
                }
                if ui.button("Generate Data").clicked() {
                    *gst = GuiState::GenerateData;
                }
                if ui.button("Save Data").clicked() {
                    *gst = GuiState::SaveData(FileDialog::save_file(None));
                }
            });
        },
        GuiState::OpenFile(fdialog) => {
            match fdialog.state() {
                egui_file::State::Cancelled => {
                    println!("OpenFie::Canceled");
                    *gst = GuiState::Normal;
                },
                egui_file::State::Closed => {
                    fdialog.open();
                    fdialog.show(ctx.ctx_mut());
                },
                egui_file::State::Selected => {
                    println!("OpenFie::Selected");
                    match fdialog.path() {
                        Some(path) => {
                            match datal::load_data(&path) {
                                Ok(data_add) => {
                                    let added = data.add(data_add);
                                    let msg = format!("Data success loaded: photos: {}, temps: {}, flows: {}", added.0, added.1, added.2);
                                    cmd.spawn(Log::new(LogType::Info, &msg));
                                },
                                Err(e) => {
									cmd.spawn(Log::new(LogType::Error, &format!("Fail to load data from file: {}", e)));
								}
                            }
                            *gst = GuiState::Normal;
                        },
                        None => ()
                    }
                },
                egui_file::State::Open => {
                    fdialog.open();
                    fdialog.show(ctx.ctx_mut());
                }
            }
        },
        GuiState::GenerateData => {
            egui::Window::new("DATA GENERATOR").anchor(Align2::CENTER_CENTER, egui::vec2(0.0, 0.0)).show(ctx.ctx_mut(), |ui| {
                if ui.button("Exit").clicked() {
                    *gst = GuiState::Normal;
                }
            });
        },
        GuiState::SaveData(fdialog) => {
			match fdialog.state() {
                egui_file::State::Cancelled => {
                    println!("SaveData::Canceled");
                    *gst = GuiState::Normal;
                },
                egui_file::State::Closed => {
                    fdialog.open();
                    fdialog.show(ctx.ctx_mut());
                },
                egui_file::State::Selected => {
                    println!("SaveData::Selected");
					match fdialog.path() {
						Some(path) => {
							match datal::save_data(&path, &data) {
								Err(e) => {
									cmd.spawn(Log::new(LogType::Error, &format!("Fail to save data to file: {}", e)));
								},
								_ => {
									cmd.spawn(Log::new(LogType::Info, "Data success saved."));
								}
							}
							*gst = GuiState::Normal;
						},
						None => {
							fdialog.open();
							fdialog.show(ctx.ctx_mut());
						}
					}
                },
                egui_file::State::Open => {
                    fdialog.open();
                    fdialog.show(ctx.ctx_mut());
                }
			}
        }
    }
}