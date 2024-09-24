
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
use simulation::*;
use particle::*;

fn main()
{
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup_camera)
        .add_plugins(ParticleSystem)
        .add_plugins(UiSystem)
        .add_plugins(SettingsSystem)
        .add_plugins(Simulation)
        .run();
}

fn setup_camera(
    mut commands: Commands,
){
    commands.spawn(Camera2dBundle::default());
}
