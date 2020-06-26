mod core;
mod construct;

use amethyst::{
    utils::application_root_dir,
    SimpleState, GameDataBuilder, Application,
    StateData, GameData,
    renderer::{
        plugins::{RenderPbr3D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle
    },
    input::{is_key_down, InputBundle, StringBindings},
    window::DisplayConfig,
    core::TransformBundle,
    prelude::*,
    controls::{FlyControlBundle, HideCursor},
    winit::VirtualKeyCode
};

struct GameState;
impl SimpleState for GameState {
    fn on_start(&mut self, state_data: StateData<'_, GameData<'_, '_>>) {
        core::camera::initialize_camera(state_data.world);
        construct::sphere::initialize_sphere(state_data.world);
        core::light::initialize_light(state_data.world);

        // User can press the rotate button to rotate.
        // TODO Use CMD/Ctrl and click to rotate instead of this.
        // let StateData { world, .. } = state_data;
        // let mut hide_cursor = world.write_resource::<HideCursor>();
        // hide_cursor.hide = false;
    }

    fn handle_event(
        &mut self,
        data: StateData<'_, GameData<'_, '_>>,
        event: StateEvent,
    ) -> SimpleTrans {
        let StateData {world, ..} = data;
        if let StateEvent::Window(event) = &event {
            if is_key_down(&event, VirtualKeyCode::Escape) {
                let mut hide_cursor = world.write_resource::<HideCursor>();
                hide_cursor.hide = false;
            }
        }
        Trans::None
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

    let key_bindings_path = app_root.join("config/input.ron");

    // Set up the GameDataBuilder
    let game_data = GameDataBuilder::default()
        .with_bundle(
            FlyControlBundle::<StringBindings>::new(
                Some(String::from("move_x")),
                Some(String::from("move_y")),
                Some(String::from("move_z")),
            ).with_sensitivity(0.1, 0.1),
        )?
        .with_bundle(
            TransformBundle::new().with_dep(&["fly_movement"])
        )?
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                // The RenderToWindow plugin provides all the scaffolding for opening a window and drawing on it
                .with_plugin(
                    RenderToWindow::from_config(display_config)
                        .with_clear([0.529, 0.808, 0.98, 1.0]),
                )
                .with_plugin(RenderPbr3D::default()),
        )?
        .with_bundle(
            InputBundle::<StringBindings>::new().with_bindings_from_file(&key_bindings_path)?,
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
