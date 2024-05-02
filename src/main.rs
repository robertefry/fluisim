
// https://www.youtube.com/watch?v=rSKMYc1CQHE

use bevy::prelude::*;

mod util;
mod simulation;
mod ui;

use simulation::*;
use ui::*;

fn main()
{
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup_camera)
        .add_plugins(Simulation)
        .add_plugins(UiSystem)
        .run();
}

fn setup_camera(
    mut commands: Commands,
){
    commands.spawn(Camera2dBundle::default());
}
