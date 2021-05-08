#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}


fn add_two(a: i32) -> i32 {
    a + 2
}


fn greeting(name: &str) -> String {
    format!("Hello {}!", name)
}

fn do_panic() {
    panic!("Woooooooooo");
}


pub fn run() {
    let r1 = Rectangle { width: 8, height: 7 };
    let r2 = Rectangle { width: 5, height: 1 };
    r1.can_hold(&r2);
    add_two(40);
    greeting("kodeaffe");
    do_panic();
}


#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    /*
    #[test]
    fn lets_panic() {
        panic!("Make this test fail");
    }
    */

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    fn it_adds_two() {
        assert_eq!(4, add_two(2));
    }

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        //let result = greeting("Charol");
        assert!(
            result.contains("Carol"),
            "Greeting did not contain name, value was `{}`",
            result,
        );
    }

    #[test]
    #[should_panic(expected="Woo")]
    fn should_panic() {
        do_panic();
    }

    #[test]
    #[ignore]
    fn expensive() {

    }
}