use amethyst::{
    renderer::{
        light::{Light, PointLight},
        palette::rgb::Rgb
    },
    prelude::{World, Builder, WorldExt},
    core::Transform,
    ecs::prelude::Entity
};

/// Initializes a headlamp for the user. Optionally can be turned
/// off for photon usage.
pub fn initialize_light(world: &mut World) -> Entity {
    let light: Light = PointLight {
        intensity: 10.0,
        color: Rgb::new(1.0, 1.0, 1.0),
        ..PointLight::default()
    }.into();

    let mut transform = Transform::default();
    transform.set_translation_xyz(5.0, 5.0, 20.0);

    world.create_entity()
        .with(light)
        .with(transform)
        .build()
}
