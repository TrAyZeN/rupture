use amethyst::assets::AssetStorage;
use amethyst::audio::output::Output;
use amethyst::audio::Source;
use amethyst::core::ecs::{Read, System, SystemData, Write};
use amethyst::core::Time;
use amethyst::derive::SystemDesc;

use crate::{CodeFound, play, PlayerHidden, Sounds, TimeToScreamer};

#[derive(Debug, SystemDesc)]
#[system_desc(name(ScreamerSystemDesc))]
pub struct ScreamerSystem;

impl<'s> System<'s> for ScreamerSystem {
    type SystemData = (
        Read<'s, Time>,
        Read<'s, AssetStorage<Source>>,
        Read<'s, Sounds>,
        Option<Read<'s, Output>>,
        Read<'s, CodeFound>,
        Write<'s, TimeToScreamer>,
        Read<'s, PlayerHidden>,
    );

    fn run(&mut self, (time, storage, sound, output, found, mut since, hidden): Self::SystemData) {
        if since.at == 0.0 {
            since.at = time.absolute_time_seconds() + 15.0 + rand::random::<f64>() * 10.0;
        }

        if time.absolute_time_seconds() > since.at - (1.0 + (3.0 / (found.0 as f64 + 1.0)))
            && !since.played
        {
            play(&storage, &sound.coming, &output, 0.65);
            since.played = true;
        }

        if time.absolute_time_seconds() > since.at {
            if !hidden.hidden {
                play(&storage, &sound.screamer, &output, 0.9);
            }

            since.played = false;
            since.at = time.absolute_time_seconds()
                + 5.0
                + (10.0 / (found.0 as f64 + 1.0))
                + rand::random::<f64>() * 10.0;
        }
    }
}
