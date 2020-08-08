use amethyst::{
    assets::Handle,
    audio::SourceHandle,
    controls::{FlyControlTag, HideCursor},
    core::{math::Vector3, Transform},
    ecs::{Entity, World},
    input::{is_key_down, is_mouse_button_down, VirtualKeyCode},
    prelude::*,
    renderer::{
        light::{Light, PointLight},
        palette::rgb::Rgb,
        Camera, SpriteRender,
    },
    ui::{Anchor, FontHandle, UiImage, UiTransform},
    winit::MouseButton,
};
use amethyst_gltf::GltfSceneAsset;

use crate::ui::{self, *};

pub struct GameState {
    pub scene: Handle<GltfSceneAsset>,
    pub screamer: SourceHandle,
    pub coming: SourceHandle,
    pub font: FontHandle,
    pub afit: SpriteRender,
    pub bashar: UiImage,
}

#[derive(Default)]
pub struct Afit {
    pub code_found: u8,
    pub unlocked_computers: Vec<i32>,
}

#[derive(Default)]
pub struct Screamer {
    pub bashar: Option<Entity>,
}

#[derive(Default)]
pub struct TimeToScreamer {
    pub at: f64,
    pub played: bool,
    pub last_displayed: f64,
    pub display: bool,
}

#[derive(Default)]
pub struct PlayerLight(pub Option<Entity>);

#[derive(Default)]
pub struct Sounds {
    pub screamer: Option<SourceHandle>,
    pub coming: Option<SourceHandle>,
}

#[derive(Default)]
pub struct PlayerHidden {
    pub hidden: bool,
    pub can_hide: bool,
    pub pressed: bool,
}

impl SimpleState for GameState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let mut transform = Transform::default();
        transform.set_scale(Vector3::new(2.0, 2.0, 2.0));

        data.world
            .create_entity()
            .with(self.scene.clone())
            .with(transform)
            .build();

        data.world.insert(Afit::default());
        data.world.insert(TimeToScreamer::default());
        data.world.insert(Reading(true));
        data.world.insert(Sounds {
            screamer: Some(self.screamer.clone()),
            coming: Some(self.coming.clone()),
        });

        let bashar = data
            .world
            .create_entity()
            .with(UiTransform::new(
                "bashar".to_string(),
                Anchor::Middle,
                Anchor::Middle,
                0.,
                0.,
                0.,
                0.,
                0.,
            ))
            .with(self.bashar.clone())
            .build();

        data.world.insert(Screamer {
            bashar: Some(bashar),
        });

        initialize_camera(data.world);

        let texts = ui::create_texts(data.world, &self.font);
        data.world.insert(texts);

        let entity = initialize_light(data.world);
        data.world.insert(entity);
    }

    fn handle_event(
        &mut self,
        data: StateData<'_, GameData<'_, '_>>,
        event: StateEvent,
    ) -> SimpleTrans {
        let StateData { world, .. } = data;
        if let StateEvent::Window(event) = &event {
            if is_key_down(&event, VirtualKeyCode::Escape) {
                let mut hide_cursor = world.write_resource::<HideCursor>();
                hide_cursor.hide = false;
            } else if is_mouse_button_down(&event, MouseButton::Left) {
                let mut hide_cursor = world.write_resource::<HideCursor>();
                hide_cursor.hide = true;
            }
        }
        Trans::None
    }
}

fn initialize_light(world: &mut World) -> PlayerLight {
    let light: Light = PointLight {
        color: Rgb::new(1.0, 1.0, 1.0),
        intensity: 2.0,
        smoothness: 1.0,
        ..PointLight::default()
    }
    .into();

    let mut transform = Transform::default();
    transform.set_translation_xyz(0.0, 1.5, 0.0);

    let entity = world
        .create_entity()
        .with(light)
        .with(transform)
        .with(FlyControlTag::default())
        .build();

    PlayerLight(Some(entity))
}

fn initialize_camera(world: &mut World) {
    let mut transform = Transform::default();
    transform.set_translation_xyz(0.0, 1.2, 0.0);

    world
        .create_entity()
        .with(Camera::standard_3d(1024.0, 768.0))
        .with(transform)
        .with(FlyControlTag::default())
        .build();
}
