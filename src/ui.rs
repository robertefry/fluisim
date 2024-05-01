
use bevy::prelude::*;
use bevy_egui::*;
use bevy_egui::egui::Widget;

use crate::*;

#[derive(Component)]
pub(crate) struct UiSystem;

impl Plugin for UiSystem
{
    fn build(&self, app: &mut App)
    {
        app.add_plugins(EguiPlugin);
        app.add_systems(Update, UiSystem::redraw);
    }
}

impl UiSystem
{
    fn redraw(
        mut contexts: EguiContexts,
        mut particle_transforms: Query<&mut Transform, With<Particle>>,
        mut particle_resources: ResMut<ParticleResources>,
    ){
        let window = egui::Window::new("Settings");

        let reference_particle_radius = particle_resources.radius;

        window.show(contexts.ctx_mut(), |ui|
        {
            egui::Grid::new("Particle Settings").show(ui, |ui|
            {
                ui.label("Particle Radius:");
                egui::Slider::new(&mut particle_resources.radius, Particle::RADIUS.into()).ui(ui);
                ui.end_row();

                ui.label("Collision Damping:");
                egui::Slider::new(&mut particle_resources.collision_damping, Particle::COLLISION_DAMPING.into()).ui(ui);
                ui.end_row();

                ui.label("Gravity:");
                egui::Slider::new(&mut particle_resources.gravity, Particle::GRAVITY.into()).ui(ui);
                ui.end_row();

                ui.label("Force Multiplier:");
                egui::Slider::new(&mut particle_resources.force_multiplier, Particle::FORCE_MULTIPLIER.into()).ui(ui);
                ui.end_row();
            });
        });

        if reference_particle_radius != particle_resources.radius
        {
            for mut particle_transform in particle_transforms.iter_mut()
            {
                particle_transform.scale = particle_resources.scale();
            }
        }
    }
}
