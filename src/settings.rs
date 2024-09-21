
use bevy::prelude::*;

use crate::util::*;

#[derive(Copy, Clone, PartialEq)]
#[derive(Resource)]
pub(crate) struct Settings
{
    pub particle_radius: f32,
    pub border_damping: f32,
    pub gravity: f32,
    pub force_multiplier: f32,
}

impl Settings
{
    pub(crate) const PARTICLE_RADIUS:  ClosedInterval<f32> = ClosedInterval::new(1.0, 100.0);
    pub(crate) const BORDER_DAMPING:   ClosedInterval<f32> = ClosedInterval::new(0.0,   1.0);
    pub(crate) const GRAVITY:          ClosedInterval<f32> = ClosedInterval::new(0.0,  20.0);
    pub(crate) const FORCE_MULTIPLIER: ClosedInterval<f32> = ClosedInterval::new(0.0, 100.0);
}

impl Default for Settings
{
    fn default() -> Self
    {
        Self
        {
            particle_radius: Settings::PARTICLE_RADIUS.denormalise(0.1919191919191919),
            border_damping: Settings::BORDER_DAMPING.lower_bound(),
            gravity: 9.8,
            force_multiplier: 32.0,
        }
    }
}

impl Settings
{
    pub(crate) fn particle_scale(&self) -> Vec3
    {
        let scale = self.particle_radius / Settings::PARTICLE_RADIUS.upper_bound();
        Vec3::new(scale, scale, scale)
    }
}
