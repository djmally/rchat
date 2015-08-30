use std::io;

pub fn read_stdin() -> String {
    let mut message = String::new();
    io::stdin().read_line(&mut message)
               .ok()
               .expect("Unable to read from console!");
    message
}
