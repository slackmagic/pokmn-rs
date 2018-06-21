use part01::pokemon::*;

pub fn main() {
    let pikachu: Pokemon = Pokemon {
        id: 1,
        name: "Pikachu".to_string(),
        element: "Foudre".to_string(),
        in_pokeball: true,
    };

    let pokeball: Pokeball = Pokeball { pokemon: pikachu };

    //Let's fight.
    //let combattant = ouvrir_pokeball(pokeball);

    //Time to return.
    //pokeball = rentrer_pokemon(combattant);
}

pub fn ouvrir_pokeball(pokeball: Pokeball) -> Pokemon {
    //pokemon.in_pokeball = false;
    println!("{}, je te choisis !", pokeball.pokemon.name);
    return pokeball.pokemon;
}

pub fn rentrer_pokemon(pokemon: Pokemon) -> Pokeball {
    //pokemon.in_pokeball = false;
    println!("Reviens {} !", pokemon.name);
    return Pokeball { pokemon: pokemon };
}
