extern crate rpassword;


use rpassword::read_password;
use std::io::{stdin, stdout, Write};


/**
Input item for command line inputs with prompts.
*/
pub enum InputItem {
    Normal(&'static str),
    Hidden(&'static str),
}


/**
Read specified items to the command line.
*/
pub fn read_items(prompts: Vec<InputItem>) -> Option<Vec<String>> {
    let mut result = Vec::new();

    for current_input in prompts {
        let mut line = String::new();

        print!("{}: ", match current_input {
            InputItem::Normal(value) => value,
            InputItem::Hidden(value) => value,
        });
        stdout().flush().unwrap();

        match current_input {
            InputItem::Normal(_) => { let _ = stdin().read_line(&mut line).ok()?; },
            InputItem::Hidden(_) => line = read_password().ok()?,
        };

        result.push(line);
    }

    return Some(result);
}
