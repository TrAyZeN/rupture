use amethyst::{
    assets::AssetStorage,
    audio::{output::Output, Source},
    core::Time,
    derive::SystemDesc,
    ecs::{Read, System, SystemData, Write},
};
use std::time::Duration;

use rand::Rng;

use crate::{
    play,
    states::game::{Sounds, UnlockedComputers},
    ui::Reading,
};

const COMPUTER_NUMBER: usize = 32;

#[derive(Debug, SystemDesc)]
#[system_desc(name(ComputerystemDesc))]
pub struct ComputerSystem;

impl<'s> System<'s> for ComputerSystem {
    type SystemData = (
        Read<'s, Time>,
        Read<'s, AssetStorage<Source>>,
        Read<'s, Sounds>,
        Option<Read<'s, Output>>,
        Write<'s, UnlockedComputers>,
        Read<'s, Reading>,
    );

    fn run(&mut self, (time, storage, sounds, output, mut uc, reading): Self::SystemData) {
        if reading.0 {
            uc.last_unlock_time = time.absolute_time();
            return;
        }

        let mut rng = rand::thread_rng();
        if uc.unlocked_computers.len() < COMPUTER_NUMBER
            && time.absolute_time()
                > uc.last_unlock_time
                    + Duration::new(uc.unlocked_computers.len() as u64 + rng.gen_range(6, 12), 0)
        {
            let mut computer_id = rng.gen_range(0, COMPUTER_NUMBER);
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
            play(&storage, &sounds.boot, &output, 0.2);
        }
    }
}
