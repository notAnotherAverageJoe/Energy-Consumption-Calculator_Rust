use std::env;
use std::process;
use std::io;

fn main() {
    let args: Vec<String> = env::args().collect();
    let power: f32;
    let time: f32;
    let cost_per_kwh: f32;

    if args.len() != 4 {
        println!("Usage: {} <power_consumption_kw> <running_time_hours> <cost_per_kwh>", args[0]);

        // Prompt user for input
        power = prompt_input("Enter power consumption (kW): ");
        time = prompt_input("Enter running time (hours): ");
        cost_per_kwh = prompt_input("Enter cost per kWh (USD): ");
    } else {
        power = match args[1].parse() {
            Ok(value) if value >= 0.0 => value,
            _ => {
                eprintln!("Invalid power consumption: must be a non-negative number");
                process::exit(1);
            }
        };
        time = match args[2].parse() {
            Ok(value) if value >= 0.0 => value,
            _ => {
                eprintln!("Invalid running time: must be a non-negative number");
                process::exit(1);
            }
        };
        cost_per_kwh = match args[3].parse() {
            Ok(value) if value >= 0.0 => value,
            _ => {
                eprintln!("Invalid cost per kWh: must be a non-negative number");
                process::exit(1);
            }
        };
    }

    let energy = calculate_energy_consumption(power, time);
    let cost = calculate_cost(energy, cost_per_kwh);
    let monthly = calculate_monthly(cost);

    println!("Energy Consumption: {:.2} kWh", energy);
    println!("Cost: ${:.2}", cost);
    println!("Monthly cost: ${:.2}", monthly);
}

fn prompt_input(prompt: &str) -> f32 {
    loop {
        println!("{}", prompt);
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        match input.trim().parse() {
            Ok(value) if value >= 0.0 => return value,
            _ => println!("Invalid input: must be a non-negative number"),
        }
    }
}

fn calculate_energy_consumption(power: f32, time: f32) -> f32 {
    power * time
}

fn calculate_cost(energy: f32, cost_per_kwh: f32) -> f32 {
    energy * cost_per_kwh
}

fn calculate_monthly(cost: f32) -> f32 {
    cost * 30.0 // Assuming 30 days in a month for simplicity
}
