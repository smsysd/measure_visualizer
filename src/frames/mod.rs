use bevy::prelude::*;
use bevy_egui::{egui::{self, Align2, TextStyle, ScrollArea, RichText, Color32, Context}, EguiContext, EguiPlugin};
use egui_file::FileDialog;
use chrono::{DateTime};

pub mod logger;
pub mod menu;
pub mod control;

use crate::utils::{data_loader as datal, config::Config};

#[derive(Resource)]
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

#[derive(Resource)]
pub struct GuiApp {

}

impl Default for GuiApp {
	fn default() -> Self {
		Self {

		}
	}
}

fn gui_setup(mut cmd: Commands, config: Res<Config>) {
    cmd.insert_resource(GuiState::default());
    cmd.insert_resource(control::Control::new(&config.default_deltas));
}

impl Plugin for GuiApp {
    fn build(&self, app: &mut App) {
        app.add_startup_system(gui_setup);
        app.add_system(logger::show);
        app.add_system(logger::clear);
        app.add_system(menu::show);
        app.add_system(control::show);
        app.add_system(control::update_ranges);
        app.add_event::<logger::EventClear>();
        app.add_event::<control::EventControlDataChanged>();
    }

    fn name(&self) -> &str {
        "GUI APP"
    }

    fn is_unique(&self) -> bool {
        true
    }
}
