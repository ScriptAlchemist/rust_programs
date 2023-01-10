use bevy::prelude::*;
use bevy::input::{KeyboardInput, KeyboardKey};

fn main() {
    App::build()
        .add_default_plugins()
        .add_startup_system(setup.system())
        .add_system(key_event_system.system())
        .run();
}

fn setup(mut commands: Commands) {
    // Create a text component to display the keystrokes
    commands.spawn(TextComponents {
        text: Text::new("Keystrokes:").with_font(Font::load("assets/fonts/FiraSans-Bold.ttf")),
        ..Default::default()
    });
}

fn key_event_system(keyboard_input: Res<Input<KeyboardInput>>, mut text: Query<&mut Text>) {
    // Check if any keys are down
    if let Some(keys) = keyboard_input.keys_down() {
        // Iterate over the keys that are down
        for key in keys {
            let key_string = match key {
                KeyboardKey::A => "A",
                KeyboardKey::B => "B",
                // Add cases for other keys as needed
                _ => "",
            };
            // Append the key string to the text component
            text.get_mut(0).unwrap().value.push_str(key_string);
        }
    }
}

