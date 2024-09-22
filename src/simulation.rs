
use bevy::prelude::*;
use bevy::sprite::*;
use bevy::window::*;

use crate::util::*;
use crate::settings::*;
use crate::state::*;
use crate::particle::*;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) struct Simulation;

impl Plugin for Simulation
{
    fn build(&self, app: &mut App)
    {
        app.add_plugins(ParticleSystem);

        app.add_systems(Startup,
            (
                Simulation::setup,
            )
            .in_set(Simulation));

        app.configure_sets(Update, Simulation.run_if(in_state(SimStates::Running)));
        app.add_systems(Update,
            (
                Simulation::on_gravity,
                Simulation::movement,
                Simulation::confine_to_window,
            )
            .chain()
            .in_set(Simulation));
    }
}

impl Simulation
{
    fn setup(
        mut commands: Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<ColorMaterial>>,
        settings: Res<Settings>,
    ){
        let particle_mesh: Mesh = Circle::new(Settings::PARTICLE_RADIUS.upper_bound()).into();
        let particle_material = ColorMaterial::from(Color::CYAN);
        let particle_scale = settings.particle_scale();

        commands.spawn(
        (
            MaterialMesh2dBundle
            {
                mesh: meshes.add(particle_mesh).into(),
                material: materials.add(particle_material).into(),
                transform: Transform::from_scale(particle_scale),
                ..default()
            },
            Particle
            {
                velocity: Vec2::new(0.0, 0.0),
            },
        ));
    }

    fn movement(
        mut particles: Query<(&mut Transform, &Particle)>,
        time: Res<Time>,
    ){
        for (mut transform, particle) in particles.iter_mut()
        {
            let delta = particle.velocity * time.delta_seconds();
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
            particle.velocity += gravity_vector * time.delta_seconds();
        }
    }
}
