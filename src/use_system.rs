use amethyst::{
    derive::SystemDesc,
    core::Transform,
    controls::FlyControlTag,
    ecs::{System, SystemData, Read, ReadStorage, Write, WriteStorage, Join},
    input::{InputHandler, StringBindings},
    ui::UiText,
};

use crate::{Afit, COMPUTER_NUMBER, space::*, Texts};

#[derive(SystemDesc)]
pub struct UseSystem;

impl<'s> System<'s> for UseSystem {
    type SystemData = (
        ReadStorage<'s, Transform>,
        WriteStorage<'s, UiText>,
        Read<'s, Texts>,
        Read<'s, InputHandler<StringBindings>>,
        Write<'s, Afit>,
        ReadStorage<'s, FlyControlTag>,
    );

    fn run(&mut self, (transforms, mut ui, texts, input, mut afit, tags): Self::SystemData) {
        for (transform, _) in (&transforms, &tags).join() {
            if let Some(_use) = texts._use {
                if let Some(text) = ui.get_mut(_use) {
                    let pos = transform.translation();
                    if is_close_from_computer(pos.x, pos.z) {
                        text.text = "Appuyez sur 'E' ou 'F' pour récupérer le code".to_string();
                    } else {
                        text.text = String::new();
                    }
                }
            }

            if let Some(pressed) = input.action_is_down("use") {
                if pressed {
                    for i in 0..COMPUTER_NUMBER {
                        if afit.unlocked_computers.contains(&i) && is_able_to_use_computer(&transform, i) {
                            afit.unlocked_computers.remove(i as usize);
                            afit.code_found += 1;
                            break;
                        }
                    }
                }
            }
        }
    }
}
