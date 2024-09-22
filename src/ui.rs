
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

        let reference_settings = settings.clone();

        window.show(contexts.ctx_mut(), |ui|
        {
            egui::Grid::new("Particle Settings").show(ui, |ui|
            {
                if state_reader.get() == &SimStates::Running
                {
                    ui.set_enabled(false);
                }

                Self::slider_particle_radius(ui, &mut settings.particle_radius);
                Self::slider_border_damping(ui, &mut settings.border_damping);
                Self::slider_gravity(ui, &mut settings.gravity);
                Self::slider_force_multiplier(ui, &mut settings.force_multiplier);
            });

            Self::show_start_stop_button(ui, state_reader, &mut state_writer);
        });

        if reference_settings != *settings
        {
            event_writer.send(SettingsChangedEvent);
        }
    }

    fn slider_particle_radius(ui: &mut egui::Ui, particle_radius: &mut f32)
    {
        ui.label("Particle Radius:");
        egui::Slider::new(
            particle_radius,
            Settings::PARTICLE_RADIUS.into()
            ).ui(ui);
        ui.end_row();
    }

    fn slider_border_damping(ui: &mut egui::Ui, border_damping: &mut f32)
    {
        ui.label("Border Damping:");
        egui::Slider::new(
            border_damping,
            Settings::BORDER_DAMPING.into()
            ).ui(ui);
        ui.end_row();
    }

    fn slider_gravity(ui: &mut egui::Ui, gravity: &mut f32)
    {
        ui.label("Gravity:");
        egui::Slider::new(
            gravity,
            Settings::GRAVITY.into()
            ).ui(ui);
        ui.end_row();
    }

    fn slider_force_multiplier(ui: &mut egui::Ui, force_multiplier: &mut f32)
    {
        ui.label("Force Multiplier:");
        egui::Slider::new(
            force_multiplier,
            Settings::FORCE_MULTIPLIER.into()
            ).ui(ui);
        ui.end_row();
    }

    fn show_start_stop_button(ui: &mut egui::Ui,
        state_reader: Res<State<SimStates>>,
        state_writer: &mut ResMut<NextState<SimStates>>,
    ){
        let button_text = match state_reader.get()
        {
            SimStates::Configure => "Start Simulation",
            SimStates::Running => "Pause Simulation",
            SimStates::Paused => "Resume Simulation",
        };

        if ui.button(button_text).clicked()
        {
            match state_reader.get()
            {
                SimStates::Configure => (*state_writer).set(SimStates::Running),
                SimStates::Running => (*state_writer).set(SimStates::Paused),
                SimStates::Paused => (*state_writer).set(SimStates::Running),
            }
        }
    }
}
