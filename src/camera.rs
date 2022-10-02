use bevy::{
    prelude::*,
    render::{
        render_resource::{
            Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
        },
        texture::BevyDefault,
        view::RenderLayers, camera::RenderTarget,
    }, sprite::{Material2d, MaterialMesh2dBundle},
};

use crate::shaders::pixelise::PixeliseMaterial;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(camera_setup);
    }
}

#[derive(Component)]
pub struct MainCamera;

#[derive(Component)]
pub struct Screen;

#[derive(Deref, DerefMut)]
pub struct PostProcessingLayer(pub RenderLayers);

#[derive(Deref, DerefMut)]
pub struct CameraRenderImage(pub Handle<Image>);

#[derive(Deref, DerefMut)]
pub struct ScreenRes(pub Entity);

fn camera_setup(
    mut commands: Commands,
    windows: Res<Windows>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut images: ResMut<Assets<Image>>,
    mut post_processing_materials: ResMut<Assets<PixeliseMaterial>>,
) {
    let window = windows.primary();

    let size = Extent3d {
        width: window.width() as u32,
        height: window.height() as u32,
        ..Default::default()
    };

    // This is the texture that will be rendered to.
    let mut image = Image {
        texture_descriptor: TextureDescriptor {
            label: None,
            size,
            dimension: TextureDimension::D2,
            format: TextureFormat::bevy_default(),
            mip_level_count: 1,
            sample_count: 1,
            usage: TextureUsages::TEXTURE_BINDING
                | TextureUsages::COPY_DST
                | TextureUsages::RENDER_ATTACHMENT,
        },
        ..default()
    };

    // Fill image.data with zeroes
    image.resize(size);

    let image_handle = images.add(image);

    // Add main camera rendering to the image
    commands
        .spawn_bundle(Camera2dBundle {
            camera: Camera {
				target: RenderTarget::Image(image_handle.clone()),
                priority: 0,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Name::new("MainCamera"))
        .insert(MainCamera);

    // This specifies the layer used for the post processing camera, which will be attached to the post processing camera and 2d quad.
    let post_processing_pass_layer = RenderLayers::layer((RenderLayers::TOTAL_LAYERS - 1) as u8);

    let post_processing_pass_layer_resource = PostProcessingLayer(post_processing_pass_layer);

    // This material has the texture that has been rendered.
    let material_handle = post_processing_materials.add(PixeliseMaterial {
        source_image: image_handle.clone(),
    });

    set_post_processing_effects(
        &mut commands,
        material_handle,
        &windows,
        &mut meshes,
        &post_processing_pass_layer_resource,
    );

    commands.insert_resource(post_processing_pass_layer_resource);
    commands.insert_resource(CameraRenderImage(image_handle));

    // The post-processing pass camera.
    commands
        .spawn_bundle(Camera2dBundle {
            camera: Camera {
                // renders after the first main camera which has default value: 0.
                priority: 1,
                ..default()
            },
            ..Camera2dBundle::default()
        })
        .insert(post_processing_pass_layer)
        .insert(Name::new("SecondaryCamera"));
}

fn set_post_processing_effects<M: Material2d>(
	commands: &mut Commands,
	material: Handle<M>,
    windows: &Res<Windows>,
	meshes: &mut ResMut<Assets<Mesh>>,
	post_processing_pass_layer: &PostProcessingLayer,
) {
    let window = windows.primary();

    let size = Vec2::new(
        window.width(),
        window.height(),
    );

    let quad_handle = meshes.add(Mesh::from(shape::Quad::new(size)));

	// Post processing 2d quad, with material using the render texture done by the main camera, with a custom shader.
	let screen = commands
		.spawn_bundle(MaterialMesh2dBundle {
			mesh: quad_handle.into(),
			material,
			transform: Transform {
				translation: Vec3::new(0.0, 0.0, 1.5),
				..default()
			},
			..default()
		})
		.insert(post_processing_pass_layer.0)
		.insert(Name::new("Screen"))
		.insert(Screen).id();

	commands.insert_resource(ScreenRes(screen));
}