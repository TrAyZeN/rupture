use amethyst::{
    core::Time,
    derive::SystemDesc,
    ecs::{Read, System, SystemData, Write},
};
use std::time::Duration;

use rand::Rng;

use crate::states::game::UnlockedComputers;

const COMPUTER_NUMBER: usize = 32;

#[derive(Debug, SystemDesc)]
#[system_desc(name(CodeSystemDesc))]
pub struct CodeSystem;

impl<'s> System<'s> for CodeSystem {
    type SystemData = (Read<'s, Time>, Write<'s, UnlockedComputers>);

    fn run(&mut self, (time, mut uc): Self::SystemData) {
        if uc.unlocked_computers.len() < COMPUTER_NUMBER
            && time.absolute_time()
                > uc.last_unlock_time + Duration::new(4 + uc.unlocked_computers.len() as u64, 0)
        {
            let mut computer_id = rand::thread_rng().gen_range(0, COMPUTER_NUMBER);
            let i = match uc.unlocked_computers.binary_search(&computer_id) {
                Ok(mut i) => {
                    // already present we need to find another one
                    while computer_id == uc.unlocked_computers[i % uc.unlocked_computers.len()] {
                        i += 1;
                        computer_id = (computer_id + 1) % COMPUTER_NUMBER;
                    }
                    i
                }
                Err(i) => i,
            };

            uc.unlocked_computers.insert(i, computer_id);
            uc.last_unlock_time = time.absolute_time();
        }
    }
}
