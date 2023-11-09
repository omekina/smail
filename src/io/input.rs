use std::io::{stdin, stdout, Write};

pub fn read_items(prompts: Vec<&str>) -> Option<Vec<String>> {
    let mut result = Vec::new();
    for prompt in prompts {
        let mut line = String::new();
        print!("{}: ", prompt);
        stdout().flush().unwrap();
        match stdin().read_line(&mut line) {
            Ok(_) => {},
            Err(_) => return None,
        };
        result.push(line);
    }
    return Some(result);
}
