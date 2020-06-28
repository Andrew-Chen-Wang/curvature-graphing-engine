use amethyst::{
    core::Transform,
    ecs::prelude::{Entity, Join, ReadStorage, System, WriteStorage},
    prelude::{Builder, World, WorldExt},
    renderer::{
        camera::Camera,
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

#[derive(Default)]
pub struct HeadlampSystem;

/// Updates the headlamp.
/// Mainly used for moving it around and turning it on/off
impl<'a> System<'a> for HeadlampSystem {
    type SystemData = (
        ReadStorage<'a, Camera>,
        WriteStorage<'a, Light>,
        WriteStorage<'a, Transform>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (cameras, mut lights, mut transforms) = data;

        // Get camera's transform and set the headlamp's to the same.
        let (_, target_transform) = ((&cameras, &mut transforms).join().next()).unwrap();
        let camera_isometry = *target_transform.isometry();
        for (_, transform) in (&mut lights, &mut transforms).join() {
            transform.set_isometry(camera_isometry);
        }
    }
}
