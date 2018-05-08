use std::thread;
use std::time::Duration;

fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(
        simulated_user_specified_value,
        simulated_random_number
    );

    let x = 4;
    let equal_to_x = |z| z == x;

    let y = 4;
    assert!(equal_to_x(y));
}

fn simulated_expensive_calculation(intensity: u32) -> u32 {

    thread::sleep(Duration::from_secs(2));
    intensity
}

fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_closure = Cacher::new(|num| {
        println!("calc...");

        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!("today, do {} push ups", 
                 expensive_closure.value(intensity));
          //       simulated_expensive_calculation(intensity));
        println!("next, do {} situps",
                 expensive_closure.value(intensity));
//                 simulated_expensive_calculation(intensity));
    } else {
        if random_number == 3 {
            println!("take a break today");
        } else {
            println!("today, run for {} mins",
                     expensive_closure.value(intensity));
  //                   simulated_expensive_calculation(intensity));
        }
    }
}

fn test_closure() {
    fn add_one_v1(x: u32) -> u32 { x+1 }
    let add_one_v2 = |x: u32| -> u32 { x+1 };
    let add_one_v3 = |x: u32| {x + 1};
    let add_one_v4 = |x: u32| x+1;

    let example_closure = |x| x;
    example_closure(5);
    example_closure(100);
}

struct Cacher<T> 
where T: Fn(u32) -> u32 {
    calculation: T,
    value: Option<u32>,
}

impl<T> Cacher<T>
where T: Fn(u32) -> u32 {
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation, 
            value: None,
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

#[test]
fn call_with_different_values() {
    let mut c = Cacher::new(|a| a);

    let v1 = c.value(1);
    let v2 = c.value(2);

    assert_eq!(v2, 2);
}

}
