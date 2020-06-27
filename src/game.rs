use amethyst::{
    SimpleState, StateData, GameData,
    StateEvent, SimpleTrans,
    ecs::prelude::Entity,
    prelude::WorldExt,
};

use crate::construct::sphere::initialize_sphere;
use crate::core::{
    camera::initialize_camera,
    light::initialize_light
};

#[derive(Default)]
pub struct GameState {
    paused: bool,

    // Buttons in the menu
    rotate_button: Option<Entity>,
}

impl SimpleState for GameState {
    fn on_start(&mut self, state_data: StateData<'_, GameData<'_, '_>>) {
        initialize_camera(state_data.world);
        initialize_sphere(state_data.world);
        initialize_light(state_data.world);

        // User can press the rotate button to rotate.
        // TODO Use CMD/Ctrl and click to rotate instead of this.
        // let StateData { world, .. } = state_data;
        // let mut hide_cursor = world.write_resource::<HideCursor>();
        // hide_cursor.hide = false;
    }

    fn on_stop(&mut self, data: StateData<GameData>) {
        data.world.delete_all();
        self.rotate_button = None;
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
        use amethyst::{
            input::{is_close_requested, is_key_down},
            controls::HideCursor,
            winit::VirtualKeyCode,
            prelude::Trans,
        };
        let StateData {world, ..} = data;
        if let StateEvent::Window(event) = &event {
            if is_key_down(&event, VirtualKeyCode::Escape) {
                let mut hide_cursor = world.write_resource::<HideCursor>();
                hide_cursor.hide = false;
            } else if is_close_requested(&event) {

            }
        }
        Trans::None
    }
}