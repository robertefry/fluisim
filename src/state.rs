
use bevy::prelude::*;

use crate::*;

#[derive(States, Default, Debug, Clone, Eq, PartialEq, Hash)]
pub(crate) enum States
{
    #[default] Running,
}

#[derive(Component)]
pub(crate) struct StateManager;

impl Plugin for StateManager
{
    fn build(&self, app: &mut App)
    {
        app.add_plugins(Simulation);
    }
}
