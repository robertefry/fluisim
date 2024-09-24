
use bevy::prelude::*;
use bevy::sprite::*;
use bevy::window::*;

use crate::settings::*;
use crate::state::*;
use crate::particle::*;

pub(crate) struct Simulation;

impl Plugin for Simulation
{
    fn build(&self, app: &mut App)
    {
        app.init_state::<SimState>();

        app.add_systems(Startup,
            Simulation::respawn_particle_grid
            .after(ParticleSystem)
            );

        app.add_systems(OnEnter(SimState::Configure),
            Simulation::respawn_particle_grid
            );

        app.add_systems(Update,
            Simulation::respawn_particle_grid
            .run_if(on_event::<SettingsChangedEvent>())
            .run_if(in_state(SimState::Configure))
            );

        app.add_systems(Update,
            (
                Simulation::on_gravity,
                Simulation::movement,
                Simulation::confine_to_window,
            )
            .chain()
            .run_if(in_state(SimState::Running))
            );
    }
}

impl Simulation
{
    fn respawn_particle_grid(
        mut commands: Commands,
        particles: Query<Entity, With<Particle>>,
        particle_resources: Res<ParticleResources>,
        settings: Res<Settings>,
    ){
        for particle in particles.iter()
        {
            commands.entity(particle).despawn_recursive();
        }

        for (i,j) in itertools::iproduct!(
            0..settings.particle_count.y,
            0..settings.particle_count.x,
        ){
            let grid_size = settings.grid_size();
            let offset = settings.grid_offsets();
            let x = (i as f32) * grid_size + offset.y;
            let y = (j as f32) * grid_size + offset.x;

            commands.spawn((
                MaterialMesh2dBundle
                {
                    mesh: particle_resources.mesh.clone().into(),
                    material: particle_resources.material.clone(),
                    transform: Transform::from_scale(settings.particle_scale())
                        .with_translation(Vec2::new(x,y).extend(0.0)),
                    ..default()
                },
                Particle
                {
                    velocity: Vec2::new(0.0, 0.0),
                },
            ));
        }
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
