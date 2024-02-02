use std::io;
use std::io::Write;
// Program that takes in names, then picks a random winner
// Inputs: names
// Process: pick random
// Output: the name

fn get_input<T: std::str::FromStr>(prompt: &str) -> T {
    loop {
        print!("{}", prompt);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        match input.trim().parse() {
            Ok(value) => break value,
            Err(_) => println!("Invalid input. Please try again."),
        }
    }
}

fn main() {
    // Initialize empty vector
    let mut names: Vec<String> = vec![];

    // loop
    loop {
        // get input_name
        let name: String = get_input("Enter a name: ");
        // if input is blank, stop loop.
        if name == "" {
            break;
        }

        // if not blank, push to vector.
        names.push(name);
    }

    print!("Hello world!");

    // select random in vector
    // print, "The winner is... {}"
}
