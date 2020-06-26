use amethyst::{
    renderer::Camera,
    prelude::{World, Builder, WorldExt},
    core::Transform,
    controls::FlyControlTag
};

pub fn initialize_camera(world: &mut World) {
    let mut transform = Transform::default();
    transform.set_translation_xyz(0.0, 0.0, 10.0);

    let entity = world.create_entity()
        .with(Camera::standard_3d(800.0, 600.0))
        .with(transform)
        .build();
    world.write_storage::<FlyControlTag>().insert(entity, Default::default());
}