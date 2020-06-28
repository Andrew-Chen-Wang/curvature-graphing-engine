use amethyst::{
    assets::{AssetStorage, Handle, Loader},
    controls::HideCursor,
    ecs::Entity,
    input::{is_close_requested, is_key_down, is_mouse_button_down},
    prelude::Trans,
    prelude::{World, WorldExt},
    renderer::{ImageFormat, Texture},
    ui::{Anchor, UiButtonBuilder, UiEventType, UiImage},
    winit::{VirtualKeyCode, MouseButton},
    GameData, SimpleState, SimpleTrans, StateData, StateEvent,
};

use crate::construct::sphere::initialize_sphere;
use crate::core::{camera::initialize_camera, headlamp::initialize_light};

#[derive(Default)]
pub struct GameState {
    headlamp: Option<Entity>,
    light_on: bool,
    paused: bool,

    // Buttons in the menu
    show_hide_bottom_menu: u32,
    menu_shown: bool,
    rotate_camera_button: u32,
    // Adds a sphere
    add_object: u32,
    // Morphing as in reshaping it
    morph_object: u32,
    // Make it larger or smaller via the center
    scale_object: u32,
    delete_object: u32,
    headlamp_button: u32,
}

// TODO In map.rs, we should add dir and coords

impl SimpleState for GameState {
    fn on_start(&mut self, state_data: StateData<'_, GameData<'_, '_>>) {
        self.paused = false;
        let StateData { world, .. } = state_data;
        initialize_camera(world);
        initialize_sphere(world); // TODO delete when you can initialize your own objects
        self.light_on = true;
        self.headlamp = Some(initialize_light(world));

        // Initialize menu
        // Manually supply id in case of collision
        self.rotate_camera_button = create_menu_button(world, "toolbar/3d_rotation.png", 100);
        self.headlamp_button = create_menu_button(world, "toolbar/headlamp.png", 101);
        self.show_hide_bottom_menu = create_menu_button(world, "toolbar/open_hide_menu.png", 102);
        self.menu_shown = true;
        self.add_object = create_menu_button(world, "toolbar/add_obj.png", 103);
        self.morph_object = create_menu_button(world, "toolbar/wrench_edit.png", 104);
        self.scale_object = create_menu_button(world, "toolbar/scale_obj.png", 105);
        self.delete_object = create_menu_button(world, "toolbar/trash.png", 106);

        // User can press the rotate button to rotate, so initially not in rotate.
        let mut hide_cursor = world.write_resource::<HideCursor>();
        hide_cursor.hide = false;
    }

    fn on_stop(&mut self, data: StateData<GameData>) {
        data.world.delete_all();
    }

    fn on_pause(&mut self, _data: StateData<'_, GameData<'_, '_>>) {
        self.paused = true;
    }

    fn on_resume(&mut self, _data: StateData<'_, GameData<'_, '_>>) {
        self.paused = false;
    }

    fn handle_event(
        &mut self,
        data: StateData<'_, GameData<'_, '_>>,
        event: StateEvent,
    ) -> SimpleTrans {
        let StateData { world, .. } = data;
        match &event {
            StateEvent::Window(event) => {
                if is_key_down(&event, VirtualKeyCode::Escape) {
                    // Press the rotate button to start camera rotation.
                    let mut hide_cursor = world.write_resource::<HideCursor>();
                    hide_cursor.hide = false;
                } else if is_mouse_button_down(&event, MouseButton::Right) {
                    // TODO Show info about and let edit of object.
                    // It can be like affecting its velocity and in which dir.
                    // Or copying the object
                    // Do nothing if nothing's there... or you can add object ig.
                } else if is_close_requested(&event) {
                    return Trans::Quit;
                }
                // TODO Add paused thing to get back to main menu... oh that too: main menu
                // TODO Allow point manipulation and central point scaling big/small.
            }
            StateEvent::Ui(ui_event) => {
                let target_id = ui_event.target.id();
                // TODO Add tooltips for menu buttons
                if target_id == self.rotate_camera_button
                    && ui_event.event_type == UiEventType::ClickStop
                {
                    // Press escape to get out of camera rotation.
                    let mut hide_cursor = world.write_resource::<HideCursor>();
                    hide_cursor.hide = true;
                // Also to get out of an edit mode or the like
                } else if target_id == self.headlamp_button {
                    // Turns on and off the user's headlamp
                    if self.light_on {
                        if let Some(headlamp) = self.headlamp {
                            world.delete_entity(headlamp);
                        }
                        self.headlamp = None;
                    } else {
                        self.headlamp = Some(initialize_light(world));
                    }
                } else if target_id == self.show_hide_bottom_menu {
                    // If menu hidden, show menu, and vice versa
                    if self.menu_shown {

                    } else {
                    }
                }
            }
            _ => {}
        }
        Trans::None
    }
}

fn create_menu_button(world: &mut World, path: &str, id: u32) -> u32 {
    let texture_handle: Handle<Texture> = {
        let loader = world.read_resource::<Loader>();
        let texture_assets = world.read_resource::<AssetStorage<Texture>>();
        loader.load(path, ImageFormat::default(), (), &texture_assets)
    };

    let (_button_id, button) = UiButtonBuilder::<(), u32>::default()
        .with_id(id)
        .with_position(0.0, -256.0)
        .with_size(45., 45.)
        .with_anchor(Anchor::TopMiddle)
        .with_image(UiImage::Texture(texture_handle))
        .build_from_world(&world);
    return button.image_entity.id();
}
