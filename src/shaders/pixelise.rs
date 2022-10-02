use bevy::{prelude::*, sprite::Material2d, render::render_resource::{ShaderRef, AsBindGroup}, reflect::TypeUuid};

#[derive(AsBindGroup, TypeUuid, Clone)]
#[uuid = "f13048de-7114-45d8-a0bd-80ca1c8bf66c"]
pub struct PixeliseMaterial {
	#[texture(0)]
	#[sampler(1)]
	pub source_image: Handle<Image>,
}

impl Material2d for PixeliseMaterial {
	fn fragment_shader() -> ShaderRef {
		"shaders/pixelise.wgsl".into()
	}
}