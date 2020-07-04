use amethyst::{
    assets::{AssetStorage, Handle, Loader},
    controls::HideCursor,
    core::Hidden,
    ecs::Entity,
    input::{is_close_requested, is_key_down, is_mouse_button_down},
    prelude::{Builder, Trans, World, WorldExt},
    renderer::{ImageFormat, Texture},
    ui::{Anchor, Interactable, UiButtonBuilder, UiEventType, UiImage, UiTransform},
    winit::{MouseButton, VirtualKeyCode},
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
    background_image: Option<Entity>,
    show_hide_bottom_menu: Option<Entity>,
    menu_shown: bool,
    rotate_camera_button: Option<Entity>,
    // Adds a sphere
    add_object: Option<Entity>,
    // Morphing as in reshaping it
    morph_object: Option<Entity>,
    // Make it larger or smaller via the center
    scale_object: Option<Entity>,
    delete_object: Option<Entity>,
    headlamp_button: Option<Entity>,
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
        self.create_menu(world);

        // User can press the rotate button to rotate,
        // so initially camera not in rotate mode.
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
                let target_id = ui_event.target;
                if ui_event.event_type == UiEventType::ClickStop {
                    // TODO Add tooltips for menu buttons
                    if target_id == self.rotate_camera_button.unwrap() {
                        // Press escape to get out of camera rotation.
                        let mut hide_cursor = world.write_resource::<HideCursor>();
                        hide_cursor.hide = true;
                    // Also to get out of an edit mode or the like
                    } else if target_id == self.headlamp_button.unwrap() {
                        // Turns on and off the user's headlamp
                        if self.light_on {
                            if let Some(headlamp) = self.headlamp {
                                let _ = world.delete_entity(headlamp);
                            }
                            self.headlamp = None;
                        } else {
                            self.headlamp = Some(initialize_light(world));
                        }
                        self.light_on = !self.light_on;
                    } else if target_id == self.show_hide_bottom_menu.unwrap() {
                        // If menu hidden, show menu, and vice versa
                        if self.menu_shown {
                            let mut hidden = world.write_storage::<Hidden>();
                            let _ = hidden.insert(self.rotate_camera_button.unwrap(), Hidden);
                            let _ = hidden.insert(self.add_object.unwrap(), Hidden);
                            let _ = hidden.insert(self.morph_object.unwrap(), Hidden);
                            let _ = hidden.insert(self.scale_object.unwrap(), Hidden);
                            let _ = hidden.insert(self.delete_object.unwrap(), Hidden);
                            let _ = hidden.insert(self.headlamp_button.unwrap(), Hidden);
                            // show_hide_bottom_menu: Entity
                        } else {
                            let mut hidden = world.write_storage::<Hidden>();
                            let _ = hidden.remove(self.rotate_camera_button.unwrap());
                            let _ = hidden.remove(self.add_object.unwrap());
                            let _ = hidden.remove(self.morph_object.unwrap());
                            let _ = hidden.remove(self.scale_object.unwrap());
                            let _ = hidden.remove(self.delete_object.unwrap());
                            let _ = hidden.remove(self.headlamp_button.unwrap());
                        }
                        self.menu_shown = !self.menu_shown;
                    }
                }
            }
            _ => {}
        }
        Trans::None
    }
}

impl GameState {
    fn create_menu(&mut self, world: &mut World) {
        self.menu_shown = true;
        let menu_transform = UiTransform::new(
            String::from("bottom_menu"),
            Anchor::BottomMiddle,
            Anchor::Middle,
            0.,
            0.,
            0.,
            700.,
            300.,
        );
        let image = UiImage::SolidColor([0., 1., 0., 1.]);
        let bi = world
            .create_entity()
            .with(menu_transform)
            .with(image)
            .with(Interactable)
            .build();
        self.background_image = Some(bi);

        // Manually supply id in case of collision
        self.rotate_camera_button =
            self.create_menu_button(world, "toolbar/3d_rotation.png", -200., 100.);
        self.headlamp_button = self.create_menu_button(world, "toolbar/headlamp.png", -100., 100.);
        self.show_hide_bottom_menu =
            self.create_menu_button(world, "toolbar/open_hide_menu.png", 200., 100.);
        self.add_object = self.create_menu_button(world, "toolbar/add_obj.png", 50., 50.);
        self.morph_object = self.create_menu_button(world, "toolbar/wrench_edit.png", 100., 50.);
        self.delete_object = self.create_menu_button(world, "toolbar/trash.png", 50., 100.);
        self.scale_object = self.create_menu_button(world, "toolbar/scale_obj.png", 100., 100.);
    }

    fn create_menu_button(
        &mut self,
        world: &mut World,
        path: &str,
        position_x: f32,
        position_y: f32,
    ) -> Option<Entity> {
        let texture_handle: Handle<Texture> = {
            let loader = world.read_resource::<Loader>();
            let texture_assets = world.read_resource::<AssetStorage<Texture>>();
            loader.load(path, ImageFormat::default(), (), &texture_assets)
        };

        let (_button_id, button) = UiButtonBuilder::<(), u32>::default()
            .with_position(position_x, position_y)
            .with_size(45., 45.)
            .with_anchor(Anchor::BottomMiddle)
            .with_image(UiImage::Texture(texture_handle))
            .build_from_world(&world);
        Some(button.image_entity)
    }
}
