use bevy::{prelude::*, sprite::Material2dPlugin};

use self::pixelise::PixeliseMaterial;

pub mod pixelise;

pub struct ShaderPlugin;

impl Plugin for ShaderPlugin {
	fn build(&self, app: &mut App) {
		app.add_plugin(Material2dPlugin::<PixeliseMaterial>::default());
	}
}