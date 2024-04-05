use bevy::prelude::*;
use bevy_editor_pls::EditorPlugin;

// EditorPlugin requires DefaultPlugins to work
fn main() {
    App::new()
        .add_plugins((DefaultPlugins, EditorPlugin::default()))
        .run();
}
