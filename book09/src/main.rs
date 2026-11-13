use std::vec;

fn main() {
    println!("Hello, world!");

    let width = 1024;
    let height = 576;
    let image = GrayscaleMap {
        pixels: vec![0; width * height],
        size: (width, height),
    };
    let _ = image;

    let image = new_map((width, height), vec![0, 0]);
    assert_eq!(image.size, (width, height));
    assert_eq!(image.pixels, vec![0, 0]);

    let hokey = Broom {
        name: "Hokey".to_string(),
        height: 60,
        health: 100,
        position: (100.0, 200.0, 0.0),
        intent: BroomIntent::FetchWater,
    };

    let (hokey1, hokey2) = chop(hokey);
    assert_eq!(hokey1.name, "Hokey I");
    assert_eq!(hokey1.height, 30);
    assert_eq!(hokey1.health, 100);

    assert_eq!(hokey2.name, "Hokey II");
    assert_eq!(hokey2.height, 30);
    assert_eq!(hokey2.health, 100);

    let image_bounds = Bounds(1024, 768);
    assert_eq!(image_bounds.0 * image_bounds.1, 786432);

    let b = new_bounds(10, 20);
    assert_eq!(b.0, 10);
    assert_eq!(b.1, 20);

    let a = Ascii(vec![10, 20]);
    println!("{:?}, {:?}", a, a.0);

    let o = Onesuch;
    println!("{:?}", o);

    let mut q = Queue::new();
    q.push('A');
    q.push('B');
    q.push('C');
    if let Some(c) = q.pop() {
        println!("{}", c);
    }
    q.push('1');
    q.push('2');

    loop {
        if let Some(c) = q.pop() {
            println!("{}", c);
        } else {
            break;
        }
    }

    let mut bq = Box::new(Queue::new());
    bq.push('A');
    bq.push('B');

    let scaled = Vector2::UNIT.scaled_by(10.0);
    println!("{}, {}", scaled.x, scaled.y);

    let mut q = Queue2::<char>::new();
    q.push('C');

    let x = [1, 2, 3, 4, 5, 6];
    let e = find_extrema(&x);
    println!("{:?}", e);

    use std::f64::consts::FRAC_PI_2;
    let sine_poly = Polynomial::new([1.0, 2.0, 3.0, 4.0]);
    println!("{:?}, {}", sine_poly, sine_poly.eval(FRAC_PI_2));

    let p = Point { x: 10.0, y: 20.0 };
    println!("{:?}", p);
}

fn new_map(size: (usize, usize), pixels: Vec<u8>) -> GrayscaleMap {
    GrayscaleMap { pixels, size }
}

#[allow(unused)]
struct GrayscaleMap {
    pixels: Vec<u8>,
    size: (usize, usize),
}

#[allow(dead_code)]
struct Broom {
    name: String,
    height: u32,
    health: u32,
    position: (f32, f32, f32),
    intent: BroomIntent,
}

#[allow(dead_code)]
#[derive(Clone, Copy)]
enum BroomIntent {
    FetchWater,
    DumpWater,
}

fn chop(b: Broom) -> (Broom, Broom) {
    let mut broom1 = Broom {
        height: b.height / 2,
        ..b
    };

    let mut broom2 = Broom {
        name: broom1.name.clone(),
        ..broom1
    };

    broom1.name.push_str(" I");
    broom2.name.push_str(" II");
    (broom1, broom2)
}

struct Bounds(usize, usize);
pub struct Bounds2(pub usize, pub usize);

fn new_bounds(elem0: usize, elem1: usize) -> Bounds {
    Bounds(elem0, elem1)
}

#[derive(Debug)]
struct Ascii(Vec<u8>);

#[derive(Debug)]
struct Onesuch;

pub struct Queue {
    older: Vec<char>,
    younger: Vec<char>,
}

impl Queue {
    pub fn new() -> Queue {
        Queue {
            older: Vec::new(),
            younger: Vec::new(),
        }
    }

    pub fn push(&mut self, c: char) {
        self.younger.push(c);
    }

    pub fn pop(&mut self) -> Option<char> {
        if self.older.is_empty() {
            if self.younger.is_empty() {
                return None;
            }

            use std::mem::swap;
            swap(&mut self.older, &mut self.younger);
            self.older.reverse();
        }
        self.older.pop()
    }

    pub fn is_empty(&self) -> bool {
        self.older.is_empty() && self.younger.is_empty()
    }

    pub fn split(self) -> (Vec<char>, Vec<char>) {
        (self.older, self.younger)
    }
}

pub struct Vector2 {
    x: f32,
    y: f32,
}

#[allow(dead_code)]
impl Vector2 {
    const ZERO: Vector2 = Vector2 { x: 0.0, y: 0.0 };
    const UNIT: Vector2 = Vector2 { x: 1.0, y: 1.0 };

    const NAME: &'static str = "Vector2";
    const ID: u32 = 10;

    fn scaled_by(self, data: f32) -> Vector2 {
        let _ = data;
        self
    }
}

pub struct Queue2<T> {
    older: Vec<T>,
    younger: Vec<T>,
}

impl<T> Queue2<T> {
    pub fn new_2() -> Self {
        Self {
            older: vec![],
            younger: Vec::new(),
        }
    }

    pub fn new() -> Queue2<T> {
        Queue2 {
            older: Vec::new(),
            younger: Vec::new(),
        }
    }

    pub fn push(&mut self, t: T) {
        self.younger.push(t);
    }

    pub fn is_empty(&self) -> bool {
        self.older.is_empty() && self.younger.is_empty()
    }
}

impl Queue2<f64> {
    pub fn sum(&self) -> f64 {
        0.0
    }
}

#[derive(Debug)]
#[allow(dead_code)]
struct Extrema<'elt> {
    greatest: &'elt i32,
    least: &'elt i32,
}

fn find_extrema<'a>(slice: &'a [i32]) -> Extrema<'a> {
    let mut greatest = &slice[0];
    let mut least = &slice[0];
    for i in 1..slice.len() {
        if slice[i] < *least {
            least = &slice[i];
        }
        if slice[i] > *greatest {
            greatest = &slice[i];
        }
    }
    Extrema { greatest, least }
}

#[derive(Debug)]
struct Polynomial<const N: usize> {
    coefficients: [f64; N],
}

impl<const N: usize> Polynomial<N> {
    fn new(coefficients: [f64; N]) -> Polynomial<N> {
        Self { coefficients }
    }
    fn eval(&self, x: f64) -> f64 {
        let mut sum = 0.0;
        for i in (0..N).rev() {
            sum = self.coefficients[i] + x * sum;
        }
        sum
    }
}

#[allow(unused)]
struct LumpOfReferences<'a, T, const N: usize> {
    the_lump: [&'a T; N],
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Point {
    x: f64,
    y: f64,
}
