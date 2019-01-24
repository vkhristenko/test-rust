pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if (value < 1 || value > 100) {
            panic!("guess value must be between 1 and 100. we got {}", value);
        }

        Guess { value }
    }
}

#[derive(Debug)]
pub struct Rectangle {
    length: i32,
    width: i32,
}
    
impl Rectangle {
    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.length > other.length && self.width > other.width
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn another() {
        panic!("testing a test");
    }

    use super::*;

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle { length: 8, width: 7 };
        let smaller = Rectangle { length: 7, width: 1 };
        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle { length: 8, width: 7 };
        let smaller = Rectangle { length: 7, width: 1 };
        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    #[should_panic(expected = "guess value must be between 1 and 100")]
    fn greater_than_100() {
        Guess::new(200);
    }

    #[test]
    fn test_result_struct() -> Result<(), String> {
        if 2 + 2 == 5 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }
}
