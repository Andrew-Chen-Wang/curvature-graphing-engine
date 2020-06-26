use crate::construct::sphere::initialize_sphere;
use crate::core::{
    camera::initialize_camera,
    light::initialize_light
};

use amethyst::{
    SimpleState, StateData, GameData,
    StateEvent, SimpleTrans
};

pub struct GameState;
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

    fn handle_event(
        &mut self,
        data: StateData<'_, GameData<'_, '_>>,
        event: StateEvent,
    ) -> SimpleTrans {
        use amethyst::{
            input::is_key_down,
            controls::HideCursor,
            winit::VirtualKeyCode,
            prelude::{Trans, WorldExt}
        };
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