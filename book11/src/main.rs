use std::fs::File;
use std::io::Write;

fn main() {
    println!("Hello, world!");

    if let Ok(mut file) = File::create("hello.txt") {
        match say_hello(&mut file) {
            Ok(_) => {}
            Err(err) => {
                println!("{}", err);
            }
        }
    }

    let mut bytes = vec![];
    if let Ok(_) = say_hello(&mut bytes) {}
}

fn say_hello(out: &mut dyn Write) -> std::io::Result<()> {
    out.write_all(b"hello world\n")?;
    out.flush()
}
