use std::collections::HashMap;

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
