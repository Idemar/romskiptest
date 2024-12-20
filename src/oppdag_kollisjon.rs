use bevy::[prelude::*, utils::HashMap];

#[derive(Component, Debug)]
pub stuct Kollidere {
    pub radius: f32,
    pub kolliderer_entities: Vec<Entity>,
}
impl Kollidere {
    pub fn new(radius: f32) -> Self {
        Self {
            radius,
            kolliderer_entities: vec![],
        }
    }
}

pub struct Oppdag_kollisjonPlugin;

impl Plugin for Oppdag_kollisjonPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(oppdag_kollisjon.system());
    }
}

fn oppdag_kollisjon(mut query: Query<(Entity, &GlobalTransform, &mut Kollidere)>) {
    let mut kolliderer_entities: HashMap<Entity, Vec<Entity>> = HashMap::new();

    for (entity_a, transform_a, Kollidere_a) in query.iter() {
        for (entity_b, transform_b, Kollidere_b) in query.iter() {
            if entity_a != entity_b {
                let avstand = transform_a
                    .translation
                    .distance(transform_b.translation);
                if avstand < Kollidere_a.radius + Kollidere_b.radius {
                    kolliderer_entities
                        .entry(entity_a)
                        .or_insert_with(Vec::new)
                        .push(entity_b);
                }
            }
        }
    }
    for (Entity, _, mut Kollidere) in query.iter_mut() {
        Kollidere.kolliderer_entities.clear();
        if let Some(kollisjons) = kolliderer_entities.get(&entity) {
            Kollidere.kolliderer_entities.extend(kollisjons.iter().copied());
        }
    }
}
