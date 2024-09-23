
use bevy::{math::U16Vec2, prelude::*};

use crate::util::*;

pub(crate) struct SettingsSystem;

impl Plugin for SettingsSystem
{
    fn build(&self, app: &mut App)
    {
        app.init_resource::<Settings>();
        app.add_event::<SettingsChangedEvent>();
    }
}

#[derive(Resource, Copy, Clone, PartialEq)]
pub(crate) struct Settings
{
    pub particle_count: U16Vec2,
    pub particle_radius: f32,
    pub particle_sep: f32,
    pub border_damping: f32,
    pub gravity: f32,
    pub force_multiplier: f32,
}

impl Settings
{
    pub(crate) const PARTICLE_COUNT_ROWS: ClosedInterval<u32> = ClosedInterval::new(1  , 1000  );
    pub(crate) const PARTICLE_COUNT_COLS: ClosedInterval<u32> = ClosedInterval::new(1  , 1000  );
    pub(crate) const PARTICLE_RADIUS:     ClosedInterval<f32> = ClosedInterval::new(1.0,  100.0);
    pub(crate) const PARTICLE_SEP:        ClosedInterval<f32> = ClosedInterval::new(0.0,  100.0);
    pub(crate) const BORDER_DAMPING:      ClosedInterval<f32> = ClosedInterval::new(0.0,    1.0);
    pub(crate) const GRAVITY:             ClosedInterval<f32> = ClosedInterval::new(0.0,   20.0);
    pub(crate) const FORCE_MULTIPLIER:    ClosedInterval<f32> = ClosedInterval::new(0.0,  100.0);
}

impl Default for Settings
{
    fn default() -> Self
    {
        Self
        {
            particle_count: U16Vec2::new(1,1),
            particle_radius: Settings::PARTICLE_RADIUS.denormalise(0.1919191919191919),
            particle_sep: 0.0,
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

    pub(crate) fn grid_size(&self) -> f32
    {
        self.particle_radius * 2.0 + self.particle_sep
    }

    pub(crate) fn grid_offsets(&self) -> Vec2
    {
        let count_x = self.particle_count.x as f32;
        let count_y = self.particle_count.y as f32;
        let sep = self.particle_sep;
        let radius = self.particle_radius;

        let grid_wid = (radius * 2.0 + sep) * count_x - sep;
        let grid_hei = (radius * 2.0 + sep) * count_y - sep;
        Vec2::new(radius-grid_wid/2.0, radius-grid_hei/2.0)
    }
}

#[derive(Event, PartialEq)]
pub(crate) enum SettingsChangedEvent
{
    ParticleSetup,
    ParticleRadius,
    BorderDamping,
    Gravity,
    ForceMultiplier,
}
