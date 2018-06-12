use std::io;

fn main() {
    // Load config files, if any.

    // Run command loop.
    rush_loop();

    // Perform any shutdown/cleanup.
}

fn rush_loop() {
    let mut input = String::new();

    loop {
        match io::stdin().read_line(&mut input) {
            Ok(n) => {
                println!("{} bytes read", n);
                println!("{}", input);
            }
            Err(error) => println!("error: {}", error),
        }
    }
}
