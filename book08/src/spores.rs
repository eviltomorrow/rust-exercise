pub struct Spore {
    #[allow(unused)]
    pub name: String,
}

pub struct Sporangium {}

pub struct Gene {}

struct Cell {}

struct Phone {
    #[allow(unused)]
    name: String,
}

pub fn produce_spore(_factory: &mut Sporangium) -> Spore {
    Spore {
        name: String::from("B"),
    }
}

pub(crate) fn genes(_spore: &Spore) -> Vec<Gene> {
    recombine(&mut Cell {});
    vec![]
}

fn recombine(_parent: &mut Cell) {}

mod phcogc;

#[allow(dead_code)]
pub const ROOM_TEMPERATURE: f64 = 20.0;

#[allow(dead_code)]
pub static ROOM_TEMPERATURE_2: f64 = 20.0;

#[allow(dead_code)]
pub struct git_revspec {}

#[cfg(target_os = "android")]
mod mobile {}
