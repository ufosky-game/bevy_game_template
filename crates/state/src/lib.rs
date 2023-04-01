use bevy::prelude::States;

// We use States to separate different stages of our application
// See https://github.com/bevyengine/bevy/blob/main/examples/ecs/state.rs
// Or https://bevy-cheatbook.github.io/programming/states.html
#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash)]
pub enum GameState {
    // During the loading State the LoadingPlugin will load our assets
    #[default]
    Loading,
    // During this State the actual game logic is executed
    Playing,
    // Here the menu is drawn and we wait for player interaction
    Menu,
}
