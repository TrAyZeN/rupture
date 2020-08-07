use amethyst::animation::VertexSkinningBundle;
use amethyst::assets::{AssetStorage, Handle, Loader, ProgressCounter};
use amethyst::controls::{ArcBallControlBundle, FlyControlTag, HideCursor};
use amethyst::core::math::{convert, Unit, Vector3};
use amethyst::core::timing::Time;
use amethyst::core::Transform;
use amethyst::input::{
    get_input_axis_simple, is_key_down, is_mouse_button_down, BindingTypes, InputBundle,
    InputHandler, StringBindings, VirtualKeyCode,
};
use amethyst::renderer::light::{Light, PointLight};
use amethyst::renderer::palette::rgb::Rgb;
use amethyst::renderer::Camera;
use amethyst::utils::auto_fov::AutoFovSystem;
use amethyst::winit::MouseButton;
use amethyst::{
    audio::{output::Output, AudioBundle, Mp3Format, Source, SourceHandle},
    core::transform::TransformBundle,
    derive::SystemDesc,
    ecs::{Join, Read, ReadStorage, System, SystemData, World, Write, WriteStorage},
    prelude::*,
    renderer::{
        plugins::{RenderShaded3D, RenderSkybox, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    utils::application_root_dir,
};
use amethyst_gltf::{GltfSceneAsset, GltfSceneFormat, GltfSceneLoaderSystemDesc};

pub struct LoadingState {
    progress_counter: ProgressCounter,
    scene: Option<Handle<GltfSceneAsset>>,
    screamer: Option<SourceHandle>,
    coming: Option<SourceHandle>,
}

impl SimpleState for LoadingState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let loader = data.world.read_resource::<Loader>();

        self.scene = Some(loader.load(
            "models/SalleMachine.glb",
            GltfSceneFormat::default(),
            &mut self.progress_counter,
            &data.world.read_resource::<AssetStorage<GltfSceneAsset>>(),
        ));
        self.screamer = Some(loader.load(
            "sounds/screamer.mp3",
            Mp3Format,
            &mut self.progress_counter,
            &data.world.read_resource::<AssetStorage<Source>>(),
        ));
        self.coming = Some(loader.load(
            "sounds/coming.mp3",
            Mp3Format,
            &mut self.progress_counter,
            &data.world.read_resource::<AssetStorage<Source>>(),
        ));
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
}

#[derive(Default)]
pub struct CodeFound(u8);

#[derive(Default)]
pub struct TimeToScreamer {
    at: f64,
    played: bool,
}

#[derive(Default)]
pub struct Sounds {
    screamer: Option<SourceHandle>,
    coming: Option<SourceHandle>,
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

        initialize_camera(data.world);
        initialize_light(data.world);
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

fn initialize_light(world: &mut World) {
    let light: Light = PointLight {
        color: Rgb::new(1.0, 1.0, 1.0),
        intensity: 2.0,
        smoothness: 1.0,
        ..PointLight::default()
    }
    .into();

    let mut transform = Transform::default();
    transform.set_translation_xyz(0.0, 1.5, 0.0);

    world
        .create_entity()
        .with(light)
        .with(transform)
        .with(FlyControlTag::default())
        .build();
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
            RuptureMovementSystem::<StringBindings>::new(
                5.,
                Some(String::from("move_x")),
                Some(String::from("move_z")),
            ),
            "rupture_movement",
            &[],
        )
        .with(ScreamerSystem, "screamer", &[])
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
                .with_plugin(RenderSkybox::default()),
        )?
        .with_bundle(AudioBundle::default())?;

    let mut game = Application::new(
        assets_dir,
        LoadingState {
            progress_counter: ProgressCounter::default(),
            scene: None,
            screamer: None,
            coming: None,
        },
        game_data,
    )?;
    game.run();

    Ok(())
}

#[derive(Debug, SystemDesc)]
#[system_desc(name(RuptureMovementSystemDesc))]
pub struct RuptureMovementSystem<T>
where
    T: BindingTypes,
{
    /// The movement speed of the movement in units per second.
    speed: f32,
    /// The name of the input axis to locally move in the x coordinates.
    right_input_axis: Option<T::Axis>,
    /// The name of the input axis to locally move in the z coordinates.
    forward_input_axis: Option<T::Axis>,
}

