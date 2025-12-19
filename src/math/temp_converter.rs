use std::io;

fn main() {
    println!("Temperature Converter");
    println!("1) Celsius -> Fahrenheit");
    println!("2) Fahrenheit -> Celsius");
    println!("Choose an option:");

    let mut choice = String::new();
    io::stdin().read_line(&mut choice).unwrap();

    let choice = choice.trim();

    if choice == "1" {
        let celsius = read_number("Enter the temperature in Celsius:");
        let fahrenheit = celsius * 9.0 / 5.0 + 32.0;
        println!("{celsius}°C = {fahrenheit}°F");
    } else if choice == "2" {
        let fahrenheit = read_number("Enter the temperature in Fahrenheit:");
        let celsius = (fahrenheit - 32.0) * 5.0 / 9.0;
        println!("{fahrenheit}°F = {celsius}°C");
    } else {
        println!("Invalid option!");
    }
}

fn read_number(msg: &str) -> f64 {
    println!("{msg}");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().parse::<f64>().expect("Invalid number")
}
