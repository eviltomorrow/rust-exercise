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
    spores::genes(&Spore {});
}

mod plant_structures;
mod spores;
