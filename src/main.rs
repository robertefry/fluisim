
// https://www.youtube.com/watch?v=rSKMYc1CQHE

use bevy::prelude::*;

mod particles; use particles::*;
mod ui; use ui::*;

fn main()
{
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(UiSystem)
        .add_systems(Startup, setup_camera)
        .add_plugins(ParticleSystem)
        .run();
}

fn setup_camera(
    mut commands: Commands,
){
    commands.spawn(Camera2dBundle::default());
}
