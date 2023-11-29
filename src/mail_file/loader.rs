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
    let possiblepaths = vec![
        filepath.clone(),
        (filepath.clone() + ".smail")
    ];

    for path in possiblepaths {
        let contents = read_to_string(path);

        if contents.is_ok() {
            return parse_mail_file(&contents.ok()?);
        }
    }

    output::error("Could not read from SMAIL file.");
    println!("Maybe you forgot to 'smail create'?");
    return None;
}


/**
Main e-mail file parsing function.
*/
pub fn parse_mail_file(mail_file_contents: &String) -> Option<Vec<MailField>> {

    let mail_file_lines = mail_file_contents.split("\n");
    let mut result: Vec<MailField> = Vec::new();
    let mut flag: String = String::new();
    let mut temp: String = String::new();

    for current_line in mail_file_lines {
        if current_line.len() > 2 && current_line.chars().take(2).collect::<String>() == "==" {
            if flag.len() > 0 { result.push(MailField { flag, content: temp }); }
            temp = String::new();
            flag = String::from(&current_line[2..]);
            continue;
        }
        if temp.len() > 0 {
            temp += "\n";
        }
        if current_line.chars().take(1).collect::<String>() == "\\" {
            if current_line.len() == 1 { continue; }
            temp += &current_line[1..];
            continue;
        }
        temp += current_line;
    }

    if flag.len() > 0 && temp.len() > 0 {
        result.push(MailField { flag, content: temp });
    }

    if result.len() == 0 {
        output::error("No fields were parsed.");
        return None;
    }

    return Some(result);
}


/**
Search given array of MailFields and return value with specified key.
 */
pub fn search_key_in_mail_file(mail_file_contents: &Vec<MailField>, key: &str) -> Option<String> {
    for current_item in mail_file_contents {
        if current_item.flag == key {
            return Some(current_item.content.clone());
        }
    }
    output::error(".smail file entry was not found:");
    println!("{}", key);
    return None;
}
