#![allow(non_camel_case_types)]

use self::spores::Sporangium;
use self::spores::Spore;

fn main() {
    println!("Hello, world!");

    // 1、根 crate 创建的 mod，可以直接访问，它的同级后代也可以直接访问。
    // 2、嵌套模块中的语法项需要将它和它所在的模块标记为公开的，才能使其对外可见。
    // 3、pub(crate) 对于当前 crate 可见，pub(super) 对于父 mod 可见，pub(in <path>) 对于特定父 mod 可见。
    // 4、模块的 4 种表现形式：
    //  a、 mod XXX {}
    //  b、 XXX/mod.rs
    //  c、 XXX.rs
    //  d、 XXX/XXX_1.rs
    // 总结：模块位于自己的文件中、模块位于自己的带有 mod.rs 的目录中，以及模块在自己的文件中，并带有包含子模块的补充目录

    spores::produce_spore(&mut Sporangium {});
    spores::genes(&Spore {
        name: String::from("A"),
    });

    let mut s1 = 10;
    let mut s2 = 20;
    if s1 > s2 {
        std::mem::swap(&mut s1, &mut s2);
    }

    use std::f64::consts::PI;
    println!("{}", PI);

    use std::collections::HashMap as CollectionHashMap;
    #[allow(unused)]
    use std::collections::HashSet;
    #[allow(unused)]
    use std::fs::File as fsFile;
    #[allow(unused)]
    use std::fs::{self};

    let s = CollectionHashMap::<i32, String>::new();
    println!("{:?}", s);

    plant_structures::try_synthesize();

    let _ = self::image::Sampler {};

    #[allow(unused)]
    use ::image::Pixel;
}

mod plant_structures;
mod spores;

mod image {
    pub struct Sampler {}
}

#[test]
fn math_works() {
    let x: i32 = 1;
    assert!(x.is_positive());
    assert_eq!(x + 1, 2);
}

#[test]
#[allow(unconditional_panic, unused_must_use)]
#[should_panic(expected = "divide by zero")]
fn test_divide_by_zero_error() {
    1 / 0;
}

#[allow(unused_imports)]
use std::num::ParseIntError;
#[test]

fn explicit_radix() -> Result<(), ParseIntError> {
    i32::from_str_radix("1024", 10)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    fn roughly_equal(a: f64, b: f64) -> bool {
        (a - b).abs() < 1e-6
    }

    #[test]
    fn trig_works() {
        use std::f64::consts::PI;
        assert!(roughly_equal(PI.sin(), 0.0));
    }
}
