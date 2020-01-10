use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

static LOREM_IPSUM: &str =
    "Lorem ipsum dolor sit amet, consectetur adipisicing elit, sed do eiusmod
tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam,
quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo
consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse
cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non
proident, sunt in culpa qui officia deserunt mollit anim id est laborum.
";

fn main() {
    // Create a path to the desired file
    let in_path = Path::new("resources/lorem ipsum.txt");
    let in_display = in_path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(&in_path) {
        // The `description` method of `io::Error` returns a string that
        // describes the error
        Err(why) => panic!("couldn't open {}: {}", in_display,
                                                   why.description()),
        Ok(file) => file,
    };

    // Read the file contents into a string, returns `io::Result<usize>`
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", in_display,
                                                   why.description()),
        Ok(_) => print!("{} contains:\n{}", in_display, s),
    }

    // `file` goes out of scope, and the "hello.txt" file gets closed




    let out_path = Path::new("lorem_ipsum.txt");
    let out_display = out_path.display();

    // Open a file in write-only mode, returns `io::Result<File>`
    let mut file = match File::create(&out_path) {
        Err(why) => panic!("couldn't create {}: {}", out_display, why.description()),
        Ok(file) => file,
    };

    // Write the `LOREM_IPSUM` string to `file`, returns `io::Result<()>`
    match file.write_all(LOREM_IPSUM.as_bytes()) {
        Err(why) => panic!("couldn't write to {}: {}", out_display, why.description()),
        Ok(_) => println!("successfully wrote to {}", out_display),
    }
}