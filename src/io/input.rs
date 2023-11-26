extern crate rpassword;
    
use rpassword::read_password;
use std::io::{stdin, stdout, Write};

pub fn read_items(prompts: Vec<Vec<&str>>) -> Option<Vec<String>> {
    let mut result = Vec::new();
    for prompt in prompts {
        let mut line = String::new();

        if prompt[1] == "pass" {
            print!("{} (will be hidden): ", prompt[0]);
            stdout().flush().unwrap();

            line = read_password().unwrap();
        } else {
            print!("{}: ", prompt[0]);
            stdout().flush().unwrap();

            match stdin().read_line(&mut line) {
                Ok(_) => {},
                Err(_) => return None,
            };
        }

        result.push(line);
    }
    return Some(result);
}
