use amethyst::core::ecs::{Read, System, SystemData, Write, WriteStorage};
use amethyst::derive::SystemDesc;
use amethyst::input::{InputHandler, StringBindings};
use amethyst::renderer::light::Light;
use amethyst::ui::UiText;

use crate::{PlayerHidden, PlayerLight, Texts};

#[derive(Debug, SystemDesc)]
#[system_desc(name(HidingSystemDesc))]
pub struct HidingSystem;

impl<'s> System<'s> for HidingSystem {
    type SystemData = (
        Write<'s, PlayerHidden>,
        WriteStorage<'s, UiText>,
        Read<'s, Texts>,
        Read<'s, InputHandler<StringBindings>>,
        WriteStorage<'s, Light>,
        Read<'s, PlayerLight>,
    );

    fn run(&mut self, (mut hidden, mut ui, texts, bindings, mut lights, light): Self::SystemData) {
        if let Some(hide) = texts.hide {
            if let Some(text) = ui.get_mut(hide) {
                if hidden.hidden {
                    text.text = "Rappuyez sur 'P' pour ne plus vous cacher".to_string();
                } else if hidden.can_hide {
                    text.text = "Appuyez sur 'P' pour vous cacher".to_string();
                } else {
                    text.text = String::new();
                }
            }
        }

        if let Some(pressed) = bindings.action_is_down("hide") {
            if pressed && !hidden.pressed {
                hidden.pressed = true;
                hidden.hidden = !hidden.hidden;
            }

            if !pressed && hidden.pressed {
                hidden.pressed = false;
            }
        }

        if let Some(light) = light.0 {
            if let Some(light) = lights.get_mut(light) {
                if let Light::Point(point) = light {
                    point.intensity = if hidden.hidden { 0.0 } else { 2.0 };
                }
            }
        }
    }
}
