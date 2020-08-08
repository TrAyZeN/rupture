use amethyst::{
    assets::AssetStorage,
    audio::{output::Output, Source},
    core::Time,
    derive::SystemDesc,
    ecs::{Read, System, SystemData, Write, WriteStorage},
    ui::UiTransform,
};

use crate::ui::Reading;
use crate::{
    play,
    states::game::{Afit, PlayerHidden, Screamer, Sounds, TimeToScreamer},
    MAX_CODE,
};

#[derive(Debug, SystemDesc)]
#[system_desc(name(ScreamerSystemDesc))]
pub struct ScreamerSystem;

impl<'s> System<'s> for ScreamerSystem {
    type SystemData = (
        Read<'s, Time>,
        Read<'s, AssetStorage<Source>>,
        Read<'s, Sounds>,
        Read<'s, Screamer>,
        WriteStorage<'s, UiTransform>,
        Option<Read<'s, Output>>,
        Read<'s, Afit>,
        Write<'s, TimeToScreamer>,
        Read<'s, PlayerHidden>,
        Read<'s, Reading>,
    );

    fn run(
        &mut self,
        (time, storage, sound, screamer, mut ui, output, afit, mut since, hidden, reading): Self::SystemData,
    ) {
        if reading.0 {
            return;
        }

        if since.at == 0.0 {
            since.at = time.absolute_time_seconds() + 15.0 + rand::random::<f64>() * 10.0;
        }

        if time.absolute_time_seconds() > since.at - (1.0 + (3.0 / (afit.code_found as f64 + 1.0)))
            && !since.played
        {
            play(&storage, &sound.coming, &output, 0.65);
            since.played = true;
        }

        if time.absolute_time_seconds() - since.last_displayed > 3.5 && since.display {
            std::process::exit(0);
        }

        if time.absolute_time_seconds() > since.at {
            if !hidden.hidden {
                if let Some(bashar) = screamer.bashar {
                    if let Some(transform) = ui.get_mut(bashar) {
                        transform.width = 1024.;
                        transform.height = 768.;
                        since.last_displayed = time.absolute_time_seconds();
                        since.display = true;
                        play(&storage, &sound.screamer, &output, 0.9);
                    }
                }
            }

            since.played = false;
            since.at = time.absolute_time_seconds()
                + 5.0
                + (MAX_CODE as f64 / (afit.code_found as f64 + 1.0))
                + rand::random::<f64>() * 10.0;
        }
    }
}
