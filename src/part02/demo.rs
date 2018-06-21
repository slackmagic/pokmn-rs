use part02::pokeball::*;
use part02::pokemon::*;

pub fn main() {
    let pikachu = Pokemon::new(1, "Pikachu".to_string(), "Foudre".to_string(), true);
    let pokeball = Pokeball::new(pikachu);

    //Let's fight.
    pokeball.ouvrir();

    //Time to return.
    pokeball.rentre();
}
