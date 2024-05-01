
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
        mut particle_radius: ResMut<ParticleRadius>,
    ){
        let window = egui::Window::new("Settings");

        let reference_particle_radius = particle_radius.radius;

        window.show(contexts.ctx_mut(), |ui|
        {
            egui::Grid::new("Particle Settings").show(ui, |ui|
            {
                let radius_range = Particle::RADIUS_MIN..=Particle::RADIUS_MAX;

                ui.label("Radius:");
                egui::Slider::new(&mut particle_radius.radius, radius_range).ui(ui);
                ui.end_row();
            });
        });

        if reference_particle_radius != particle_radius.radius
        {
            for mut particle_transform in particle_transforms.iter_mut()
            {
                particle_transform.scale = particle_radius.to_scale();
            }
        }
    }
}
