use bevy::prelude::*;
use bevy_egui::{EguiContext, egui::{self, Align2, Slider}};
use serde::{Deserialize, Serialize};

use crate::utils::data_loader as datal;
use datal::Data;
use super::GuiState;

pub const MAX_PHOTO_DEEP_DELTA: f64 = 100.0;
pub const MIN_PHOTO_DEEP_DELTA: f64 = 0.1;
pub const MAX_TEMP_DEEP_DELTA: f64 = 100.0;
pub const MIN_TEMP_DEEP_DELTA: f64 = 0.1;
pub const MAX_FLOW_DEEP_DELTA: f64 = 100.0;
pub const MIN_FLOW_DEEP_DELTA: f64 = 0.1;


pub struct EventControlDataChanged;

#[derive(Deserialize, Serialize, Clone)]
pub struct Deltas {
	pub timestamp: i64,
	pub photo_deep: f64,
	pub temp_deep: f64,
	pub flow_deep: f64
}

#[derive(Resource)]
pub struct Control {
	pub ranges: datal::Ranges,
	pub deltas: Deltas,
	pub deep: f64,
	pub timestamp: i64
}

impl Control {
	pub fn new(deltas: &Deltas) -> Self {
		Self {
			ranges: datal::Ranges::default(),
			deltas: deltas.clone(),
			deep: 0.0,
			timestamp: 0
		}
	}
}

pub fn update_ranges(data: Res<Data>, mut ctld: ResMut<Control>) {
	if data.is_changed() {
		ctld.ranges = data.ranges();
		if ctld.deep > ctld.ranges.deep_max {
			ctld.deep = ctld.ranges.deep_max;
		}
		if ctld.deep < ctld.ranges.deep_min {
			ctld.deep = ctld.ranges.deep_min;
		}
		if ctld.timestamp > ctld.ranges.timestamp_max {
			ctld.timestamp = ctld.ranges.timestamp_max;
		}
		if ctld.timestamp < ctld.ranges.timestamp_min {
			ctld.timestamp = ctld.ranges.timestamp_min;
		}
	}
}

pub fn show(mut cmd: Commands, mut ctx: ResMut<EguiContext>, mut ctld: ResMut<Control>, mut evw: EventWriter<EventControlDataChanged>) {
	egui::Area::new("TOP_CONTROL").anchor(Align2::CENTER_TOP, egui::Vec2::default()).show(ctx.ctx_mut(), |ui| {
		ui.horizontal(|ui| {
			let timestamp_max = ctld.ranges.timestamp_max;
			let timestamp_min = ctld.ranges.timestamp_min;
			if ui.add(Slider::new(&mut ctld.timestamp, timestamp_min..=timestamp_max)).changed() {
				evw.send(EventControlDataChanged);
			}
		});
		ui.horizontal(|ui| {
			if ui.add(Slider::new(&mut ctld.deltas.photo_deep, MIN_PHOTO_DEEP_DELTA..=MAX_PHOTO_DEEP_DELTA).text("Photo deep delta:")).changed() {
				evw.send(EventControlDataChanged);
			}
			ui.separator();
			if ui.add(Slider::new(&mut ctld.deltas.temp_deep, MIN_TEMP_DEEP_DELTA..=MAX_TEMP_DEEP_DELTA).text("Temp deep delta:")).changed() {
				evw.send(EventControlDataChanged);
			}
			ui.separator();
			if ui.add(Slider::new(&mut ctld.deltas.flow_deep, MIN_FLOW_DEEP_DELTA..=MAX_FLOW_DEEP_DELTA).text("Flow deep delta:")).changed() {
				evw.send(EventControlDataChanged);
			}
		});
	});

	egui::Area::new("DEEP_CONTROL").anchor(Align2::RIGHT_CENTER, egui::Vec2::default()).show(ctx.ctx_mut(), |ui| {
		ui.vertical(|ui| {
			let deep_max = ctld.ranges.deep_max;
			let deep_min = ctld.ranges.deep_min;
			if ui.add(Slider::new(&mut ctld.deep, deep_min..=deep_max).orientation(egui::SliderOrientation::Vertical)).changed() {
				evw.send(EventControlDataChanged);
			}
		});
	});
}