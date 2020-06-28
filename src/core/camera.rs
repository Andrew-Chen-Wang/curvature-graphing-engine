use amethyst::{
    controls::FlyControlTag,
    core::Transform,
    prelude::{Builder, World, WorldExt},
    renderer::Camera,
    utils::auto_fov::AutoFov,
};

pub fn initialize_camera(world: &mut World) {
    let mut transform = Transform::default();
    transform.set_translation_xyz(0.0, 0.0, 10.0);

    let entity = world
        .create_entity()
        .with(Camera::standard_3d(800.0, 600.0))
        .with(transform)
        .build();
    // Helps to change object size on screen due to aspect ratios.
    world
        .write_component::<AutoFov>()
        .insert(entity, Default::default())
        .expect("Unable to attach AutoFov to camera.");
    // For flying and rotating
    world
        .write_storage::<FlyControlTag>()
        .insert(entity, Default::default())
        .expect("Unable to attach FlyControlTag to camera");
}
