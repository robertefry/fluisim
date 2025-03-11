
use bevy::prelude::*;
use bevy::sprite::*;

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
            let y = (i as f32) * grid_size + offset.y;
            let x = (j as f32) * grid_size + offset.x;

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
}
