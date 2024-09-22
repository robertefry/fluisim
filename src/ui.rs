
use bevy::prelude::*;
use bevy_egui::*;
use bevy_egui::egui::Widget;

use crate::settings::*;

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
        mut event_writer: EventWriter<SettingsChangedEvent>,
        mut settings: ResMut<Settings>,
    ){
        let window = egui::Window::new("Settings");

        let reference_resources = settings.clone();

        window.show(contexts.ctx_mut(), |ui|
        {
            egui::Grid::new("Particle Settings").show(ui, |ui|
            {
                ui.label("Particle Radius:");
                egui::Slider::new(
                    &mut settings.particle_radius,
                    Settings::PARTICLE_RADIUS.into()
                    ).ui(ui);
                ui.end_row();

                ui.label("Border Damping:");
                egui::Slider::new(
                    &mut settings.border_damping,
                    Settings::BORDER_DAMPING.into()
                    ).ui(ui);
                ui.end_row();

                ui.label("Gravity:");
                egui::Slider::new(
                    &mut settings.gravity,
                    Settings::GRAVITY.into()
                    ).ui(ui);
                ui.end_row();

                ui.label("Force Multiplier:");
                egui::Slider::new(
                    &mut settings.force_multiplier,
                    Settings::FORCE_MULTIPLIER.into()
                    ).ui(ui);
                ui.end_row();

                ui.button("Start Simulation").clicked()
            });
        });

        if reference_resources.particle_radius != settings.particle_radius
        {
            event_writer.send(SettingsChangedEvent);
        }
    }
}
