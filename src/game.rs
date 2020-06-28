use amethyst::{
    SimpleState, StateData, GameData,
    StateEvent, SimpleTrans,
    prelude::{World, WorldExt},
    controls::HideCursor,
    ui::{Anchor, UiButtonBuilder, UiImage, UiEventType},
    input::{is_close_requested, is_key_down},
    winit::VirtualKeyCode,
    prelude::Trans,
};

use crate::construct::sphere::initialize_sphere;
use crate::core::{
    camera::initialize_camera,
    headlamp::initialize_light
};

#[derive(Default)]
pub struct GameState {
    paused: bool,

    // Buttons in the menu
    show_bottom_menu: u32,
    hide_bottom_menu: u32,
    rotate_camera_button: u32,
    headlamp_button: u32,
    light_on: bool,
}

const OPEN_MENU_TEXT: &str = "↑";
const CLOSE_MENU_TEXT: &str = "↓";

const ROTATE_TEXT: &str = "↺";
const LIGHT_BULB_TEXT: &str = "💡";

impl SimpleState for GameState {
    fn on_start(&mut self, state_data: StateData<'_, GameData<'_, '_>>) {
        let StateData { world, .. } = state_data;
        initialize_camera(world);
        initialize_sphere(world);
        initialize_light(world);

        // Initialize menu
        // Manually supply id in case of collision
        self.rotate_camera_button = create_menu_button(world, ROTATE_TEXT, 123);

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
        let StateData {world, ..} = data;
        match &event {
            StateEvent::Window(event) => {
                if is_key_down(&event, VirtualKeyCode::Escape) {
                    // Press the rotate button to start camera rotation.
                    let mut hide_cursor = world.write_resource::<HideCursor>();
                    hide_cursor.hide = false;
                } else if is_close_requested(&event) {
                    return Trans::Quit
                }
                // TODO Allow point manipulation and central point scaling big/small.
            }
            StateEvent::Ui(ui_event) => {
                let target_id = ui_event.target.id();
                // TODO Add tooltips for menu buttons
                if target_id == self.rotate_camera_button && ui_event.event_type == UiEventType::ClickStop {
                    // Press escape to get out of camera rotation.
                    let mut hide_cursor = world.write_resource::<HideCursor>();
                    hide_cursor.hide = true;
                } else if target_id == self.headlamp_button {
                    // Turns on and off the user's headlamp
                } else if target_id == self.show_bottom_menu {
                    // If menu hidden, show menu
                } else if target_id == self.hide_bottom_menu {
                    // If menu visible, hide menu
                }
            }
            _ => {}
        }
        Trans::None
    }
}

fn create_menu_button(world: &mut World, text: &str, id: u32) -> u32 {
    let (_button_id, button) = UiButtonBuilder::<(), u32>::new(text)
        .with_id(id)
        .with_font_size(32.0)
        .with_position(0.0, -256.0)
        .with_size(1000., 200.)
        .with_anchor(Anchor::TopMiddle)
        .with_image(UiImage::SolidColor([0.8, 0.6, 0.3, 1.0]))
        .with_hover_image(UiImage::SolidColor([0.1, 0.1, 0.1, 0.5]))
        .build_from_world(&world);
    return button.image_entity.id()
}