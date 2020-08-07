use amethyst::controls::FlyControlTag;
use amethyst::core::{Time, Transform};
use amethyst::core::ecs::{Join, Read, ReadStorage, System, SystemData, Write, WriteStorage};
use amethyst::core::math::{convert, Unit, Vector3};
use amethyst::derive::SystemDesc;
use amethyst::input::{BindingTypes, get_input_axis_simple, InputHandler};

use crate::{PlayerHidden, space::*};

#[derive(Debug, SystemDesc)]
#[system_desc(name(RuptureMovementSystemDesc))]
pub struct RuptureMovementSystem<T>
    where
        T: BindingTypes,
{
    /// The movement speed of the movement in units per second.
    speed: f32,
    /// The name of the input axis to locally move in the x coordinates.
    right_input_axis: Option<T::Axis>,
    /// The name of the input axis to locally move in the z coordinates.
    forward_input_axis: Option<T::Axis>,
}

impl<T: BindingTypes> RuptureMovementSystem<T> {
    /// Builds a new `FlyMovementSystem` using the provided speeds and axis controls.
    pub fn new(
        speed: f32,
        right_input_axis: Option<T::Axis>,
        forward_input_axis: Option<T::Axis>,
    ) -> Self {
        RuptureMovementSystem {
            speed,
            right_input_axis,
            forward_input_axis,
        }
    }
}

impl<'a, T: BindingTypes> System<'a> for RuptureMovementSystem<T> {
    type SystemData = (
        Read<'a, Time>,
        WriteStorage<'a, Transform>,
        Read<'a, InputHandler<T>>,
        ReadStorage<'a, FlyControlTag>,
        Write<'a, PlayerHidden>,
    );

    fn run(&mut self, (time, mut transform, input, tag, mut hide): Self::SystemData) {
        let x = get_input_axis_simple(&self.right_input_axis, &input);
        let z = get_input_axis_simple(&self.forward_input_axis, &input);

        if hide.hidden {
            return;
        }

        if let Some(dir) = Unit::try_new(Vector3::new(x, 0.0, z), convert(1.0e-6)) {
            for (transform, _) in (&mut transform, &tag).join() {
                let delta_sec = time.delta_seconds();
                let old = transform.translation().clone();

                transform.append_translation_along(dir, delta_sec * self.speed);

                let current = transform.translation().clone();
                if !is_in_bound(current.x, old.z) {
                    transform.set_translation_x(old.x);
                }
                if !is_in_bound(old.x, current.z) {
                    transform.set_translation_z(old.z);
                }

                let current = transform.translation().clone();
                hide.can_hide = is_close_from_computer(current.x, current.z)
                    || is_close_from_computer(current.x + 14.0, current.z);

                transform.set_translation_y(old.y);

                // println!("X: {}, Z: {}", current.x, current.z);
            }
        }
    }
}