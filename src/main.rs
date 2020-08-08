use amethyst::{
    animation::VertexSkinningBundle,
    assets::{AssetStorage, ProgressCounter},
    audio::{output::Output, AudioBundle, Source, SourceHandle},
    controls::ArcBallControlBundle,
    core::TransformBundle,
    ecs::Read,
    input::{InputBundle, StringBindings},
    prelude::*,
    renderer::{
        plugins::{RenderShaded3D, RenderSkybox, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    ui::{RenderUi, UiBundle},
    utils::{application_root_dir, auto_fov::AutoFovSystem},
};
use amethyst_gltf::GltfSceneLoaderSystemDesc;

mod states;
mod systems;
mod ui;
mod space;

use states::loading::LoadingState;
use systems::{
    hide::HidingSystem,
    movement::RuptureMovementSystem,
    screamer::ScreamerSystem,
    use_system::UseSystem,
};
use ui::TextSystem;

const MAX_CODE: u8 = 10;
pub const COMPUTER_NUMBER: i32 = 32;

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
        .with(TextSystem, "text", &[])
        .with(UseSystem, "use", &[])
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
            bashar: None,
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
