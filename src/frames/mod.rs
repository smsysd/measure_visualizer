use bevy::prelude::*;
use bevy_egui::{egui::{self, Align2, TextStyle, ScrollArea, RichText, Color32, Context}, EguiContext, EguiPlugin};
use egui_file::FileDialog;
use chrono::{DateTime, Local};

use crate::utils::data_loader as datal;

const LOG_LIMIT_DEF: usize = 25;

pub enum GuiState {
    Normal,
    OpenFile(FileDialog),
    GenerateData,
    SaveData(FileDialog)
}

impl Default for GuiState {
	fn default() -> Self {
		Self::Normal
	}
}

pub enum LogType {
	Info,
	Warn,
	Error
}

pub struct Logger {
	limit: usize,
	lines: Vec<(LogType, DateTime<Local>, String)>
}

impl Default for Logger {
	fn default() -> Self {
		Self {
			limit: LOG_LIMIT_DEF,
			lines: Vec::new()
		}
	}
}

impl Logger {
	pub fn push(&mut self, ltype: LogType, line: &str) {
		if self.lines.len() >= self.limit {
			self.lines.remove(0);
		}
		self.lines.push((ltype, Local::now(), String::from(line)));
	}
	
	pub fn show(&mut self, ctx: &mut EguiContext) {
		
		egui::Window::new("LOGGER")
		.anchor(Align2::RIGHT_BOTTOM, egui::vec2(0.0, 0.0))
		.default_size(egui::Vec2::new(400.0, 100.0))
		.resizable(true)
		.show(ctx.ctx_mut(), |ui| {
			ui.horizontal(|ui| {
				ui.label("Recent logs:");
				if ui.button("Clear").clicked() {
					self.lines.clear();
				}
			});
			ui.separator();
			let text_style = TextStyle::Body;
			let row_height = ui.text_style_height(&text_style);
			let num_rows = self.lines.len();
			ScrollArea::vertical().auto_shrink([false;2]).show_rows(
				ui,
				row_height,
				num_rows,
				|ui, row_range| {
					for row in row_range {
						let color = match self.lines[row].0 {
							LogType::Error => Color32::RED,
							LogType::Warn => Color32::GOLD,
							LogType::Info => Color32::LIGHT_GREEN
						};
						let time = self.lines[row].1.format("%H:%M:%S").to_string();
						let text = format!("[{}]: {}", time, self.lines[row].2);
						ui.label(RichText::new(text).color(color));
					}
				},
			);
		});
	}
}

#[derive(Resource)]
pub struct GuiApp {
	pub state: GuiState,
	pub logger: Logger
}

impl Default for GuiApp {
	fn default() -> Self {
		Self {
			state: GuiState::default(),
			logger: Logger::default()
		}
	}
}

pub fn gui(mut ctx: ResMut<EguiContext>, mut app: ResMut<GuiApp>, mut data: ResMut<datal::Data>) {
    match &mut app.as_mut().state {
        GuiState::Normal => {
            egui::Window::new("CONTROL").anchor(Align2::LEFT_TOP, egui::vec2(0.0, 0.0)).show(ctx.ctx_mut(), |ui| {
                if ui.button("Clear Data").clicked() {
                    data.clear();
                    app.logger.push(LogType::Info, "Data cleared");
                }
                if ui.button("Open File").clicked() {
                    app.as_mut().state = GuiState::OpenFile(FileDialog::open_file(None));
                }
                if ui.button("Generate Data").clicked() {
                    app.as_mut().state = GuiState::GenerateData;
                }
                if ui.button("Save Data").clicked() {
                    app.as_mut().state = GuiState::SaveData(FileDialog::save_file(None));
                }
            });
        },
        GuiState::OpenFile(fdialog) => {
            match fdialog.state() {
                egui_file::State::Cancelled => {
                    println!("OpenFie::Canceled");
                    app.as_mut().state = GuiState::Normal;
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
                                    app.logger.push(LogType::Info, &msg);
                                },
                                Err(e) => {
                                    let msg = format!("Fail to load data from file: {}", e);
                                    println!("{}", msg);
                                    app.logger.push(LogType::Error, &msg);
                                }
                            }
                            app.as_mut().state = GuiState::Normal;
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
                    app.as_mut().state = GuiState::Normal;
                }
            });
        },
        GuiState::SaveData(fdialog) => {
            match fdialog.path() {
                Some(path) => {
                    match datal::save_data(&path, &data) {
                        Err(e) => {
                            let msg = format!("Fail to save data to file: {}", e);
                            app.logger.push(LogType::Error, &msg);
                        },
                        _ => app.logger.push(LogType::Info, "Data success saved")
                    }
                    app.as_mut().state = GuiState::Normal;
                },
                None => {
                    fdialog.open();
                    fdialog.show(ctx.ctx_mut());
                }
            }
        }
    }
    app.logger.show(ctx.as_mut());
}