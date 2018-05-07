use std::thread;
std::time::Duration;

fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(
        simulated_user_specified_value,
        simulated_random_number
    );
}

fn simulated_expensive_calculation(intensity: u32) -> u32 {

    thread::sleep(Duration::from_secs(2));
    intensity
}

fn generate_workout(intensity: u32, random_number: u32) {
    if intensity < 25 {
        println!("today, do {} push ups", 
                 simulated_expensive_calculation(intensity));
        println!("next, do {} situps",
                 simulated_expensive_calculation(intensity));
    } else {
        if random_number == 3 {
            println!("take a break today");
        } else {
            println!("today, run for {} mins",
                     simulated_expensive_calculation(intensity));
        }
    }
}
