mod chapter05;
mod chapter06;

pub use crate::chapter05::c05;
pub use crate::chapter06::c06;

fn main() {
    c05::structs();

    c06::enums();
}
