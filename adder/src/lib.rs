pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub fn add_two(x: i32) -> i32 {
    x + 2
} 

// private function
fn internal_adder(a: i32, b:i32) -> i32 {
    a + b
}

pub fn greeting(name: &str) -> String {
    format!{"Hello, {}!", name}
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

pub struct Guess {
    value: u32,
}

impl Guess {
    pub fn new(value: u32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100");
        }
        Guess {
            value
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    // comment out the ignore if you want this test to fail
    // with the ignore attribute this test will be ignored 
    #[test]
    #[ignore]
    fn make_it_fail() {
        panic!("this test will fail");
    }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {length: 8, width: 7};
        let smaller = Rectangle {length: 7, width: 5};
        
        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {length: 8, width: 7};
        let smaller = Rectangle { length: 7, width: 4};

        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(result.contains("Carol"));
    }

    // this will pass if the code panics
    #[test]
    #[should_panic]
    fn greater_than_100() {
        Guess::new(200);
    }

    #[test]
    #[should_panic(expected = "Guess value must be between 1 and 100")]
    fn less_than_200() {
        Guess::new(0);
    }

    #[test]
    fn internal_function_test() {
        assert_eq!(4, internal_adder(2, 2));
    }
}

// you test inequality with the assert_eq! and assert_ne! macros
// testing proper error handling with should_panic macro

// you'll put unit tests in the src directory in esach file with th ecode that they're testing
// convention is to create module named test in each file to contain the test functions and to
// annotate teh module with cfg(test).