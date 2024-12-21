mod bevegelse;
mod debug;
mod kamera;
mod romskip;
mod asset_loader;
mod asteroids;
mod oppdag_kollisjon;

use bevy::prelude::*;

use bevegelse::BevegelsePlugin;
use debug::DebugPlugin;
use kamera::KameraPlugin;
use romskip::RomskipPlugin;
use asset_loader::AssetLoaderPlugin;
use asteroids::AsteroidPlugin;
use oppdag_kollisjon::Oppdag_kollisjonPlugin;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::srgb(0.1, 0.0, 0.25)))
        .insert_resource(AmbientLight {
            color: Color::default(),
            brightness: 1200.0,
        })
        .add_plugins(DefaultPlugins)
        .add_plugins(BevegelsePlugin)
        .add_plugins(DebugPlugin)
        .add_plugins(RomskipPlugin)
        .add_plugins(KameraPlugin)
        .add_plugins(AssetLoaderPlugin)
        .add_plugins(AsteroidPlugin)
        .add_plugins(Oppdag_kollisjonPlugin)
        .run();
}
