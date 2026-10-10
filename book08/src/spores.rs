pub struct Spore {}

pub struct Sporangium {}

pub struct Gene {}

struct Cell {}

pub fn produce_spore(_factory: &mut Sporangium) -> Spore {
    Spore {}
}

pub(crate) fn genes(_spore: &Spore) -> Vec<Gene> {
    recombine(&mut Cell {});
    vec![]
}

fn recombine(_parent: &mut Cell) {}
