use amethyst::{
    derive::SystemDesc,
    ecs::{Entity, Read, System, SystemData, World, Write, WriteStorage},
    input::{InputHandler, StringBindings},
    prelude::{Builder, WorldExt},
    ui::{Anchor, FontHandle, LineMode, UiText, UiTransform},
    winit::MouseButton,
};

#[derive(Default)]
pub struct Texts {
    pub hide: Option<Entity>,
    pub code: Option<Entity>,
    pub menu: Option<Entity>,
}

#[derive(Default)]
pub struct Reading(pub bool);

pub fn create_texts(world: &mut World, font: &FontHandle) -> Texts {
    let hide = world
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
            font.clone(),
            String::new(),
            [1., 1., 1., 1.],
            40.,
        ))
        .build();

    let code = world
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
            font.clone(),
            "Tests passes a 0%".to_string(),
            [1., 1., 1., 1.],
            60.,
        ))
        .build();

    let mut text = UiText::new(
        font.clone(),
        "Vous devez rendre l'AFIT demain soir au plus tard, mais vous n'avez rien fait.\n\n\

        Pris a la fois de panique et d'une enorme flemme, vous decidez de vous introduire la nuit\n \
        en salle machine pour aller recuperer le travail des malheureux ayant oublie de fermer leur session.\n\n\

        Mais attention, Bashar rode et il ne ne vous laissera pas faire !\n\
        Alors si vous entendez un bruit etrange, cachez-vous vite sous une table,\n\
        et vous ressortirez peut-etre vivant d'ici !\n\n\
        
        Cliquez n'importe ou pour commencer"
            .to_string(),
        [1., 1., 1., 1.],
        35.,
    );
    text.line_mode = LineMode::Wrap;

    let menu = world
        .create_entity()
        .with(UiTransform::new(
            "menu".to_string(),
            Anchor::Middle,
            Anchor::Middle,
            0.,
            0.,
            1.,
            1500.,
            375.,
        ))
        .with(text)
        .build();

    Texts {
        hide: Some(hide),
        code: Some(code),
        menu: Some(menu),
    }
}

#[derive(Debug, SystemDesc)]
#[system_desc(name(TextSystemDesc))]
pub struct TextSystem;

impl<'s> System<'s> for TextSystem {
    type SystemData = (
        Write<'s, Reading>,
        WriteStorage<'s, UiText>,
        Read<'s, Texts>,
        Read<'s, InputHandler<StringBindings>>,
    );

    fn run(&mut self, (mut reading, mut ui, texts, inputs): Self::SystemData) {
        if reading.0 && inputs.mouse_button_is_down(MouseButton::Left) {
            reading.0 = false;

            if let Some(menu) = texts.menu {
                if let Some(text) = ui.get_mut(menu) {
                    text.text = String::new();
                }
            }
        }
    }
}
