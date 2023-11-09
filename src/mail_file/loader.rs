use std::fs::read_to_string;
use crate::io::output;


/**
E-Mail file fields. Delimited by field flags as defined in documentation.
*/
pub struct MailField {
    pub flag: String,
    pub content: String,
}


/**
Load e-mail file into fields.
*/
pub fn load_mail_file(filepath: &String) -> Option<Vec<MailField>> {
    let contents: String = match read_to_string(filepath) {
        Ok(value) => value,
        Err(_) => {
            output::error("Could not read from SMAIL file.");
            println!("Maybe you forgot to 'smail create'?");
            return None;
        },
    };

    return parse_mail_file(&contents);
}


/**
Main e-mail file parsing function.
*/
fn parse_mail_file(mail_file_contents: &String) -> Option<Vec<MailField>> {

    let mail_file_lines = mail_file_contents.split("\n");
    let mut result: Vec<MailField> = Vec::new();
    let mut flag: String = String::new();
    let mut temp: String = String::new();

    for current_line in mail_file_lines {
        if current_line.len() > 2 && current_line[0..2].to_string() == "==" {
            if flag.len() > 0 { result.push(MailField { flag, content: temp }); }
            temp = String::new();
            flag = String::from(&current_line[3..]);
            continue;
        }
        if temp.len() > 0 {
            temp += "\n";
        }
        temp += current_line;
    }

    if result.len() == 0 {
        output::error("No fields were parsed.");
        return None;
    }

    return Some(result);
}
