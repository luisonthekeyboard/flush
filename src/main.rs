use std::ffi::CString;
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
        io::stdout().write(line.as_bytes()).unwrap();
        //        println!("->{}", line);

        // let args = rush_split_line(&line);
        //  println!("->{}", args);
    }
}

// fn rush_split_line(line: &String) -> [String] {

// }

fn rush_read_line() -> CString {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => return CString::new(input.as_bytes()).unwrap(),
        Err(_) => return CString::new("error").unwrap(),
    }
}
