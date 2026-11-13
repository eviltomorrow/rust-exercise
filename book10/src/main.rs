use std::cmp::Ordering::Equal;
use std::cmp::Ordering::Greater;
use std::cmp::Ordering::Less;
use std::cmp::Ordering::{self};
use std::collections::HashMap;

fn main() {
    println!("Hello, world!");
    compare(10, 20);
    compare2(10, 20);

    let _ = Pet::Giraffe;
    let _ = Pet::Orca;

    assert_eq!(size_of::<HttpStatus>(), 2);
    assert_eq!(HttpStatus::Ok as i32, 200);

    if let Some(status) = http_status_from_u32(200) {
        println!("{}", status as i32)
    }
    if let Some(status) = http_status_from_u32(10) {
        println!("{}", status as i32)
    }

    let s = TimeUnit::Seconds.plural();
    println!("{}", s);

    let s = TimeUnit::Seconds.singular();
    println!("{}", s);

    let four_score_and_even_years_ago = RoughTime::InThePast(TimeUnit::Days, 30);
    let three_hours_from_now = RoughTime::JustNow;
    println!(
        "{:?}, {:?}",
        four_score_and_even_years_ago, three_hours_from_now
    );

    const ORIGIN: Point3d = Point3d {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };
    let unit_sphere = Shape::Sphere {
        center: ORIGIN,
        radius: 1.0,
    };
    println!("{:?}", unit_sphere);
    let jupiter_tree = BinaryTree::NotEmpty(Box::new(TreeNode {
        element: "Jupiter",
        left: BinaryTree::Empty,
        right: BinaryTree::Empty,
    }));

    let mars_tree = BinaryTree::NotEmpty(Box::new(TreeNode {
        element: "Mars",
        left: jupiter_tree,
        right: BinaryTree::Empty,
    }));
    println!("{:?}", mars_tree);

    let s = rough_time_to_english(RoughTime::JustNow);
    println!("{}", s);

    let meadow = Meadow {};
    match meadow.count_rabbits() {
        0 => {}
        1 => {
            println!("A rabbit is nosing around in the clover.");
        }
        n => {
            println!("There are {} rabbits hopping about in the meadow", n);
        }
    }

    let s = describe_point(10, 20);
    println!("{}", s);

    let balloon = Balloon {
        location: Point { x: 10.0, y: 20.0 },
    };
    match balloon.location {
        Point { x: 10.0, y: height } => {
            println!("straight up {} meters", height);
        }
        Point { x: 20.0, .. } => {
            println!("{} == 20.0", 20.0);
        }
        Point { x, y } => {
            println!("{}, {}", x, y);
        }
    }

    let v = hs1_to_rgp([1, 2, 3]);
    println!("{:?}", v);

    let mut account = Account {
        name: String::from("shepard"),
        language: String::from("english"),
    };

    let ui = UI {};
    match account {
        Account {
            ref name,
            ref mut language,
        } => {
            ui.greet(name, language);
            ui.append_language_with_quote(language);
            ui.show_settings(&account);
        }
    }

    let sphere = Sphere {
        center: Point3d {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        },
    };

    match sphere.center() {
        &Point3d { x, y, z } => {
            println!("{}, {}, {}", x, y, z);
        }
    }

    match sphere.center() {
        &Point3d { x, y, z } if x == 2.0 => {
            println!("{}, {}, {}", x, y, z);
        }
        _ => {
            println!("not match!");
        }
    }

    let x = 10;
    match x {
        1..10 => {
            println!("math")
        }
        _ => {
            println!("no");
        }
    }
}

fn compare(n: i32, m: i32) -> Ordering {
    if n < m {
        Ordering::Less
    } else if n > m {
        Ordering::Greater
    } else {
        Ordering::Equal
    }
}

fn compare2(n: i32, m: i32) -> Ordering {
    if n < m {
        Less
    } else if n > m {
        Greater
    } else {
        Equal
    }
}

enum Pet {
    Orca,
    Giraffe,
}

