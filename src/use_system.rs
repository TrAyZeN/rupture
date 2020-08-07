use amethyst::{
    controls::FlyControlTag,
    core::Transform,
    derive::SystemDesc,
    ecs::{Join, Read, ReadStorage, System, SystemData, Write},
    input::{InputHandler, StringBindings},
};

use crate::{space::is_able_to_use_computer, Afit, COMPUTER_NUMBER};

#[derive(SystemDesc)]
pub struct UseSystem;

impl<'s> System<'s> for UseSystem {
    type SystemData = (
        ReadStorage<'s, Transform>,
        Read<'s, InputHandler<StringBindings>>,
        Write<'s, Afit>,
        ReadStorage<'s, FlyControlTag>,
    );

    fn run(&mut self, (transforms, input, mut afit, tags): Self::SystemData) {
        if let Some(pressed) = input.action_is_down("use") {
            'outer: for (transform, _) in (&transforms, &tags).join() {
                for i in 0..COMPUTER_NUMBER {
                    if afit.unlocked_computers.contains(&i)
                        && is_able_to_use_computer(&transform, i)
                    {
                        afit.unlocked_computers.remove(i as usize);
                        afit.code_found += 1;
                        break 'outer;
                    }
                }
            }
        }
    }
}
