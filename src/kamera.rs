use bevy::prelude::*;

const KAMERA_DISTANSE: f32 = 80.0;

pub struct KameraPlugin;

impl Plugin for KameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_kamera);
    }
}

fn spawn_kamera(mut commands: Commands) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, KAMERA_DISTANSE, 0.0).looking_at(Vec3::ZERO, Vec3::Z),
        ..default()
    });
}