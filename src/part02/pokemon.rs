pub struct Pokemon {
    pub id: u32,
    pub name: String,
    pub element: String,
    pub in_pokeball: bool,
}

impl Pokemon {
    pub fn new(id: u32, name: String, element: String, in_pokeball: bool) -> Pokemon {
        Pokemon {
            id: id,
            name: name,
            element: element,
            in_pokeball: in_pokeball,
        }
    }
}
