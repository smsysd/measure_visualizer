use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

use crate::frames::control::{Control, EventControlDataChanged};

pub struct Repr2D {

}

impl Default for Repr2D {
	fn default() -> Self {
		Self {
			
		}
	}
}

impl Plugin for Repr2D {
	fn build(&self, app: &mut App) {
		app.add_startup_system(setup);
		app.add_system(redraw);
	}

	fn name(&self) -> &str {
		"Repr2D"
	}

	fn is_unique(&self) -> bool {
		true
	}
}

fn setup(mut cmd: Commands, asset_srv: Res<AssetServer>, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<ColorMaterial>>) {
	cmd.spawn(Camera2dBundle::default());
    cmd.spawn(SpriteBundle {
        texture: asset_srv.load("test1.png"),
        ..default()
    });
	// Circle
	cmd.spawn(MaterialMesh2dBundle {
		mesh: meshes.add(shape::Circle::new(50.).into()).into(),
		material: materials.add(ColorMaterial::from(Color::PURPLE)),
		transform: Transform::from_translation(Vec3::new(-100., 0., 0.)),
		..default()
	});
}

fn redraw(mut cmd: Commands, ctld: Res<Control>, evr: EventReader<EventControlDataChanged>) {
	if !evr.is_empty() {
		println!("CONTROL DATA CHANGED>");
	}
}