use bevy::prelude::*;

const DESPAWN_AVSTAND: f32 = 100.0;

pub struct DespawnPlugin;

impl Plugin for DespawnPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(Update, despawn_langt_vekk_entities);
    }
}

fn despawn_langt_vekk_entities(mut commands: Commmands, query: Queyry<(Entity, &GlobalTransform)>) {
    for (entity, transform) in query.iter() {
        let avstand = transform.translation().distance(Vec3::ZERO);

        if avstand > DESPAWN_AVSTAND {
            commands.entity(entity).despawn_recursive();
        }
    }
}
