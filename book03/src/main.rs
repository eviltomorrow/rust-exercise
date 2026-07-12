fn main() {
    println!("Hello, world!");

    let mut v = build_vector_1();
    v.push(30i16);
    println!("{:?}", v);

    let v = build_vector_2();
    println!("{:?}", v);
}

fn build_vector_1() -> Vec<i16> {
    let mut v: Vec<i16> = Vec::<i16>::new();
    v.push(10i16);
    v.push(20i16);
    v
}

fn build_vector_2() -> Vec<i16> {
    let mut v = Vec::new();
    v.push(10);
    v.push(20);
    v
}
