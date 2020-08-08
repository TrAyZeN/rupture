use amethyst::{
    controls::FlyControlTag,
    core::Transform,
    derive::SystemDesc,
    ecs::{Join, Read, ReadStorage, System, SystemData, Write, WriteStorage},
    input::{InputHandler, StringBindings},
    ui::UiText,
};

use crate::{
    space::*,
    states::game::{Afit, PlayerHidden, UnlockedComputers, MAX_CODE},
    ui::Texts,
};

#[derive(SystemDesc)]
#[system_desc(name(UseSystemDesc))]
pub struct UseSystem;

impl<'s> System<'s> for UseSystem {
    type SystemData = (
        ReadStorage<'s, Transform>,
        WriteStorage<'s, UiText>,
        Read<'s, Texts>,
        Read<'s, InputHandler<StringBindings>>,
        Write<'s, Afit>,
        Write<'s, UnlockedComputers>,
        Read<'s, PlayerHidden>,
        ReadStorage<'s, FlyControlTag>,
    );

    fn run(
        &mut self,
        (transforms, mut ui, texts, input, mut afit, mut uc, hidden, tags): Self::SystemData,
    ) {
        for (transform, _) in (&transforms, &tags).join() {
            let pos = transform.translation();
            if let Some(_use) = texts._use {
                if let Some(text) = ui.get_mut(_use) {
                    if is_close_from_computer(pos.x, pos.z) {
                        text.text = "Appuyez sur 'J' pour recuperer le code".to_string();
                    } else {
                        text.text = String::new();
                    }
                }
            }

            if is_close_from_computer(pos.x, pos.z) {
                if let Some(pressed) = input.action_is_down("use") {
                    if !hidden.hidden && pressed {
                        for i in 0..uc.unlocked_computers.len() {
                            if is_able_to_use_computer(&transform, uc.unlocked_computers[i]) {
                                uc.unlocked_computers.remove(i);
                                afit.code_found += 1;

                                if let Some(code) = texts.code {
                                    if let Some(text) = ui.get_mut(code) {
                                        text.text = format!(
                                            "Tests passes a {}%",
                                            (afit.code_found.min(MAX_CODE) as f32 / MAX_CODE as f32
                                                * 100.0)
                                                as i32
                                        );
                                    }
                                }
                                break;
                            }
                        }
                    }
                }
            }
        }
    }
}
