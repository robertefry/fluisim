
use bevy::prelude::*;

use crate::util::*;
use super::settings::*;

#[derive(Resource)]
pub(crate) struct ParticleResources
{
    pub radius: f32,
    pub border_damping: f32,
    pub gravity: f32,
    pub force_multiplier: f32,
}

impl Default for ParticleResources
{
    fn default() -> Self
    {
        ParticleResources
        {
            radius: Settings::PARTICLE_RADIUS.denormalise(0.1919191919191919),
            border_damping: Settings::BORDER_DAMPING.lower_bound(),
            gravity: 9.8,
            force_multiplier: 32.0,
        }
    }
}

impl ParticleResources
{
    pub(crate) fn scale(&self) -> Vec3
    {
        let scale = self.radius / Settings::PARTICLE_RADIUS.upper_bound();
        Vec3::new(scale, scale, scale)
    }
}
