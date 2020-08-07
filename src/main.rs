use amethyst::{
    animation::VertexSkinningBundle,
    assets::{AssetStorage, Handle, Loader, ProgressCounter},
    audio::{output::Output, AudioBundle, Mp3Format, Source, SourceHandle},
    controls::{ArcBallControlBundle, FlyControlTag, HideCursor},
    core::{math::Vector3, Transform, TransformBundle},
    ecs::{Entity, Read, World},
    input::{is_key_down, is_mouse_button_down, InputBundle, StringBindings, VirtualKeyCode},
    prelude::*,
    renderer::{
        light::{Light, PointLight},
        palette::rgb::Rgb,
        plugins::{RenderShaded3D, RenderSkybox, RenderToWindow},
        types::DefaultBackend,
        Camera, ImageFormat, RenderingBundle, SpriteRender, SpriteSheet, SpriteSheetFormat,
        Texture,
    },
    ui::{Anchor, FontHandle, RenderUi, TtfFormat, UiBundle, UiText, UiTransform},
    utils::{application_root_dir, auto_fov::AutoFovSystem},
    winit::MouseButton,
};

use amethyst_gltf::{GltfSceneAsset, GltfSceneFormat, GltfSceneLoaderSystemDesc};

use crate::hide::HidingSystem;
use crate::movement::RuptureMovementSystem;
use crate::screamer::ScreamerSystem;

mod hide;
mod movement;
mod screamer;
mod space;

const MAX_CODE: u8 = 10;

pub struct LoadingState {
    progress_counter: ProgressCounter,
    scene: Option<Handle<GltfSceneAsset>>,
    screamer: Option<SourceHandle>,
    coming: Option<SourceHandle>,
    font: Option<FontHandle>,
    afit: Option<Handle<SpriteSheet>>,
}

impl LoadingState {
    fn load_sprite_sheet(&mut self, data: &StateData<'_, GameData<'_, '_>>) -> Handle<SpriteSheet> {
        let texture_handle = {
            let loader = data.world.read_resource::<Loader>();
            let texture_storage = data.world.read_resource::<AssetStorage<Texture>>();
            loader.load(
                "textures/afit.png",
                ImageFormat::default(),
                &mut self.progress_counter,
                &texture_storage,
            )
        };

        let loader = data.world.read_resource::<Loader>();
        let sprite_sheet_store = data.world.read_resource::<AssetStorage<SpriteSheet>>();
        loader.load(
            "textures/afit_spritesheet.ron", // Here we load the associated ron file
            SpriteSheetFormat(texture_handle),
            &mut self.progress_counter,
            &sprite_sheet_store,
        )
    }
}

impl SimpleState for LoadingState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let loader = data.world.read_resource::<Loader>();

        self.scene = Some(loader.load(
            "models/SalleMachine.glb",
            GltfSceneFormat::default(),
            &mut self.progress_counter,
            &data.world.read_resource(),
        ));
        self.screamer = Some(loader.load(
            "sounds/screamer.mp3",
            Mp3Format,
            &mut self.progress_counter,
            &data.world.read_resource(),
        ));
        self.coming = Some(loader.load(
            "sounds/coming.mp3",
            Mp3Format,
            &mut self.progress_counter,
            &data.world.read_resource(),
        ));
        self.font = Some(loader.load(
            "fonts/crow.ttf",
            TtfFormat,
            &mut self.progress_counter,
            &data.world.read_resource(),
        ));
        self.afit = Some(self.load_sprite_sheet(&data));
    }

    fn update(&mut self, _data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        if self.progress_counter.is_complete() {
            Trans::Switch(Box::new(GameState {
                scene: self.scene.take().expect(
                    "Expected `scene` to exist when \
                        `progress_counter` is complete.",
                ),
                screamer: self.screamer.take().expect("iléou le screamer.mp3 :("),
                coming: self.coming.take().expect("iléou le coming.mp3 :c"),
                font: self.font.take().expect("iléou le crow.ttf D:"),
                afit: SpriteRender {
                    sprite_sheet: self.afit.take().expect("iléou le afit.png"),
                    sprite_number: 0
                },
            }))
        } else {
            Trans::None
        }
    }
}

struct GameState {
    scene: Handle<GltfSceneAsset>,
    screamer: SourceHandle,
    coming: SourceHandle,
    font: FontHandle,
    afit: SpriteRender,
}

