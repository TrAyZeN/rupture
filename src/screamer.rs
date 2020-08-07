use amethyst::{
    assets::AssetStorage,
    audio::{output::Output, Source},
    core::Time,
    derive::SystemDesc,
    ecs::{Read, System, SystemData, Write},
};

use crate::{play, Afit, PlayerHidden, Sounds, TimeToScreamer, MAX_CODE};

#[derive(Debug, SystemDesc)]
#[system_desc(name(ScreamerSystemDesc))]
pub struct ScreamerSystem;

impl<'s> System<'s> for ScreamerSystem {
    type SystemData = (
        Read<'s, Time>,
        Read<'s, AssetStorage<Source>>,
        Read<'s, Sounds>,
        Option<Read<'s, Output>>,
        Read<'s, Afit>,
        Write<'s, TimeToScreamer>,
        Read<'s, PlayerHidden>,
    );

    fn run(&mut self, (time, storage, sound, output, afit, mut since, hidden): Self::SystemData) {
        if since.at == 0.0 {
            since.at = time.absolute_time_seconds() + 15.0 + rand::random::<f64>() * 10.0;
        }

        if time.absolute_time_seconds() > since.at - (1.0 + (3.0 / (afit.code_found as f64 + 1.0)))
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
                + (MAX_CODE as f64 / (afit.code_found as f64 + 1.0))
                + rand::random::<f64>() * 10.0;
        }
    }
}
