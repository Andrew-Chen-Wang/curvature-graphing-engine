use amethyst::{
    core::Transform,
    ecs::prelude::Entity,
    prelude::{Builder, World, WorldExt},
    renderer::{
        light::{Light, PointLight},
        palette::rgb::Rgb,
    },
};

/// Initializes a headlamp for the user. Optionally can be turned
/// off for photon usage.
pub fn initialize_light(world: &mut World) -> Entity {
    let light: Light = PointLight {
        intensity: 10.0,
        color: Rgb::new(1.0, 1.0, 1.0),
        ..PointLight::default()
    }
    .into();

    // TODO Delete this when you start creating an initial object on your own
    let mut transform = Transform::default();
    transform.set_translation_xyz(5.0, 5.0, 20.0);

    world.create_entity().with(light).with(transform).build()
}
