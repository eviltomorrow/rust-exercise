fn main() {
    println!("Hello, world!");

    print_padovan();

    let point = Box::new((0.625, 0.5));
    let label = format!("{:?}", point);
    println!("{}", label);

    let mut composers = Vec::<Person>::new();
    composers.push(Person {
        name: "Palestrina".to_string(),
        birth: 1525,
    });
    composers.push(Person {
        name: "Dowland".to_string(),
        birth: 1563,
    });
    composers.push(Person {
        name: "Lully".to_string(),
        birth: 1632,
    });

    for composer in &composers {
        println!("{}, born {}", composer.name, composer.birth);
    }

    println!("{:?}", composers);
}

fn print_padovan() {
    let mut padovan = vec![1, 1, 1];
    for i in 3..10 {
        let next = padovan[i - 3] + padovan[i - 2];
        padovan.push(next);
    }
    println!("padovan = {:?}", padovan);
}

#[derive(Debug)]
struct Person {
    name: String,
    birth: i32,
}
