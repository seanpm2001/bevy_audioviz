use bevy::prelude::*;
use bevy::reflect::{TypePath, TypeUuid};
use bevy::render::render_resource::{AsBindGroup, ShaderRef};
use bevy::sprite::Material2d;

use crate::ARRAY_UNIFORM_SIZE;

use crate::Colors;

#[derive(Resource)]
pub struct CircleSplitEntity(pub Option<Entity>);
impl Default for CircleSplitEntity {
    fn default() -> Self {
        CircleSplitEntity(None)
    }
}

#[derive(Component, Debug, Clone, AsBindGroup, TypeUuid, TypePath, Asset)]
#[uuid = "a3dafd0f-45ef-4d05-9a78-e309a208859b"]
pub struct CircleSplitMaterial {
    #[uniform(0)]
    pub left_data: [Vec4; ARRAY_UNIFORM_SIZE], // Use an array of vec4s (which is an array of [f32; 4] in Rust)}
    #[uniform(1)]
    pub right_data: [Vec4; ARRAY_UNIFORM_SIZE], // Use an array of vec4s (which is an array of [f32; 4] in Rust)}
    #[uniform(2)]
    pub viewport_width: f32,
    #[uniform(3)]
    pub viewport_height: f32,
    #[uniform(4)]
    pub monochrome: u32,
    #[uniform(5)]
    pub colors: [Vec4; 4],
}
impl Material2d for CircleSplitMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/circle_split_fragment.wgsl".into()
    }
}
//a Mandelbrot material with the given uniforms.
pub fn prepare_circle_split_material(
    materials: &mut ResMut<Assets<CircleSplitMaterial>>,
    width: f32,
    height: f32,
    colors: &Colors,
) -> Handle<CircleSplitMaterial> {
    let material = CircleSplitMaterial {
        left_data: [Vec4::new(0.0, 0.0, 0.0, 0.0); ARRAY_UNIFORM_SIZE],
        right_data: [Vec4::new(0.0, 0.0, 0.0, 0.0); ARRAY_UNIFORM_SIZE],
        viewport_width: width,
        viewport_height: height,
        monochrome: if colors.monochrome { 1 } else { 0 },
        colors: [
            Vec4::new(
                colors.color_start.r(),
                colors.color_start.g(),
                colors.color_start.b(),
                colors.color_start.a(),
            ),
            Vec4::new(
                colors.color_middle.r(),
                colors.color_middle.g(),
                colors.color_middle.b(),
                colors.color_middle.a(),
            ),
            Vec4::new(
                colors.color_end.r(),
                colors.color_end.g(),
                colors.color_end.b(),
                colors.color_end.a(),
            ),
            Vec4::ZERO,
        ],
        //color_start: [0f32; 4],
        //color_middle: [0f32; 3],
        //color_end: [0f32; 3],
    };
    materials.add(material)
}
