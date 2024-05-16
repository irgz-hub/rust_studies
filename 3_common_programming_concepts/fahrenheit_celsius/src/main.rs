use std::io;

fn main() {
    println!("Type FC to convert Fahrenheit to Celsius or \
    CF to convert Celsius to Fahrenheit!");

    loop {
        let mut option_convert: String = String::new();
    
        io::stdin()
            .read_line(&mut option_convert)
            .expect("Failed to read line");

        let option_convert = option_convert.trim().to_uppercase();

        if option_convert == "FC" {
            println!("Type your Fahrenheit number!");

            let fahrenheit: f32 = read_number();

            let celsius: f32 = (fahrenheit - 32.0) * 5.0 / 9.0;

            println!("{} Farenheit is equal to {} Celsius.", fahrenheit, celsius);

            break;

        } else if option_convert == "CF" {
            println!("Type your Celsius number!");

            let celsius: f32 = read_number();

            let fahrenheit: f32 = celsius*9./5. + 32.;

            println!("{} Celsius is equal to {} Fahrenheit.", celsius, fahrenheit);

            break;

        }
    }
}

fn read_number() -> f32 {
    let mut number: String = String::new();

    io::stdin()
        .read_line(&mut number)
        .expect("Failed to read line");

    let number: f32 = number.trim().parse().expect("Please type a number");

    number
}