use std::io;

fn main() {
    println!("Type your nth Fibonacci number");

    let mut nth: String = String::new();

    io::stdin().read_line(&mut nth).expect("Failed to read line");

    let nth: u64 = nth.trim().parse().expect("Type a number!");

    let mut fib_number: u64 = 0;

    if nth > 1 {
        let mut n0: u64 = 0;
        let mut n1: u64 = 1;
        let mut count: u64 = 0;
        while count < nth - 1{
            fib_number = n0 + n1;
            n0 = n1;
            n1 = fib_number;

            count += 1;
        }
    } else if nth == 1 {
        fib_number = 1;
    }

    println!("The {}th Fibonacci number is {}", nth, fib_number);
}
