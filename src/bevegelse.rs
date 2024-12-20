use bevy::prelude::*;

use crate::oppdag_kollisjon::kollidere;

#[derive(Component, Debug)]
pub struct Hastighet {
    pub value: Vec3,
}

impl Hastighet {
    pub fn new(value: Vec3) -> Self {
        Self {value}
    }
}

#[derive(Component, Debug)]
pub struct Akselerasjon {
    pub value: Vec3,
}

impl Akselerasjon {
    pub fn new(value: Vec3) -> Self {
        Self {value}
    }
}

#[derive(Bundle)]
pub struct BevegeligObjektBundle {
    pub hastighet: Hastighet,
    pub akselerasjon: Akselerasjon,
    pub kollidere: kollidere,
}

pub struct BevegelsePlugin;

impl Plugin for BevegelsePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(Update, oppdater_hastighet, oppdater_posisjon);
    }
}

fn oppdater_hastighet(mut query: Query<(&Akselerasjon, &mut Hastighet)>, time: Res<Time>) {
    for (Akselerasjon, mut hastighet) in query.iter_mut() {
        hastighet.value += akselerasjon.value * time.delta_seconds();
    }
}

fn oppdater_posisjon(mut query: Query<(&Akselerasjon, &mut Hastighet)>, time: Res<Time>) {
    for (hastighet, mut transform) in query.iter_mut() {
        transform.translation += hastighet.value * time.delta_seconds();
    }
}