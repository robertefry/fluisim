
use bevy::prelude::*;

#[derive(SystemSet, States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) enum SimState
{
    #[default] Configure,
    Running,
    Paused,
}