#[derive(Default)]
pub struct Texts {
    hide: Option<Entity>,
    code: Option<Entity>,
}

#[derive(Default)]
pub struct CodeFound(u8);

#[derive(Default)]
pub struct TimeToScreamer {
    at: f64,
    played: bool,
}

#[derive(Default)]
pub struct PlayerLight(Option<Entity>);

#[derive(Default)]
pub struct Sounds {
    screamer: Option<SourceHandle>,
    coming: Option<SourceHandle>,
}

#[derive(Default)]
pub struct PlayerHidden {
    hidden: bool,
    can_hide: bool,
    pressed: bool,
}

impl SimpleState for GameState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let mut transform = Transform::default();
        transform.set_scale(Vector3::new(2.0, 2.0, 2.0));
        // Create the scene entity.
        data.world
            .create_entity()
            // Use the scene handle as a component
            .with(self.scene.clone())
            .with(transform)
            .build();

        data.world.insert(CodeFound::default());
        data.world.insert(TimeToScreamer::default());
        data.world.insert(Sounds {
            screamer: Some(self.screamer.clone()),
            coming: Some(self.coming.clone()),
        });

        let hide = data
            .world
            .create_entity()
            .with(UiTransform::new(
                "hide".to_string(),
                Anchor::BottomRight,
                Anchor::BottomRight,
                -50.,
                50.,
                1.,
                650.,
                50.,
            ))
            .with(UiText::new(
                self.font.clone(),
                String::new(),
                [1., 1., 1., 1.],
                40.,
            ))
            .build();

        let code = data
            .world
            .create_entity()
            .with(UiTransform::new(
                "code".to_string(),
                Anchor::TopLeft,
                Anchor::TopLeft,
                10.,
                -50.,
                1.,
                500.,
                50.,
            ))
            .with(UiText::new(
                self.font.clone(),
                "Tests passes a 0%".to_string(),
                [1., 1., 1., 1.],
                60.,
            ))
            .build();

        data.world.insert(Texts {
            hide: Some(hide),
            code: Some(code),
        });

        initialize_camera(data.world);

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

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;

    let assets_dir = app_root.join("assets");
    let config_dir = app_root.join("config");
    let display_config_path = config_dir.join("display.ron");
    let key_bindings_path = config_dir.join("input.ron");

    let game_data = GameDataBuilder::default()
        .with(AutoFovSystem::default(), "auto_fov", &[])
        .with_system_desc(
            GltfSceneLoaderSystemDesc::default(),
            "gltf_loader",
            &[], // This is important so that entity instantiation is performed in a single frame.
        )
        .with(
            RuptureMovementSystem::new(
                2.5,
                Some(String::from("move_x")),
                Some(String::from("move_z")),
            ),
            "rupture_movement",
            &[],
        )
        .with(ScreamerSystem, "screamer", &[])
        .with(HidingSystem, "hiding", &[])
        .with_bundle(ArcBallControlBundle::<StringBindings>::new().with_sensitivity(0.1, 0.1))?
        .with_bundle(TransformBundle::new().with_dep(&["arc_ball_rotation"]))?
        .with_bundle(
            InputBundle::<StringBindings>::new().with_bindings_from_file(&key_bindings_path)?,
        )?
        .with_bundle(VertexSkinningBundle::new().with_dep(&["transform_system"]))?
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(display_config_path)?
                        .with_clear([0.34, 0.36, 0.52, 1.0]),
                )
                .with_plugin(RenderShaded3D::default())
                .with_plugin(RenderUi::default())
                .with_plugin(RenderSkybox::default()),
        )?
        .with_bundle(AudioBundle::default())?
        .with_bundle(UiBundle::<StringBindings>::new())?;

    let mut game = Application::new(
        assets_dir,
        LoadingState {
            progress_counter: ProgressCounter::default(),
            scene: None,
            screamer: None,
            coming: None,
            font: None,
            afit: None,
        },
        game_data,
    )?;
    game.run();

    Ok(())
}

fn play<'s>(
    storage: &Read<'s, AssetStorage<Source>>,
    handle: &Option<SourceHandle>,
    output: &Option<Read<'s, Output>>,
    volume: f32,
) {
    if let Some(output) = output {
        if let Some(handle) = handle {
            if let Some(sound) = storage.get(handle) {
                output.play_once(sound, volume);
            }
        }
    }
}
