
// https://www.youtube.com/watch?v=rSKMYc1CQHE

use bevy::prelude::*;

mod util;

mod ui;
mod settings;
mod state;
mod simulation;
mod particles;

use ui::*;
use settings::*;
use state::*;

fn main()
{
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup_camera)
        .add_plugins(StateManager)
        .add_plugins(UiSystem)
        .init_resource::<Settings>()
        .run();
}

fn setup_camera(
    mut commands: Commands,
){
    commands.spawn(Camera2dBundle::default());
}
