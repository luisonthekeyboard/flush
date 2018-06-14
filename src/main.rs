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
        let mut input = String::new();
        print!(">>>> ");
        io::stdout().flush().unwrap();

        let line = rush_read_line(&mut input);
        io::stdout().write(line).unwrap();

        let args = rush_split_line(&line);
        println!(
            "-> '{}', '{}'",
            String::from_utf8_lossy(args.0.as_bytes()),
            String::from_utf8_lossy(args.1.as_bytes())
        );
    }
}

fn rush_split_line(line: &[u8]) -> (CString, CString) {
    for (i, &item) in line.iter().enumerate() {
        if item == b' ' {
            unsafe {
                return (
                    CString::from_vec_unchecked(line[0..i].to_vec()),
                    CString::from_vec_unchecked(line[i..line.len()].to_vec()),
                );
            }
        }
    }

    unsafe {
        return (
            CString::from_vec_unchecked(line.to_vec()),
            CString::new("").unwrap(),
        );
    }
}

fn rush_read_line(input: &mut String) -> &[u8] {
    match io::stdin().read_line(input) {
        Ok(_) => return input.as_bytes(),
        Err(_) => return "error".as_bytes(),
    }
}
