// Oui c'est dégueulasse, mais est-ce qu'il y a vraiment une autre solution xd
pub fn is_in_bound(x: f32, z: f32) -> bool {
    (x > -25.0 && z > -2.65 && x < 0.65 && z < 0.65) // Couloir
        || is_in_room(x, z) // Salle droite
        || is_in_room(x + 14.0, z) // Salle gauche
}

pub fn is_in_room(x: f32, z: f32) -> bool {
    (x > -2.35 && z > -3.35 && x < -1.55 && z < -2.65) // Porte droite
        || (x > -10.55 && z > -3.35 && x < -9.55 && z < -2.65) // Porte gauche 
        || (x > -12.75 && z > -7.0 && x < 0.55 && z < -3.35) // Entrée salle
        || is_close_from_computer(x, z)
}

pub fn is_close_from_computer(x: f32, z: f32) -> bool {
    (x > -0.85 && z > -22.5 && x < 0.55 && z < -7.0) // Inter droit
        || (x > -8.8 && z > -22.5 && x < -3.1 && z < -7.0) // Inter centre
        || (x > -12.75 && z > -22.5 && x < -11.25 && z < -7.0) // Inter gauche
}
