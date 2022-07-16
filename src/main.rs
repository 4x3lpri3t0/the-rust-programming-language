#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
// ^ So we can write examples without the compiler complaining with warnings.

mod chapters;

pub use crate::chapters::chapter02::c02;
pub use crate::chapters::chapter03::c03;
pub use crate::chapters::chapter04::c04;
pub use crate::chapters::chapter05::c05;
pub use crate::chapters::chapter06::c06;
pub use crate::chapters::chapter07::c07;
pub use crate::chapters::chapter08::c08;
pub use crate::chapters::chapter09::c09;
pub use crate::chapters::chapter10::c10;

fn main() {
    // c02::guessing_game();
    // c03::common_concepts();
    // c04::ownership();
    // c05::structs();
    // c06::enums();
    // c07::packages_crates_modules();
    // c08::common_collections();
    // c09::error_handling();
    c10::generics_traits_lifetimes();
}
