use std::collections::HashMap;
use std::vec;

fn main() {
    println!("Hello, world!");

    let mut table = Table::new();
    table.insert(
        "Gesualdo".to_string(),
        vec![
            "Many madrigals".to_string(),
            "Tenebrae Responsoria".to_string(),
        ],
    );
    table.insert(
        "Caravaggio".to_string(),
        vec![
            "The musicians".to_string(),
            "The Calling of St. Matthew".to_string(),
        ],
    );
    table.insert(
        "Cellini".to_string(),
        vec![
            "Perseus with the head of Medusa".to_string(),
            "A salt cellar".to_string(),
        ],
    );
    show_ref(&table);
    sort_works(&mut table);
    show(table);

    let x = 10;
    let r = &x;
    assert_eq!(10, *r);

    let mut x = 10;
    let m = &mut x;
    *m += 32;
    assert_eq!(42, *m);

    let aria = Anime {
        name: "Aria: The Animation",
        bechdel_pass: true,
    };

    let aria_ref = &aria;
    assert_eq!(aria_ref.name, "Aria: The Animation");
    assert_eq!((*aria_ref).name, "Aria: The Animation");

    let mut v = vec![3, 2, 1];
    v.sort();
    (&mut v).sort();
    println!("{:?}", v);

    let x = 10;
    let y = 20;
    let mut r = &x;

    if true {
        r = &y;
    }
    println!("{}", r);

    let point = Point { x: 1000, y: 200 };
    let point_ref_1: &Point = &point;
    let point_ref_2: &&Point = &point_ref_1;
    let point_ref_3: &&&Point = &point_ref_2;
    println!("{:?}, {}, {}", point_ref_3, point_ref_1.x, point_ref_2.y);

    let x = 10;
    let y = 10;

    let rx = &x;
    let ry = &y;

    let rrx = &rx;
    let rry = &ry;

    println!("{}, {}", rrx, rry);
    assert!(rrx <= rry);
    assert!(rrx == rry);

    let r = &factorial(6);
    println!("{}", r);

    // let r;
    // {
    //     let x = 1;
    //     r = &x;
    // }
    // println!("{}", r);
    //
    f(&100);

    println!("{}", g());

    let x = 10;
    h(&x);
    unsafe {
        h(STASH);
    }

    let s = [7, 6, 5, 4, 3, 2, 1];
    let v = smallest(&s);
    println!("{}", v);
    println!("{:?}", s);

    let s;
    {
        let parabola = [9, 4, 1, 0, 1, 4, 9];
        s = smallest(&parabola);
        println!("{:?}", s);
    }

    let x = 10;
    {
        let s = S { r: &x };
        println!("{:?}", s);

        let d = D { s };
        println!("{:?}", d.s);
    }

    let v = vec![1, 2, 3, 4, 5];
    let s = build_vec(&v);
    println!("{:?}", s);

    let x = 10;
    let r;
    {
        let y = 20;
        {
            let g = G { x: &x, y: &y };
            r = g.x;
        }
    }
    println!("{}", r);

    let x = 10;
    {
        let y = 20;
        let z = a1(&x, &y);
        println!("{}", z);
    }

    let x = 10;
    {
        let y = 20;
        let z = a2(&x, &y);
        println!("{}", z);
    }

    let v = vec![4, 8, 19, 27, 34, 10];
    {
        let r = &v;
        r[0];
    }
    let aside = v;
    println!("{:?}", aside);

    let mut wave = Vec::new();
    let head = vec![0.0, 1.0];
    let tail = [0.0, -1.0];

    extend(&mut wave, &head);
    extend(&mut wave, &tail);

    println!("{:?}", wave);

    let mut v = vec![1, 2, 3, 4, 5];
    v.push(10);
    let r = &v;
    println!("{:?}, {}", r, r[0]);

    let mut v = vec!["A".to_string(), "B".to_string()];
    v.push("C".to_string());
    println!("{}", v[0]);
    let r = &v[0];
    println!("{}", r);

    let mut x = 10;
    let r1 = &x;
    let r2 = &x;
    println!("{}, {}", r1, r2);
    x += 10;
    println!("{}", x);

    #[allow(unused_mut)]
    let mut w = (107, 109);
    let r = &w;
    let r0 = &r.0;
    // let m1 = &mut r.1;
    println!("{}, {:?}", r0, w);

    let mut v = (136, 139);
    let m = &mut v;
    let m0 = &mut m.0;
    *m0 = 137;
    let r1 = &m.1;
    // v.1;
    println!("{}", r1);
}

fn extend(vec: &mut Vec<f64>, slice: &[f64]) {
    for elt in &slice[..] {
        vec.push(*elt);
    }
}

fn a1<'a, 'b>(r: &'a i32, s: &'b i32) -> &'a i32 {
    let _ = s;
    r
}

fn a2<'a>(r: &'a i32, s: &'a i32) -> &'a i32 {
    let _ = s;
    r
}

#[allow(dead_code)]
struct G<'a, 'b> {
    x: &'a i32,
    y: &'b i32,
}

fn build_vec(i: &[i32]) -> Vec<&i32> {
    let mut v = Vec::new();
    for n in i {
        v.push(n);
    }
    v
}

struct D<'b> {
    s: S<'b>,
}

#[derive(Debug)]
struct S<'a> {
    #[allow(dead_code)]
    r: &'a i32,
}

fn smallest(v: &[i32]) -> &i32 {
    let mut s = &v[0];
    for r in &v[1..] {
        if r < s {
            s = r;
        }
    }
    s
}

static mut STASH: &i32 = &10;

fn h(p: &i32) {
    println!("{}", p);
}

fn f(p: &'static i32) {
    unsafe {
        STASH = p;
    }
}

fn g() -> i32 {
    unsafe { *STASH }
}

fn factorial(n: usize) -> usize {
    (1..n + 1).product()
}

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

struct Anime {
    name: &'static str,

    #[allow(dead_code)]
    bechdel_pass: bool,
}

type Table = HashMap<String, Vec<String>>;

fn show(table: Table) {
    for (artist, works) in table {
        println!("works by {}:", artist);
        for work in works {
            println!("  {}", work);
        }
    }
}

fn show_ref(table: &Table) {
    for (artist, works) in table {
        println!("works by {}:", artist);
        for work in works {
            println!("  {}", work);
        }
    }
}

fn sort_works(table: &mut Table) {
    for (_, works) in table {
        works.sort();
        for work in works {
            work.push('!');
        }
    }
}
