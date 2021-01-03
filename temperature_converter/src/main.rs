use std::io;

fn main() {
    let selection = conversion_selection();
    let temperature = temperature_selection();

    if selection == 1 {
        println!("{}°C -> {}°F", temperature, celsius_to_fahrenheit(temperature));
    } else if selection == 2 {
        println!("{}°F -> {}°C", temperature, fahrenheit_to_celsius(temperature));
    }
}

fn temperature_selection() -> f32 {
    return loop {
        println!("Temperature: ");
        let mut temperature = String::new();

        io::stdin()
            .read_line(&mut temperature)
            .expect("Failed to read line");

        match temperature.trim().parse::<f32>() {
            Ok(n) => break n,
            Err(_) => continue
        };
    };
}

fn conversion_selection() -> u32 {
    return loop {
        println!("1. Celsius to Fahrenheit");
        println!("2. Fahrenheit to Celsius");
        println!("Select: ");

        let mut temp = String::new();

        io::stdin()
            .read_line(&mut temp)
            .expect("Failed to read line");

        let temp: u32 = match temp.trim().parse() {
            Ok(n) => n,
            Err(_) => continue
        };

        if [1, 2].contains(&temp) { break temp; }
    };
}

fn celsius_to_fahrenheit(t: f32) -> f32 {
    t * 1.8 + 32.0
}

fn fahrenheit_to_celsius(t: f32) -> f32 {
    (t - 32.0) / 1.8
}