impl<T: BindingTypes> RuptureMovementSystem<T> {
    /// Builds a new `FlyMovementSystem` using the provided speeds and axis controls.
    pub fn new(
        speed: f32,
        right_input_axis: Option<T::Axis>,
        forward_input_axis: Option<T::Axis>,
    ) -> Self {
        RuptureMovementSystem {
            speed,
            right_input_axis,
            forward_input_axis,
        }
    }
}

impl<'a, T: BindingTypes> System<'a> for RuptureMovementSystem<T> {
    type SystemData = (
        Read<'a, Time>,
        WriteStorage<'a, Transform>,
        Read<'a, InputHandler<T>>,
        ReadStorage<'a, FlyControlTag>,
    );

    fn run(&mut self, (time, mut transform, input, tag): Self::SystemData) {
        let x = get_input_axis_simple(&self.right_input_axis, &input);
        let z = get_input_axis_simple(&self.forward_input_axis, &input);

        if let Some(dir) = Unit::try_new(Vector3::new(x, 0.0, z), convert(1.0e-6)) {
            for (transform, _) in (&mut transform, &tag).join() {
                let delta_sec = time.delta_seconds();
                let old = transform.translation().clone();

                transform.append_translation_along(dir, delta_sec * self.speed);

                let current = transform.translation().clone();
                if !is_in_bound(current.x, old.z) {
                    transform.set_translation_x(old.x);
                }
                if !is_in_bound(old.x, current.z) {
                    transform.set_translation_z(old.z);
                }

                transform.set_translation_y(old.y);

                println!("X: {}, Z: {}", current.x, current.z);
            }
        }
    }
}

#[derive(Debug, SystemDesc)]
#[system_desc(name(ScreamerSystemDesc))]
pub struct ScreamerSystem;

impl<'s> System<'s> for ScreamerSystem {
    type SystemData = (
        Read<'s, Time>,
        Read<'s, AssetStorage<Source>>,
        Read<'s, Sounds>,
        Option<Read<'s, Output>>,
        Read<'s, CodeFound>,
        Write<'s, TimeToScreamer>,
    );

    fn run(&mut self, (time, storage, sound, output, found, mut since): Self::SystemData) {
        if since.at == 0.0 {
            since.at = time.absolute_time_seconds() + 15.0 + rand::random::<f64>() * 10.0;
        }

        if time.absolute_time_seconds() > since.at - (1.0 + (4.0 / (found.0 as f64 + 1.0)))
            && !since.played
        {
            play(&storage, &sound.coming, &output, 0.65);
            since.played = true;
        }

        if time.absolute_time_seconds() > since.at {
            play(&storage, &sound.screamer, &output, 0.9);

            since.played = false;
            since.at = time.absolute_time_seconds()
                + 5.0
                + (10.0 / (found.0 as f64 + 1.0))
                + rand::random::<f64>() * 10.0;
        }
    }
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

// Oui c'est dégueulasse, mais est-ce qu'il y a vraiment une autre solution xd
fn is_in_bound(x: f32, z: f32) -> bool {
    (x > -25.0 && z > -2.65 && x < 0.65 && z < 0.65) // Couloir
        || is_in_room(x, z) // Salle droite
        || is_in_room(x + 14.0, z) // Salle gauche
}

fn is_in_room(x: f32, z: f32) -> bool {
    (x > -2.35 && z > -3.35 && x < -1.55 && z < -2.65) // Porte droite
        || (x > -10.55 && z > -3.35 && x < -9.55 && z < -2.65) // Porte gauche 
        || (x > -12.75 && z > -7.0 && x < 0.55 && z < -3.35) // Entrée salle
        || (x > -0.85 && z > -22.5 && x < 0.55 && z < -7.0) // Inter droit
        || (x > -8.8 && z > -22.5 && x < -3.1 && z < -7.0) // Inter centre
        || (x > -12.75 && z > -22.5 && x < -11.25 && z < -7.0) // Inter gauche
}
