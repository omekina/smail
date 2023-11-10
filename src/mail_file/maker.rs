use std::{fs::write};
use crate::io;


const DEFAULT_MAIL: &str = "==SUBJECT
Test e-mail
==FROM
Alice <alice@nonexistent.domain>
==TO
Bob <bob@nonexistent.domain>
==STYLE
body {
    font-family: Arial;
    color: white;
    background-color: black;
}
==BODY
<h1>Hello, world!</h1>
<p>This is a test mail_file from SMAIL.</p>
<p>You can find SMAIL source code
    <a href=\"https://github.com/omekina/smail\">here</a>
    if you are interested. &#128522;
</p>
";


/**
Create .smail file(s) specified by console arguments.
*/
pub fn create(arguments: &Vec<String>) -> bool {
    if arguments.len() < 3 {
        io::output::warning("No desired files were specified.");
        return false;
    }
    for argument_id in 2..arguments.len() {
        let current_file = arguments.get(argument_id).unwrap().clone() + ".smail";
        match write(&current_file, DEFAULT_MAIL) {
            Ok(_) => { println!("Created file '{}'", current_file); },
            Err(_) => { println!("File '{}' could not be created.", current_file); },
        };
    }
    return true;
}
