
use bevy::prelude::*;
use bevy_egui::*;
use bevy_egui::egui::Widget;

use crate::settings::*;
use crate::state::*;

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
        state_reader: Res<State<SimStates>>,
        mut state_writer: ResMut<NextState<SimStates>>,
        mut settings: ResMut<Settings>,
    ){
        let window = egui::Window::new("Settings");

        window.show(contexts.ctx_mut(), |ui|
        {
            egui::Grid::new("Particle Settings").show(ui, |ui|
            {
                if state_reader.get() == &SimStates::Running
                {
                    ui.set_enabled(false);
                }

                ui.label("Particle Radius:");
                let slider_particle_radius = egui::Slider::new(
                    &mut settings.particle_radius,
                    Settings::PARTICLE_RADIUS.into(),
                ).ui(ui);
                ui.end_row();

                if slider_particle_radius.changed()
                {
                    event_writer.send(SettingsChangedEvent::ParticleRadius);
                }

                ui.label("Border Damping:");
                let slider_border_damping = egui::Slider::new(
                    &mut settings.border_damping,
                    Settings::BORDER_DAMPING.into(),
                ).ui(ui);
                ui.end_row();

                if slider_border_damping.changed()
                {
                    event_writer.send(SettingsChangedEvent::BorderDamping);
                }

                ui.label("Gravity:");
                let slider_gravity = egui::Slider::new(
                    &mut settings.gravity,
                    Settings::GRAVITY.into(),
                ).ui(ui);
                ui.end_row();

                if slider_gravity.changed()
                {
                    event_writer.send(SettingsChangedEvent::Gravity);
                }

                ui.label("Force Multiplier:");
                let slider_force_multiplier = egui::Slider::new(
                    &mut settings.force_multiplier,
                    Settings::FORCE_MULTIPLIER.into(),
                ).ui(ui);
                ui.end_row();

                if slider_force_multiplier.changed()
                {
                    event_writer.send(SettingsChangedEvent::ForceMultiplier);
                }
            });

            let button_running = match state_reader.get()
            {
                SimStates::Configure => ui.button("Start Simulation"),
                SimStates::Running => ui.button("Pause Simulation"),
                SimStates::Paused => ui.button("Resume Simulation"),
            };

            if button_running.clicked()
            {
                match state_reader.get()
                {
                    SimStates::Configure => (*state_writer).set(SimStates::Running),
                    SimStates::Running => (*state_writer).set(SimStates::Paused),
                    SimStates::Paused => (*state_writer).set(SimStates::Running),
                }
            }
        });
    }
}
