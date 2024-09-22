
// https://www.youtube.com/watch?v=rSKMYc1CQHE

use bevy::prelude::*;

mod util;

mod ui;
mod settings;
mod state;
mod simulation;
mod particle;

use ui::*;
use settings::*;
use state::*;
use simulation::*;

fn main()
{
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup_camera)
        .add_plugins(Simulation)
        .add_plugins(UiSystem)
        .add_plugins(SettingsSystem)
        .init_state::<SimState>()
        .run();
}

fn setup_camera(
    mut commands: Commands,
){
    commands.spawn(Camera2dBundle::default());
}
