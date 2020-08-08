use amethyst::{
    assets::{AssetStorage, Handle, Loader, ProgressCounter},
    audio::{Mp3Format, SourceHandle},
    ecs::World,
    prelude::*,
    renderer::{ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
    ui::{FontHandle, TtfFormat, UiImage},
};
use amethyst_gltf::{GltfSceneAsset, GltfSceneFormat};

use super::game::GameState;

pub struct LoadingState {
    pub progress_counter: ProgressCounter,
    pub scene: Option<Handle<GltfSceneAsset>>,
    pub screamer: Option<SourceHandle>,
    pub coming: Option<SourceHandle>,
    pub font: Option<FontHandle>,
    pub afit: Option<Handle<SpriteSheet>>,
    pub bashar: Option<Handle<Texture>>,
}

impl LoadingState {
    fn load_sprite_sheet(&mut self, world: &World) -> Handle<SpriteSheet> {
        let texture_handle = {
            let loader = world.read_resource::<Loader>();
            let texture_storage = world.read_resource::<AssetStorage<Texture>>();
            loader.load(
                "textures/afit.png",
                ImageFormat::default(),
                &mut self.progress_counter,
                &texture_storage,
            )
        };

        let loader = world.read_resource::<Loader>();
        let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();

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
        self.afit = Some(self.load_sprite_sheet(&data.world));
        self.bashar = Some(loader.load(
            "textures/bashar.jpeg",
            ImageFormat::default(),
            &mut self.progress_counter,
            &data.world.read_resource(),
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
                font: self.font.take().expect("iléou le crow.ttf D:"),
                afit: SpriteRender {
                    sprite_sheet: self.afit.take().expect("iléou le afit.png"),
                    sprite_number: 0,
                },
                bashar: UiImage::Texture(self.bashar.take().expect("iléou bashar.jpeg")),
            }))
        } else {
            Trans::None
        }
    }
}
