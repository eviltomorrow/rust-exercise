fn main() {
    println!("Hello, world!");

    let mut v = build_vector_1();
    v.push(30i16);
    println!("{:?}", v);

    let v = build_vector_2();
    println!("{:?}", v);

    let x = -10;
    let _sum = x + 20i32;
    println!("{:?}", x.abs());
    println!("{:?}", (-4i32).abs());

    let r = 10_i32;
    let x = i8::MAX;
    println!("{}, {}", r, x);

    let b = b'c';
    println!("{}", b);

    let b = b'\r';
    println!("{}", b);

    let b = b'\x4e';
    println!("{}", b);

    assert_eq!(10_i8 as u16, 10_u16);
    assert_eq!(2525u16 as i16, 2525_i16);

    println!("{}", i16::abs(-4));

    let mut i: i32 = 1;
    loop {
        i = i.checked_mul(10).expect("multiplication overflowed");
        if i > 100 {
            break;
        }
    }
    println!("i: {}", i);

    // checked operation:
    assert_eq!(10_u8.checked_add(20), Some(30));
    assert_eq!(100_u8.checked_add(200), None);
    assert_eq!((-128i8).checked_div(-1), None);
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
