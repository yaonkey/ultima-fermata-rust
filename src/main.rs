use bevy::{prelude::*, asset::AssetPath};

struct Sound {}

impl Sound {
    pub fn play(is_loop: bool, sound_file: String) {
        let sound = asset_server.load(sound_file);
        audio.play(music);
    }
}

fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .add_startup_system(setup)
    .run();
}

fn setup(asset_server: Res<AssetServer>, audio: Res<Audio>) {
    let music = asset_server.load("audio/main.ogg");
    audio.play(music);
}
