pub struct Pokemon {
    pub id: u32,
    pub name: String,
    pub element: String,
    pub in_pokeball: bool,
}

pub struct Pokeball {
    pub pokemon: Pokemon,
}
