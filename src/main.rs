use std::{thread, time::Duration};

use bevy::prelude::*;
use bevy_egui::EguiPlugin;

pub mod utils;
use utils::data_loader as datal;

mod frames;
use frames::GuiApp;

mod repr_2d;
mod repr_3d;

fn setup() {

}

fn main() {
    let mut app = App::new();
    app.insert_resource(GuiApp::default());
    app.insert_resource(datal::Data::default());
    app.add_plugins(DefaultPlugins);
    app.add_startup_system(setup);
    app.add_plugin(EguiPlugin);
    app.add_system(frames::gui);
    app.run();
}
