pub mod c11 {
    pub fn writing_automated_tests() {
        // How to Run Tests
        anatomy_of_a_test_fn()
    }

    fn anatomy_of_a_test_fn() {
        // To change a function into a test function, add #[test] on the line before `fn`.

        // Run tests:
        // $ cargo test

        // When we make a new lib project with Cargo, a test module with a test fn in it is automatically generated for us.
        // $ cargo new adder --lib
    }

    fn adder(num1: i32, num2: i32) -> i32 {
        num1 + num2
    }

    #[derive(Debug)]
    pub struct Rectangle {
        length: u32,
        width: u32,
    }

    impl Rectangle {
        pub fn can_hold(&self, other: &Rectangle) -> bool {
            self.length > other.length && self.width > other.width
        }
    }

    #[cfg(test)]
    mod tests {
        use super::{adder, Rectangle};

        #[test]
        fn it_works() {
            let result = 2 + 2;
            assert_eq!(result, 4);
        }

        #[test]
        fn it_works_as_well() {
            let result = 3 * 3;
            assert_eq!(result, 9);
        }

        #[test]
        fn adder_should_add_two_numbers_and_return_result() {
            let result: i32 = adder(2, 2);
            assert_eq!(2 + 2, 4)
        }

        #[test]
        fn can_hold_should_return_true_when_passing_smaller_rectangle_param() {
            let larger = Rectangle {
                length: 8,
                width: 7,
            };
            let smaller = Rectangle {
                length: 5,
                width: 1,
            };

            assert!(larger.can_hold(&smaller));
        }
    }
}
