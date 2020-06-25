use amethyst::utils::application_root_dir;
use amethyst::{
    SimpleState, GameDataBuilder, Application,
    StateData, GameData
};
use amethyst::renderer::{
    plugins::{RenderPbr3D, RenderToWindow},
    types::DefaultBackend,
    RenderingBundle, Camera,
    MaterialDefaults, Material,
    Mesh
};
use amethyst::window::DisplayConfig;
use amethyst::prelude::{World, Builder, WorldExt};
use amethyst::assets::AssetLoaderSystemData;
use amethyst::renderer::rendy::mesh::{
    Position, Normal, Tangent, TexCoord
};
use amethyst::renderer::shape::Shape;
use amethyst::core::{Transform, TransformBundle};
use amethyst::renderer::light::{Light, PointLight};
use amethyst::renderer::palette::rgb::Rgb;

struct GameState;
impl SimpleState for GameState {
    fn on_start(&mut self, state_data: StateData<'_, GameData<'_, '_>>) {
        initialize_camera(state_data.world);
        initialize_sphere(state_data.world);
        initialize_light(state_data.world);
    }
}

fn main() -> amethyst::Result<()> {
    // Set up the Amethyst logger
    amethyst::start_logger(Default::default());

    // Set up the assets directory (PathBuf)
    let app_root = application_root_dir()?;
    let assets_dir = app_root.join("assets");

    // Set up the display configuration
    let display_config = DisplayConfig {
        title: "Natural Gravity Engine".to_string(),
        dimensions: Some((800, 600)),  // 4:3
        ..Default::default()
    };

    // Set up the GameDataBuilder
    let game_data = GameDataBuilder::default()
        .with_bundle(TransformBundle::new())?
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                // The RenderToWindow plugin provides all the scaffolding for opening a window and drawing on it
                .with_plugin(
                    RenderToWindow::from_config(display_config)
                        .with_clear([0.529, 0.808, 0.98, 1.0]),
                )
                // RenderFlat2D plugin is used to render entities with a `SpriteRender` component.
                .with_plugin(RenderPbr3D::default()),
        )?;

    // Run the game!
    let mut game = Application::new(
        assets_dir,
        GameState,
        game_data
    )?;
    game.run();

    Ok(())
}

fn initialize_camera(world: &mut World) {
    let mut transform = Transform::default();
    transform.set_translation_xyz(0.0, 0.0, 10.0);

    world.create_entity()
        .with(Camera::standard_3d(1024.0, 768.0))
        .with(transform)
        .build();
}

fn initialize_sphere(world: &mut World) {
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

    world.create_entity()
        .with(mesh)
        .with(material)
        .with(transform)
        .build();
}

fn initialize_light(world: &mut World) {
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
        .build();
}