#[allow(dead_code)]
enum HttpStatus {
    Ok = 200,
    NotModified = 304,
    NotFound = 404,
}

fn http_status_from_u32(n: u32) -> Option<HttpStatus> {
    match n {
        200 => Some(HttpStatus::Ok),
        304 => Some(HttpStatus::NotModified),
        404 => Some(HttpStatus::NotFound),
        _ => None,
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[allow(dead_code)]
enum TimeUnit {
    Seconds,
    Minutes,
    Hours,
    Days,
    Months,
    Years,
}

impl TimeUnit {
    fn plural(self) -> &'static str {
        match self {
            TimeUnit::Seconds => "seconds",
            TimeUnit::Minutes => "minutes",
            TimeUnit::Hours => "hours",
            TimeUnit::Days => "days",
            TimeUnit::Months => "months",
            TimeUnit::Years => "years",
        }
    }

    fn singular(self) -> &'static str {
        self.plural().trim_end_matches('s')
    }
}

#[derive(Debug)]
#[allow(dead_code)]
enum RoughTime {
    InThePast(TimeUnit, u32),
    JustNow,
    InTheFuture(TimeUnit, u32),
}

#[derive(Debug)]
#[allow(dead_code)]
struct Point3d {
    x: f32,
    y: f32,
    z: f32,
}

#[derive(Debug)]
#[allow(dead_code)]
enum Shape {
    Sphere { center: Point3d, radius: f32 },
    Cuboid { corner1: Point3d, corner2: Point3d },
}

#[derive(Debug)]
#[allow(dead_code)]
enum Json {
    Null,
    Boolean(bool),
    Number(f64),
    String(String),
    Array(Vec<Json>),
    Object(Box<HashMap<String, Json>>),
}

#[derive(Debug)]
#[allow(dead_code)]
struct TreeNode<T> {
    element: T,
    left: BinaryTree<T>,
    right: BinaryTree<T>,
}

#[derive(Debug)]
#[allow(dead_code)]
enum BinaryTree<T> {
    Empty,
    NotEmpty(Box<TreeNode<T>>),
}

fn rough_time_to_english(rt: RoughTime) -> String {
    match rt {
        RoughTime::InThePast(units, count) => {
            format!("{} {} ago", count, units.plural())
        }
        RoughTime::JustNow => {
            format!("just now")
        }
        RoughTime::InTheFuture(units, 1) => {
            format!("{} {} from now", 1, units.singular())
        }
        RoughTime::InTheFuture(units, count) => {
            format!("{} {} from now", count, units.plural())
        }
    }
}

struct Meadow {}

impl Meadow {
    fn count_rabbits(&self) -> i32 {
        3
    }
}

fn describe_point(x: i32, y: i32) -> &'static str {
    use std::cmp::Ordering::*;
    match (x.cmp(&0), y.cmp(&0)) {
        (Equal, Equal) => "at the origin",
        (_, Equal) => "on the x axis",
        (Equal, _) => "on the y axis",
        (Greater, Greater) => "in the first quadrant",
        (Less, Greater) => "in the second quadrant",
        _ => "somewhere else",
    }
}

struct Point {
    x: f32,
    y: f32,
}

struct Balloon {
    location: Point,
}

fn hs1_to_rgp(hs1: [u8; 3]) -> [u8; 3] {
    match hs1 {
        [_, _, 0] => [0, 0, 0],
        [_, _, 255] => [255, 255, 255],
        _ => [1, 1, 1],
    }
}

#[derive(Debug)]
struct Account {
    name: String,
    language: String,
}

struct UI {}

impl UI {
    fn greet(&self, name: &str, language: &str) {
        println!("{}, {}", name, language);
    }
    fn show_settings(&self, account: &Account) {
        println!("{:?},", account);
    }
    fn append_language_with_quote(&self, language: &mut String) {
        language.push_str("!");
    }
}

struct Sphere {
    center: Point3d,
}

impl Sphere {
    fn center(&self) -> &Point3d {
        &self.center
    }
}
