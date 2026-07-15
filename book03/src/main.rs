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

    // wrapping operation:
    assert_eq!(100_u16.wrapping_mul(200), 20000);
    assert_eq!(500_u16.wrapping_mul(500), 53392);
    assert_eq!(500_i16.wrapping_mul(500), -12144);
    assert_eq!(5_i16.wrapping_shl(17), 10);

    // saturating
    assert_eq!(32760_i16.saturating_add(10), 32767);
    assert_eq!((-32760_i16).saturating_sub(10), -32768);

    // overflowing
    assert_eq!(255_u8.overflowing_sub(2), (253, false));
    assert_eq!(255_u8.overflowing_add(2), (1, true));
    assert_eq!(5_u16.overflowing_shl(17), (10, true));

    let x1: f32 = -0.;
    let x2: f32 = 0.;
    println!("{}", x1 == x2);

    let x = std::f64::consts::PI;
    println!("{}", x);

    let x: i32 = 10;
    let y = x as f64;
    println!("{}", y);

    let x = std::f64::consts::PI;
    let y = x as i32;
    println!("{}", y);

    let x = 10;
    if x < 20 {
        println!("x < 20, x is {}", x);
    }
    assert_eq!(false as i32, 0);
    assert_eq!(true as i128, 1);

    let x = &true;
    println!("{}", *x);

    let c = '你';
    println!("{}", c);

    let c: [char; 4] = ['*', 42 as char, '\x2A', '\u{CA0}'];
    println!("{:?}", c);

    let c = ['\u{000000}' as char, '\u{007F}', '\u{d7ff}', '\u{10FFFF}'];
    println!("{:?}", c);

    assert_eq!('*' as i32, 42);
    assert_eq!('\u{CA0}' as u16, 0xca0);
    assert_eq!('\u{CA0}' as i8, -0x60);

    if let Some(c) = char::from_u32(500) {
        println!("{}", c);
    }

    assert_eq!('*'.is_alphabetic(), false);
    assert_eq!('8'.to_digit(10), Some(8));
    assert_eq!(('\u{CA0}').len_utf8(), 3);

    let text = "I see the eigenvalue in thine eye";
    let (head, tail) = text.split_at(21);
    println!("{}, {}", head, tail);

    let temp = text.split_at(21);
    let head = temp.0;
    let tail = temp.1;
    println!("{}, {}", head, tail);

    let s = String::from("Hello world");
    let ref_s = &s;
    println!("{}, {}", ref_s, *ref_s);

    let i = 10;
    let ref_i = &i;
    println!("{}, {}", ref_i, *ref_i);

    let t = (12, "eggs");
    let b = Box::new(t);
    println!("{:?}", b);

    let lazy_caterer: [u32; 6] = [1, 2, 3, 4, 5, 6];
    let taxonomy = ["Animalia", "Arthropoda", "Insecta"];

    assert_eq!(lazy_caterer[3], 4);
    assert_eq!(taxonomy.len(), 3);

    let mut sieve = [true; 10000];
    for i in 2..100 {
        if sieve[i] {
            let mut j = i * i;
            while j < 10000 {
                sieve[j] = false;
                j += i;
            }
        }
    }

    assert_eq!(sieve[211], true);
    assert!(!sieve[9876]);

    let buf = [0u8; 1024];
    println!("{}", buf[0]);

    const N: usize = 10;
    let array = [0i32; N];
    println!("{:?}", array);

    let mut chaos = [5, 2, 3, 4, 1];
    chaos.sort();
    assert_eq!(chaos, [1, 2, 3, 4, 5]);

    let mut primes = vec![2, 3, 5, 7];
    assert_eq!(primes.iter().product::<i32>(), 210);

    primes.push(11);
    primes.push(13);
    assert_eq!(primes.iter().product::<i32>(), 30030);

    let x = new_pixel_buffer(3, 3);
    println!("{:?}", x);

    let mut pal = Vec::new();
    pal.push("step");
    pal.push("on");
    pal.push("no");
    pal.push("pets");
    assert_eq!(pal, vec!["step", "on", "no", "pets"]);

    let v: Vec<u32> = (0..5).collect();
    assert_eq!(v, [0, 1, 2, 3, 4]);
}

fn new_pixel_buffer(rows: usize, cols: usize) -> Vec<u8> {
    vec![0; rows * cols]
}

fn _swap<T>(_x: &mut T, _y: &mut T) {}
fn _swap_1<T>(_x: &mut T, _y: &mut T) -> () {}

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
