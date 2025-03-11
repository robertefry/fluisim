
use bevy::prelude::*;
use bevy::window::*;

use util::*;
use crate::settings::*;
use crate::state::*;

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
        let particle_radius = *Settings::PARTICLE_RADIUS.upper_value().unwrap();
        let particle_mesh: Mesh = Circle::new(particle_radius).into();
        let particle_material = ColorMaterial::from(Color::hsl(180.0, 1.0, 0.5));

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
            .run_if(on_event::<SettingsChangedEvent>)
            );

        app.add_systems(Update,
            (
                ParticleSystem::on_gravity,
                ParticleSystem::movement,
                ParticleSystem::confine_to_window,
            )
            .chain()
            .run_if(in_state(SimState::Running))
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

    fn movement(
        mut particles: Query<(&mut Transform, &Particle)>,
        time: Res<Time>,
    ){
        for (mut transform, particle) in particles.iter_mut()
        {
            let delta = particle.velocity * time.delta_secs();
            transform.translation += Vec3::new(delta.x, delta.y, 0.0);
        }
    }

    fn confine_to_window(
        mut particles: Query<(&mut Transform, &mut Particle)>,
        window_query: Query<&Window, With<PrimaryWindow>>,
        settings: Res<Settings>,
    ){
        let window = window_query.get_single().unwrap();

        let bounds_min_x = -window.width()  / 2.0 + settings.particle_radius;
        let bounds_max_x =  window.width()  / 2.0 - settings.particle_radius;
        let bounds_min_y = -window.height() / 2.0 + settings.particle_radius;
        let bounds_max_y =  window.height() / 2.0 - settings.particle_radius;

        for (mut transformation, mut particle) in particles.iter_mut()
        {
            // make the particle bounce
            if
                transformation.translation.x <= bounds_min_x ||
                transformation.translation.x >= bounds_max_x
            {
                particle.velocity.x *= -1.0 * (1.0 - settings.border_damping);
            }
            if
                transformation.translation.y <= bounds_min_y ||
                transformation.translation.y >= bounds_max_y
            {
                particle.velocity.y *= -1.0 * (1.0 - settings.border_damping);
            }

            // clamp the particles position inside the window
            transformation.translation = transformation.translation.clamp(
                Vec3::new(bounds_min_x, bounds_min_y, 0.0),
                Vec3::new(bounds_max_x, bounds_max_y, 0.0),
            );
        }
    }

    fn on_gravity(
        mut particles: Query<&mut Particle>,
        settings: Res<Settings>,
        time: Res<Time>
    ){
        let gravity = -1.0 * settings.gravity * settings.force_multiplier;
        let gravity_vector = Vec2::new(0.0, gravity);

        for mut particle in particles.iter_mut()
        {
            particle.velocity += gravity_vector * time.delta_secs();
        }
    }
}
