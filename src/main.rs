use amethyst::{
    core::transform::TransformBundle,
    prelude::*,
    renderer::{
        plugins::{RenderFlat3D, RenderToWindow, RenderSkybox},
        RenderingBundle,
        types::DefaultBackend,
    },
    utils::application_root_dir,
};
use amethyst::animation::VertexSkinningBundle;
use amethyst::assets::{AssetStorage, Handle, Loader, ProgressCounter};
use amethyst::controls::{FlyControlBundle, HideCursor, FlyControlTag};
use amethyst::core::Transform;
use amethyst::input::{is_key_down, is_mouse_button_down, StringBindings, VirtualKeyCode, InputBundle};
use amethyst::renderer::light::{DirectionalLight, Light};
use amethyst::renderer::palette::rgb::Rgb;
use amethyst::utils::auto_fov::AutoFovSystem;
use amethyst::winit::MouseButton;
use amethyst_gltf::{GltfSceneAsset, GltfSceneFormat, GltfSceneLoaderSystemDesc};
use amethyst::renderer::Camera;
use amethyst::core::math::Vector3;

pub struct LoadingState {
    /// Tracks loaded assets.
    progress_counter: ProgressCounter,
    /// Handle to the player texture.
    scene: Option<Handle<GltfSceneAsset>>,
}

impl SimpleState for LoadingState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let loader = data.world.read_resource::<Loader>();
        let scene = loader.load("models/SalleMachine.glb", GltfSceneFormat::default(), &mut self.progress_counter, &data.world.read_resource::<AssetStorage<GltfSceneAsset>>());
        self.scene = Some(scene);
    }

    fn update(&mut self, _data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        if self.progress_counter.is_complete() {
            Trans::Switch(Box::new(GameState {
                scene: self.scene
                    .take()
                    .expect(
                        "Expected `scene` to exist when \
                        `progress_counter` is complete."
                    ),
            }))
        } else {
            Trans::None
        }
    }
}

struct GameState {
    scene: Handle<GltfSceneAsset>,
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

        initialize_light(data.world);
        initialize_camera(data.world);
    }

    fn handle_event(&mut self, data: StateData<'_, GameData<'_, '_>>, event: StateEvent) -> SimpleTrans {
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
    let light: Light = DirectionalLight {
        intensity: 100.0,
        color: Rgb::new(1.0, 1.0, 1.0),
        ..DirectionalLight::default()
    }.into();

    /*let mut transform = Transform::default();
    transform.set_translation_xyz(5.0, 5.0, 20.0);*/

    world
        .create_entity()
        .with(light)
        // .with(transform)
        .build();
}

fn initialize_camera(world: &mut World) {
    let mut transform = Transform::default();
    transform.set_translation_xyz(0.0, 1.0, 0.0);

    world.create_entity()
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
        .with_bundle(
            FlyControlBundle::<StringBindings>::new(
                Some(String::from("move_x")),
                Some(String::from("move_y")),
                Some(String::from("move_z")),
            )
                .with_sensitivity(0.1, 0.1)
                .with_speed(5.),
        )?
        .with_bundle(TransformBundle::new().with_dep(&[
            "fly_movement",
        ]))?
        .with_bundle(
            InputBundle::<StringBindings>::new().with_bindings_from_file(&key_bindings_path)?,
        )?
        .with_bundle(VertexSkinningBundle::new().with_dep(&[
            "transform_system",
        ]))?
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(display_config_path)?
                        .with_clear([0.34, 0.36, 0.52, 1.0]),
                )
                .with_plugin(RenderFlat3D::default())
                .with_plugin(RenderSkybox::default()),
        )?;

    let mut game = Application::new(assets_dir, LoadingState {
        progress_counter: ProgressCounter::default(),
        scene: None,
    }, game_data)?;
    game.run();

    Ok(())
}
