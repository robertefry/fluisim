
use crate::util::*;

pub(crate) struct Settings;

impl Settings
{
    pub(crate) const PARTICLE_RADIUS:  ClosedInterval<f32> = ClosedInterval::new(1.0, 100.0);
    pub(crate) const BORDER_DAMPING:   ClosedInterval<f32> = ClosedInterval::new(0.0,   1.0);
    pub(crate) const GRAVITY:          ClosedInterval<f32> = ClosedInterval::new(0.0,  20.0);
    pub(crate) const FORCE_MULTIPLIER: ClosedInterval<f32> = ClosedInterval::new(0.0, 100.0);
}
