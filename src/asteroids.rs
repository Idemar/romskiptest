use std::ops::Range;

use bevy::{ecs::world::SpawnBatchIter, prelude::*, state::commands};
use rand::prelude::*;

use crate::{
    asset_loader::SceneAssets,
    bevegelse::{Akselerasjon, BevegeligObjektBundle, Hastighet},
    oppdag_kollisjon::Kollidere,
};

const HASTIGHET_SKALERER: f32 = 5.0;
const AKSELERASJON_SKALERER: f32 = 1.0;
const SPAWN_RANGE_X: Range<f32> = -25.0..25.0;
const SPAWN_RANGE_Z: Range<f32> = 0.0..25.0;
const SPAWN_TIME_SECS: f32 = 1.0;
const ROTASJONS_HASTIGHET: f32 = 2.5;
const RADIUS: f32 = 2.5;

#[derive(Component, Debug)]
pub struct Asteroid;

#[derive(Resource, Debug)]
pub struct SpawnTimer {
    timer: Timer,
}

pub struct AsteroidPlugin;

impl Plugin for AsteroidPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SpawnTimer {
            timer: Timer::from_seconds(SPAWN_TIME_SECS, TimerMode::Repeating),
        })
        .add_systems(
            Update,
            (spawn_asteroid, rotate_asteroids, handle_asteroid_kollisjon),
        );
    }
}

fn spawn_asteroid(mut commands: Commands, mut spawn_timer: ResMut<SpawnTimer>, time: Res<Time>, scene_assets: Res<SceneAssets>,) {
    spawn_timer.timer.tick(time.delta());
    if !spawn_timer.timer.just_finished() {
        return;
    }

    let mut rng = rand::thread_rng();

    let translation = Vec3::new(
        rng.gen_range(SPAWN_RANGE_X),
        0.,
        rng.gen_range(SPAWN_RANGE_Z),
    );

    let mut ramdon_unit_vector = || Vec3::new(rng.gen_range(-1.0..1.0), 0., rng.gen_range(-1.0..1.0)).normalize_or_zero();
    let hastighet = ramdon_unit_vector() * HASTIGHET_SKALERER;
    let akselerasjon = ramdon_unit_vector() * AKSELERASJON_SKALERER;

    commands.spawn((
        BevegeligObjektBundle {
            akselerasjon: Akselerasjon::new(akselerasjon),
            hastighet: Hastighet::new(hastighet),
            kollidere: Kollidere::new(RADIUS),
            modell: SceneBundle {
                scene: scene_assets.asteroid_scene.clone(),
                transform: Transform::from_translation(translation),
                ..default()
            },
        },
        Asteroid,
    ));
}

fn rotate_asteroids(mut query: Query<&mut Transform, With<Asteroid>>, time: Res<Time>) {
    for mut transform in query.iter_mut() {
        transform.rotate_local_z(ROTASJONS_HASTIGHET * time.delta_secs());
    }
}

fn handle_asteroid_kollisjon(commands: Commands, query: Query<(Entity, &Kollidere), With<Asteroid>>,) {
    for (entity, kollidere) in query.iter() {
        for &kollidere_entity in kollidere.kollisjon_entities.iter() {
           if query.get(kollidere_entity).is_ok() {
            continue;
           }
           commands.entity(entity)despawn_recursive();
        }
    }
}