fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

fn main() {
    // ###############
    // ## SHADOWING ##
    // ###############

    // We can _shadow_ a variable by using the same variable’s name
    // and repeating the use of the let keyword as follows:
    let x = 5;
    let x = x + 1;
    let x = x * 2;
    println!("The value of x is {}", x);

    // Shadowing spares us from having to come up with different names,
    // such as spaces_str and spaces_num; instead, we can reuse the same name:
    let spaces = "    "; // str
    let spaces = spaces.len(); // usize
    println!("There are {} spaces in there", spaces);

    // However, if we try to use mut for this, as shown here, we’ll get a compile-time error:
    // let mut spaces = "   ";
    // spaces = spaces.len();

    // ################
    // ## DATA TYPES ##
    // ################

    // Rust -> Statically typed (must know types of all variables at compile time)

    // Scalar Types -> Single value (int, float, bool, char)
    // isize and usize types depend on the arch of the computer your program is running on:
    // 64 bits if you’re on a 64-bit architecture and 32 bits if you’re on a 32-bit architecture.

    // Other int representations (Literals):

    // Decimal	98_222
    // Hex	0xff
    // Octal	0o77
    // Binary	0b1111_0000
    // Byte (u8 only)	b'A'

    // integer types default to i32: this type is generally the fastest, even on 64-bit systems

    // Floating-Point types:

    let f1 = 2.0; // f64 (default)
                  // let f2: f32 = 3.0; // f32

    println!("f1 type:");
    print_type_of(&f1); // f64 (default)

    // Char:

    // let heart_eyed_cat = '😻';
    // Rust’s char type is four bytes in size and represents a Unicode Scalar Value, which means
    // it can represent a lot more than just ASCII. Accented letters; Chinese, Japanese, and Korean characters

    // Compound Types -> Can group multiple values into one type (tuples, arrays)

    // Tuple:

    // Tuples have a fixed length: once declared, they cannot grow or shrink in size.

    let tup: (i32, f64, u8) = (500, 6.4, 1); // Each position in the tuple has a type
    println!("tup.0 type:");
    print_type_of(&tup.0);
    // The variable tup binds to the entire tuple, because a tuple is considered a single compound element.
    // We can use pattern matching to destructure a tuple value:
    let (_, y, _) = tup; // _destructuring_
    println!("The value of y is: {}", y);

    // Array:

    // TODO ...
}
