fn main() {
    println!("Hello, world!");

    let user = User { nickname: "nicell" };

    let name;
    if user.has_nickname() {
        name = user.nickname();
    } else {
        name = generate_unique_name();
    }
    println!("{}", name);

    let a = 10;
    if a < 3 {
        println!("{}", "a < 3");
    } else if 3 <= a && a < 20 {
        println!("{}", "3 <= a < 20");
    } else {
        println!("{}", "a >= 20");
    }

    let code = 0;
    match code {
        0 => println!("ok"),
        1 => println!("Wires Tangled"),
        2 => println!("User Asleep"),
        _ => println!("Unrecognized Error {}", code),
    }
}

struct User {
    nickname: &'static str,
}

impl User {
    fn has_nickname(&self) -> bool {
        true
    }
    fn nickname(&self) -> &'static str {
        self.nickname
    }
}

fn generate_unique_name() -> &'static str {
    "shepard"
}
