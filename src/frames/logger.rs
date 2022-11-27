use bevy::prelude::*;
use bevy_egui::EguiContext;
use chrono::{DateTime, Local};
use bevy_egui::{egui::{self, Align2, TextStyle, ScrollArea, RichText, Color32}};

const LOG_LIMIT_DEF: usize = 25;

pub enum LogType {
	Info,
	Warn,
	Error
}

#[derive(Component)]
pub struct Log {
	pub ltype: LogType,
	pub dt: DateTime<Local>,
	pub text: String
}

impl Log {
	pub fn new (ltype: LogType, text: &str) -> Self {
		Self {
			dt: Local::now(),
			ltype: ltype,
			text: String::from(text)
		}
	}
}

pub struct EventClear;

pub fn clear(mut cmd: Commands, logs: Query<(Entity, &Log)>, evr: EventReader<EventClear>) {
	if !evr.is_empty() {
		for l in &logs {
			cmd.entity(l.0).despawn();
		}
		return;
	}
}

pub fn show(mut cmd: Commands, mut ctx: ResMut<EguiContext>, logs: Query<&Log>, mut evw: EventWriter<EventClear>) {
	egui::Window::new("LOGGER")
	.anchor(Align2::RIGHT_BOTTOM, egui::vec2(0.0, 0.0))
	.default_size(egui::Vec2::new(400.0, 100.0))
	.resizable(true)
	.show(ctx.ctx_mut(), |ui| {
		ui.horizontal(|ui| {
			ui.label("Recent logs:");
			if ui.button("Clear").clicked() {
				evw.send(EventClear);
			}
		});
		ui.separator();
		let text_style = TextStyle::Body;
		let row_height = ui.text_style_height(&text_style);
		let num_rows = logs.iter().count();
		ScrollArea::vertical().auto_shrink([false;2]).show_rows(
			ui,
			row_height,
			num_rows,
			|ui, _| {
				for row in &logs {
					let color = match row.ltype {
						LogType::Error => Color32::RED,
						LogType::Warn => Color32::GOLD,
						LogType::Info => Color32::LIGHT_GREEN
					};
					let time = row.dt.format("%H:%M:%S").to_string();
					let text = format!("[{}]: {}", time, row.text);
					ui.label(RichText::new(text).color(color));
				}
			},
		);
	});
}