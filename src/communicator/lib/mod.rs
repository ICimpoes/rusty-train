pub mod client;
pub mod network;


mod outermost {
    pub fn middle_function() {
        inside::inner_function()
    }

    fn middle_secret_function() {
        inside::inner_function()
    }

    pub mod inside {
        use outermost;

        pub fn inner_function() {}

        fn secret_function() {
            outermost::middle_secret_function()
        }

        pub mod inside2 {
            use outermost;

            pub fn inner_function() {
                outermost::middle_secret_function()//child
            }
        }
    }
}

pub mod a {
    fn fa() {}

    pub mod series {
        fn fb() {}

        pub mod of {
            pub fn nested_modules() {
                super::fb();
                ::a::fa();
                ::a::series::fb();
            }
        }
    }
}

pub enum TrafficLight {
    Red,
    Yellow,
    Green,
}


fn try_me() {
    outermost::middle_function();//private+pub
//    outermost::middle_secret_function(); private+private
    outermost::inside::inner_function();//private+pub
    outermost::inside::inside2::inner_function();//private+pub+pub
//    outermost::inside::secret_function();//private+pub+private

    a::series::of::nested_modules();

    use a::series::of;
    of::nested_modules();

    use a::series::of::nested_modules;
    nested_modules()
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


pub fn add_two(a: i32) -> i32 {
    a + 2
}

fn prints_and_returns_10(a: i32) -> i32 {
    println!("I got the value {}", a);
    10
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exploration() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle { length: 8, width: 7 };
        let smaller = Rectangle { length: 5, width: 1 };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle { length: 8, width: 7 };
        let smaller = Rectangle { length: 5, width: 1 };

        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    fn it_adds_two() {
        assert_eq!(4, add_two(2));
    }

    #[test]
    fn this_test_will_pass() {
        let value = prints_and_returns_10(4);
        assert_eq!(10, value);
    }

//    #[test]
//    fn this_test_will_fail() {
//        let value = prints_and_returns_10(8);
//        assert_eq!(5, value);
//    }

    #[test]
    fn add_two_and_two() {
        assert_eq!(4, add_two(2));
    }

    #[test]
    fn add_three_and_two() {
        assert_eq!(5, add_two(3));
    }

    #[test]
    fn one_hundred() {
        assert_eq!(102, add_two(100));
    }

    #[test]
    #[ignore]
    fn expensive_test() {
        // code that takes an hour to run
    }
}