use amethyst::{
    assets::AssetLoaderSystemData,
    core::Transform,
    ecs::Entity,
    prelude::{Builder, World, WorldExt},
    renderer::{
        rendy::mesh::{Normal, Position, Tangent, TexCoord},
        shape::Shape,
        Material, MaterialDefaults, Mesh,
    },
};

pub fn initialize_sphere(world: &mut World) -> Entity {
    let mesh = world.exec(|loader: AssetLoaderSystemData<'_, Mesh>| {
        loader.load_from_data(
            Shape::Sphere(100, 100)
                .generate::<(Vec<Position>, Vec<Normal>, Vec<Tangent>, Vec<TexCoord>)>(None)
                .into(),
            (),
        )
    });

    let material_defaults = world.read_resource::<MaterialDefaults>().0.clone();
    let material = world.exec(|loader: AssetLoaderSystemData<'_, Material>| {
        loader.load_from_data(
            Material {
                ..material_defaults
            },
            (),
        )
    });

    let mut transform = Transform::default();
    transform.set_translation_xyz(0.0, 0.0, 0.0);

    world
        .create_entity()
        .with(mesh)
        .with(material)
        .with(transform)
        .build()
}
