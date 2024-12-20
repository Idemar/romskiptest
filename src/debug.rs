use bevy::prelude::*;

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, skriv_ut_posisjon);
    }
}

fn skriv_ut_posisjon(query: Query<(Entity, &Transform)>) {
    for (entity, transform) in query.iter() {
        info!(
            "Entity {:?} er på posisjon {:?},",
            entity, transform.translation
        );
    }
}