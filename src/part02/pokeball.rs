use part02::pokemon::Pokemon;

pub struct Pokeball {
    pub pokemon: Pokemon,
}

impl Pokeball {
    pub fn new(pokemon_to_add: Pokemon) -> Pokeball {
        //Create the object & return.
        Pokeball {
            pokemon: pokemon_to_add,
        }
    }

    pub fn ouvrir(mut self) -> Pokemon {
        println!("{}, je te choisis !", self.pokemon.name);
        self.pokemon.in_pokeball = false;
        self.pokemon
    }

    pub fn rentre(mut self) {
        println!("Reviens {}, tu t'es bien battu !", self.pokemon.name);
        self.pokemon.in_pokeball = true;
    }
}
