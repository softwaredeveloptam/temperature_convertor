use std::io;

fn main() {
    loop {    
        
        println!("Please choose an option: ");
        println!("1. Convert Fahrenheit to Celsius.");
        println!("2. Convert Celsius to Fahrenheit.");
        println!("3. Quit");

        let mut input = String::new();
        let mut temp = String::new();

        io::stdin().read_line(&mut input).expect("Failed to read line");

        let input: u8 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input. Must be a number\n"); 
                continue 
            },
        };

        if input != 3 {
            println!("Please enter the temperature:");    
            io::stdin().read_line(&mut temp).expect("Failed to read line");
        }

        if input == 1 {
            let temp: f32 = match temp.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Invalid input. Temperature must be a number\n");
                    continue
                }
            };
            let f = convert_to_celsuis(temp);
            println!("The temperature in Celsuis is: {}", f);
            break
        } else if input == 2 {
            let temp: f32 = match temp.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Invalid input. Must be a floating number\n");
                    continue
                }
            };
            let c = convert_to_fahrenheit(temp);
            println!("The temperature in Fahrenheit is: {}", c);
            break
        } else if input == 3 {
            break
        } else {
            continue;
        }
    }
}

fn convert_to_celsuis (x: f32) -> f32 {
    let celsuis: f32 = x - 32.0;
    let celsuis = celsuis * 0.55555;
    celsuis
}

fn convert_to_fahrenheit (x: f32) -> f32 {
    let fahrenheit: f32 = x * 1.8;
    let fahrenheit = fahrenheit + 32.0;
    fahrenheit
}