
use bevy::prelude::*;
use bevy::sprite::*;
use bevy::window::*;

#[derive(Component)]
pub(crate) struct ParticleSystem;

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

        app.init_resource::<ParticleRadius>();
    }
}

impl ParticleSystem
{
    fn setup(
        mut commands: Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<ColorMaterial>>,
        particle_radius: Res<ParticleRadius>,
    ){
        let particle_mesh: Mesh = Circle::new(particle_radius.radius).into();
        let particle_material = ColorMaterial::from(Color::CYAN);

        commands.spawn(
        (
            MaterialMesh2dBundle
            {
                mesh: meshes.add(particle_mesh).into(),
                material: materials.add(particle_material).into(),
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
        particle_radius: Res<ParticleRadius>,
    ){
        let window = window_query.get_single().unwrap();

        let bounds_min_x = -window.width()  / 2.0 + particle_radius.radius;
        let bounds_max_x =  window.width()  / 2.0 - particle_radius.radius;
        let bounds_min_y = -window.height() / 2.0 + particle_radius.radius;
        let bounds_max_y =  window.height() / 2.0 - particle_radius.radius;

        for (mut transformation, mut particle) in particles.iter_mut()
        {
            // make the particle bounce
            if
                transformation.translation.x <= bounds_min_x ||
                transformation.translation.x >= bounds_max_x
            {
                particle.velocity.x *= -1.0 * Particle::COLLISION_DAMPING;
            }
            if
                transformation.translation.y <= bounds_min_y ||
                transformation.translation.y >= bounds_max_y
            {
                particle.velocity.y *= -1.0 * Particle::COLLISION_DAMPING;
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
        time: Res<Time>
    ){
        const GRAVITY: Vec3 = Vec3::new(0.0, -9.8, 0.0);
        const GRAVITY_MULTIPLIER: f32 = 32.0;

        for mut particle in particles.iter_mut()
        {
            particle.velocity += GRAVITY * GRAVITY_MULTIPLIER * time.delta_seconds();
        }
    }
}

#[derive(Resource)]
pub(crate) struct ParticleRadius
{
    pub radius: f32,
}

impl Default for ParticleRadius
{
    fn default() -> Self
    {
        let radius_range = Particle::RADIUS_MAX - Particle::RADIUS_MIN;
        ParticleRadius{ radius: Particle::RADIUS_MIN + radius_range / 2.0 }
    }
}

impl ParticleRadius
{
    pub fn to_scale(&self) -> Vec3
    {
        let radius_offset = self.radius - Particle::RADIUS_MIN;
        let radius_range = Particle::RADIUS_MAX - Particle::RADIUS_MIN;
        let scale = 2.0 * radius_offset / radius_range;

        Vec3::new(scale, scale, scale)
    }
}

#[derive(Component)]
pub(crate) struct Particle
{
    velocity: Vec3,
}

impl Particle
{
    // The percentage of kinetic energy retained on collisions.
    const COLLISION_DAMPING: f32 = 0.9;

    pub const RADIUS_MIN: f32 = 1.0;
    pub const RADIUS_MAX: f32 = 100.0;
}
