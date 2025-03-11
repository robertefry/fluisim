
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
        state_reader: Res<State<SimState>>,
        mut state_writer: ResMut<NextState<SimState>>,
        mut settings: ResMut<Settings>,
    ){
        let window = egui::Window::new("Settings");

        window.show(contexts.ctx_mut(), |ui|
        {
            egui::Grid::new("Simulation Settings").show(ui, |ui|
            {
                ui.label("Particle Count:");
                ui.add_enabled_ui(
                    matches!(state_reader.get(), SimState::Configure),
                    |ui| ui.horizontal(|ui|
                    {
                        ui.label("X:");
                        let slider_particle_count_cols = egui::DragValue::new(
                            &mut settings.particle_count.x)
                            .range(Settings::PARTICLE_COUNT_COLS)
                            .ui(ui);

                        if slider_particle_count_cols.changed()
                        {
                            event_writer.send(SettingsChangedEvent::ParticleCount);
                        }

                        ui.label("Y:");
                        let slider_particle_count_rows = egui::DragValue::new(
                            &mut settings.particle_count.y)
                            .range(Settings::PARTICLE_COUNT_ROWS)
                            .ui(ui);

                        if slider_particle_count_rows.changed()
                        {
                            event_writer.send(SettingsChangedEvent::ParticleCount);
                        }
                    }));
                ui.end_row();

                ui.label("Particle Sep:");
                let slider_particle_sep = ui.add_enabled(
                    matches!(state_reader.get(), SimState::Configure),
                    egui::Slider::new(
                        &mut settings.particle_sep,
                        Settings::PARTICLE_SEP)
                    );
                ui.end_row();

                if slider_particle_sep.changed()
                {
                    event_writer.send(SettingsChangedEvent::ParticleSeparation);
                }

                ui.label("Particle Radius:");
                let slider_particle_radius = ui.add_enabled(
                    matches!(state_reader.get(), SimState::Configure),
                    egui::Slider::new(
                        &mut settings.particle_radius,
                        Settings::PARTICLE_RADIUS)
                    );
                ui.end_row();

                if slider_particle_radius.changed()
                {
                    event_writer.send(SettingsChangedEvent::ParticleRadius);
                }

                ui.label("Border Damping:");
                let slider_border_damping = egui::Slider::new(
                    &mut settings.border_damping,
                    Settings::BORDER_DAMPING)
                    .ui(ui);
                ui.end_row();

                if slider_border_damping.changed()
                {
                    event_writer.send(SettingsChangedEvent::BorderDamping);
                }

                ui.label("Gravity:");
                let slider_gravity = egui::Slider::new(
                    &mut settings.gravity,
                    Settings::GRAVITY)
                    .ui(ui);
                ui.end_row();

                if slider_gravity.changed()
                {
                    event_writer.send(SettingsChangedEvent::Gravity);
                }

                ui.label("Force Multiplier:");
                let slider_force_multiplier = egui::Slider::new(
                    &mut settings.force_multiplier,
                    Settings::FORCE_MULTIPLIER)
                    .ui(ui);
                ui.end_row();

                if slider_force_multiplier.changed()
                {
                    event_writer.send(SettingsChangedEvent::ForceMultiplier);
                }
            });

            ui.horizontal(|ui|
            {
                let button_running = match state_reader.get()
                {
                    SimState::Configure => ui.button("Start"),
                    SimState::Running => ui.button("Pause"),
                    SimState::Paused => ui.button("Resume"),
                };

                if button_running.clicked()
                {
                    match state_reader.get()
                    {
                        SimState::Configure => (*state_writer).set(SimState::Running),
                        SimState::Running => (*state_writer).set(SimState::Paused),
                        SimState::Paused => (*state_writer).set(SimState::Running),
                    }
                }

                let button_reconfigure = ui.add_enabled_ui(
                    !matches!(state_reader.get(), SimState::Configure),
                    |ui| ui.button("Reset")
                ).inner;

                if button_reconfigure.clicked()
                {
                    (*state_writer).set(SimState::Configure);
                }
            });
        });
    }
}
