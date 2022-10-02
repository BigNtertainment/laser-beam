#import bevy_pbr::mesh_view_bindings

@group(1) @binding(0)
var texture: texture_2d<f32>;

@group(1) @binding(1)
var our_sampler: sampler;

let rows: f32 = 256.0;

fn get_uv(pos: vec2<f32>) -> vec2<f32> {
    return pos / vec2(view.width, view.height);
}

fn pixelate(uv: vec2<f32>, size: vec2<f32>) -> vec2<f32> {
    return floor(uv * size) / size;
}

fn get_texture_color(uv: vec2<f32>) -> vec4<f32> {
    return textureSample(texture, our_sampler, uv);
}

@fragment
fn fragment(
    @builtin(position) position: vec4<f32>,
    #import bevy_sprite::mesh2d_vertex_output
) -> @location(0) vec4<f32> {

    let uv = get_uv(position.xy);

    let cols = rows * view.width / view.height;

    let texture_uv = uv;
    let texture_uv = pixelate(texture_uv, vec2(cols, rows));

    let color = get_texture_color(texture_uv);

    return color;

    // return vec4<f32>(1.0, 0.0, 0.0, 1.0);
}