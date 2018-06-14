use std::io::{self, Write};

fn main() {
    // Load config files, if any.

    // Run command loop.
    rush_loop();

    // Perform any shutdown/cleanup.
}

fn rush_loop() {
    loop {
        print!(">>>> ");
        io::stdout().flush().unwrap();

        let line = rush_read_line();
        println!("->{}", line);

        // let args = rush_split_line(&line);
        //  println!("->{}", args);
    }
}

// fn rush_split_line(line: &String) -> [String] {

// }

fn rush_read_line() -> String {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => return input,
        Err(_) => return String::from("error"),
    }
}
