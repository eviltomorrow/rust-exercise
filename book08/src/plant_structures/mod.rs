mod leaves;
mod roots;
mod stems;
mod synthesis;

pub enum Aminocid {
    A,
}

pub fn try_synthesize() {
    synthesis::synthesize(&[Aminocid::A]);
}
