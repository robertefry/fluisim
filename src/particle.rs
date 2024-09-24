
use bevy::prelude::*;

use crate::util::*;
use crate::settings::*;

#[derive(Component)]
pub(crate) struct Particle
{
    pub velocity: Vec2,
}

#[derive(Resource, Clone, PartialEq)]
pub(crate) struct ParticleResources
{
    pub mesh: Handle<Mesh>,
    pub material: Handle<ColorMaterial>,
}

impl ParticleResources
{
    fn setup(
        mut commands: Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<ColorMaterial>>,
    ){
        let particle_mesh: Mesh = Circle::new(Settings::PARTICLE_RADIUS.upper_bound()).into();
        let particle_material = ColorMaterial::from(Color::CYAN);

        commands.insert_resource(ParticleResources
        {
            mesh: meshes.add(particle_mesh),
            material: materials.add(particle_material),
        });
    }
}

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub(crate) struct ParticleSystem;

impl Plugin for ParticleSystem
{
    fn build(&self, app: &mut App)
    {
        app.add_systems(Startup, ParticleResources::setup
            .in_set(ParticleSystem)
            );

        app.add_systems(Update, ParticleSystem::on_particle_radius_changed
            .in_set(ParticleSystem)
            .run_if(on_event::<SettingsChangedEvent>())
            );
    }
}

impl ParticleSystem
{
    fn on_particle_radius_changed(
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
