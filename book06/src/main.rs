fn main() {
    println!("Hello, world!");

    let code = 0;
    let status = if code < 0 { -1 } else { 1 };
    println!("{}", status);

    fn is_ok() -> bool {
        true
    }

    println!("{}", is_ok());

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

    // if 和 match 必须是同样的类型

    let s = if let Some(data) = i32_option() {
        println!("data: {}", data);
        200
    } else {
        -200
    };
    println!("{:?}", s);

    let s = match i32_option() {
        Some(_) => 200,
        _ => -200,
    };
    println!("{:?}", s);

    let mut n = 1;
    while n < 10 {
        n += 1;
        println!("{}", n);
    }

    n = 0;
    while let Some(data) = i32_option() {
        println!("{}", data);
        n += 1;
        if n > 3 {
            break;
        }
    }

    n = 0;
    loop {
        n += 1;
        if n > 3 {
            println!("n > 3");
            break;
        }
    }

    for i in 1..=5 {
        println!("{}", i);
    }

    let strings = vec!["A".to_string(), "B".to_string(), "C".to_string()];
    for s in &strings[..] {
        println!("{}", s);
    }

    let mut strings = vec!["A".to_string(), "B".to_string(), "C".to_string()];
    for s in &mut strings[..] {
        s.push('!');
        println!("{}", s);
    }

    let answer = loop {
        if let Some(line) = next_line() {
            if line.starts_with("answer:") {
                break line;
            }
        } else {
            break "answer: nothing";
        }
    };
    println!("{}", answer);

    'search: for i in 1..=10 {
        if i > 5 {
            break 'search;
        }
        println!("{}", i);
    }

    let sqrt = 'outer: loop {
        let n = 100;
        for i in 1.. {
            let square = i * i;
            if square == n {
                break 'outer i;
            }
            if square > n {
                break;
            }
        }
    };
    println!("{}", sqrt);

    let mut n = 10;
    let s = loop {
        n -= 1;
        if n < 0 {
            break -1;
        }
    };
    println!("{}", s);

    loop_n1();
    loop_n2();
    loop_n3();
    loop_n4();

    // 你不需要刻意去写 -> !，但当你看到 panic!、todo!、loop {} 能随意放在 match 分支里并且编译通过时，记住：这就是 ! 在背后默默发力，让编译器保持单纯和规则统一。
    //
    let v = Vec::<i32>::new();
    println!("{:?}", v);

    let tmp = (0..10).collect::<Vec<i32>>();
    println!("{:?}", tmp);

    let v = [1, 2, 3, 4, 5];
    println!("{:?}", &v[..]);
    println!("{:?}", &v[1..]);
    println!("{:?}", &v[..5]);
    println!("{:?}", &v[1..5]);
    println!("{:?}", &v[..=4]);
    println!("{:?}", &v[1..=4]);

    let v = vec![1, 2, 3, 4, 5, 6];
    for e in &v {
        println!("{}", *e);
    }

    let is_even = |x| x % 2 == 0;
    println!("{}", is_even(10));

    let is_even = |x: u64| -> bool { x % 2 == 0 };
    println!("{}", is_even(10));
}

fn loop_n1() -> i32 {
    let mut n = 10;
    loop {
        n -= 1;
        if n < 0 {
            return -1;
        } else {
            return 1;
        }
    }
}

fn loop_n2() -> i32 {
    let mut n = 10;
    loop {
        n -= 1;
        if n < 10 {
            break 10;
        }
    }
}

fn loop_n3() -> () {
    let mut n = 1;
    loop {
        n += 1;
        if n > 10 {
            break;
        }
    }
}

fn loop_n4() -> i32 {
    #[allow(while_true)]
    while true {
        return 0;
    }
    0
}

#[allow(dead_code)]
fn get_name(id: u32) -> String {
    match id {
        1 => "Alice".to_string(),
        2 => "Bob".to_string(),
        _ => todo!(), // todo!() 返回 !，可以强行充当 String，编译通过！
    }
}

#[allow(dead_code)]
fn start_server() -> ! {
    loop {
        // 处理请求...
        // 一旦函数返回 !，编译器会优化掉 main 函数后面的所有代码
    }
}

fn next_line() -> Option<&'static str> {
    Some("answer: I'm ok")
}

fn i32_option() -> Option<i32> {
    Some(10)
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
