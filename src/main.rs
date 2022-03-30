#![allow(dead_code)]
#![allow(unused_variables)]
// ^ So we can write examples without the compiler complaining about warnings.
mod chapter05;
mod chapter06;
mod chapter07;

pub use crate::chapter05::c05;
pub use crate::chapter06::c06;
pub use crate::chapter07::c07;

fn main() {
    // c05::structs();
    // c06::enums();
    c07::packages_crates_modules();
}
