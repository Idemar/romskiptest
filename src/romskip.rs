use bevy::prelude::*;

use crate::bevegelse::Hastighet;

const STRARTER_OVERSETTELSE: Vec3 = Vec3::new(0.0, 0.0, -20.0);
const STARTER_HASTIGHET: Vec3 = Vec3::new(0.0, 0.0, 1.0);

#[derive(Bundle)]
struct RomskipBundle {
    hastighet: Hastighet,
    modell: SceneBundle,
}

pub struct RomskipPlugin;

impl Plugin for RomskipPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_romskip);
    }
}

fn spawn_romskip(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(RomskipBundle {
        hastighet: Hastighet{
            value: STARTER_HASTIGHET
        },
        modell: SceneBundle {
            scene: bevy::prelude::SceneRoot(asset_server.load("Spaceship.glb#Scene0")),
            transform: Transform::from_translation(STRARTER_OVERSETTELSE),
            ..default()
        },
    });
}