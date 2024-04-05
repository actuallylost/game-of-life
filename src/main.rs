use bevy::prelude::*;
use bevy_editor_pls::EditorPlugin;

fn main() {
    App::new().add_plugins(EditorPlugin::new()).run();
}
