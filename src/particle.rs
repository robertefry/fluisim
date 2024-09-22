
use bevy::prelude::*;

use crate::settings::*;

#[derive(Component)]
pub(crate) struct Particle
{
    pub velocity: Vec2,
}

pub(crate) struct ParticleSystem;

impl Plugin for ParticleSystem
{
    fn build(&self, app: &mut App)
    {
        app.add_systems(Update, ParticleSystem::on_settings_changed);
    }
}

impl ParticleSystem
{
    pub(crate) fn on_settings_changed(
        mut event_reader: EventReader<SettingsChangedEvent>,
        mut particle_transforms: Query<&mut Transform, With<Particle>>,
        settings: ResMut<Settings>,
    ){
        if let Some(_) = event_reader.read()
            .filter(|e| matches!(e, SettingsChangedEvent::ParticleRadius))
            .last()
        {
            for mut particle_transform in particle_transforms.iter_mut()
            {
                particle_transform.scale = settings.particle_scale();
            }
        }
    }
}
