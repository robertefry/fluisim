
use bevy::prelude::*;
use bevy::sprite::*;
use bevy::window::*;

use crate::util::*;

#[derive(Component)]
pub(crate) struct ParticleSystem;

impl ParticleSystem
{
    pub(crate) const PARTICLE_RADIUS: ClosedInterval<f32> = ClosedInterval::new(1.0, 100.0);
    pub(crate) const COLLISION_DAMPING: ClosedInterval<f32> = ClosedInterval::new(0.0, 1.0);
    pub(crate) const GRAVITY: ClosedInterval<f32> = ClosedInterval::new(0.0, 20.0);
    pub(crate) const FORCE_MULTIPLIER: ClosedInterval<f32> = ClosedInterval::new(0.0, 100.0);
}

impl Plugin for ParticleSystem
{
    fn build(&self, app: &mut App)
    {
        app.add_systems(Startup, ParticleSystem::setup);

        app.add_systems(Update, (
            ParticleSystem::on_gravity,
            ParticleSystem::movement,
            ParticleSystem::confine_to_window,
        ).chain());

        app.init_resource::<ParticleResources>();
    }
}

impl ParticleSystem
{
    fn setup(
        mut commands: Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<ColorMaterial>>,
        particle_resources: Res<ParticleResources>,
    ){
        let particle_mesh: Mesh = Circle::new(ParticleSystem::PARTICLE_RADIUS.upper_bound()).into();
        let particle_material = ColorMaterial::from(Color::CYAN);
        let particle_scale = particle_resources.scale();

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
                velocity: Vec3::new(0.0, 0.0, 0.0),
            },
        ));
    }

    fn movement(
        mut particles: Query<(&mut Transform, &Particle)>,
        time: Res<Time>,
    ){
        for (mut transform, particle) in particles.iter_mut()
        {
            transform.translation += particle.velocity * time.delta_seconds();
        }
    }

    fn confine_to_window(
        mut particles: Query<(&mut Transform, &mut Particle)>,
        window_query: Query<&Window, With<PrimaryWindow>>,
        particle_resources: Res<ParticleResources>,
    ){
        let window = window_query.get_single().unwrap();

        let bounds_min_x = -window.width()  / 2.0 + particle_resources.radius;
        let bounds_max_x =  window.width()  / 2.0 - particle_resources.radius;
        let bounds_min_y = -window.height() / 2.0 + particle_resources.radius;
        let bounds_max_y =  window.height() / 2.0 - particle_resources.radius;

        for (mut transformation, mut particle) in particles.iter_mut()
        {
            // make the particle bounce
            if
                transformation.translation.x <= bounds_min_x ||
                transformation.translation.x >= bounds_max_x
            {
                particle.velocity.x *= -1.0 * (1.0 - particle_resources.collision_damping);
            }
            if
                transformation.translation.y <= bounds_min_y ||
                transformation.translation.y >= bounds_max_y
            {
                particle.velocity.y *= -1.0 * (1.0 - particle_resources.collision_damping);
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
        particle_resources: Res<ParticleResources>,
        time: Res<Time>
    ){
        let gravity = -1.0 * particle_resources.gravity * particle_resources.force_multiplier;
        let gravity_vector = Vec3::new(0.0, gravity, 0.0);

        for mut particle in particles.iter_mut()
        {
            particle.velocity += gravity_vector * time.delta_seconds();
        }
    }
}

#[derive(Resource)]
pub(crate) struct ParticleResources
{
    pub radius: f32,
    pub collision_damping: f32,
    pub gravity: f32,
    pub force_multiplier: f32,
}

impl Default for ParticleResources
{
    fn default() -> Self
    {
        ParticleResources
        {
            radius: ParticleSystem::PARTICLE_RADIUS.denormalise(0.1919191919191919),
            collision_damping: ParticleSystem::COLLISION_DAMPING.lower_bound(),
            gravity: 9.8,
            force_multiplier: 32.0,
        }
    }
}

impl ParticleResources
{
    pub(crate) fn scale(&self) -> Vec3
    {
        let scale = self.radius / ParticleSystem::PARTICLE_RADIUS.upper_bound();
        Vec3::new(scale, scale, scale)
    }
}

#[derive(Component)]
pub(crate) struct Particle
{
    velocity: Vec3,
}
