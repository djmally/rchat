use std::io;

pub fn read_message() -> String {
    let mut message = String::new();
    io::stdin().read_line(&mut message)
               .ok()
               .expect("Unable to read from console!");
    message
}
