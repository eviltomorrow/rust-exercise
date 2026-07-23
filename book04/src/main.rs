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

    let s = vec!["A".to_string(), "B".to_string(), "C".to_string()];
    let t = s;
    let u = t;
    println!("{:?}, {:?}", u, "A");

    let mut s = "Govinda".to_string();
    let t = s;
    s = "Hello".to_string();
    println!("{}, {}", t, s);

    // 一般性原则是，如果一个变量的值有可能已经移走，并且从那以后尚未明确赋予其新值，那么它
    // 就可以被看作是为初始化状态。
    let mut x = vec![10, 20, 30];
    let ok = false;
    if ok {
        print_vec(x);
        x = Vec::new();
    }

    println!("{:?}", x);

    let mut v = Vec::new();
    for i in 101..106 {
        v.push(i.to_string());
    }

    let fifth = v.pop().expect("vector empty!");
    println!("{}", fifth);

    let second = v.swap_remove(1);
    println!("{}", second);

    let third = std::mem::replace(&mut v[2], "substitue".to_string());
    println!("{}", third);

    println!("{:?}", v);

    let v = vec!["A".to_string(), "B".to_string(), "C".to_string()];
    for mut s in v {
        s.push('!');
        println!("{}", s);
    }
}

fn print_vec(v: Vec<i32>) {
    for vv in v {
        print!("{}", vv);
    }
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
