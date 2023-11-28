use crate::config::loader::{ConfigItem, search_key_in_config};
use crate::connection::smtp;
use crate::io::output;
use std::io::{Read, stdin};
use crate::mail_file::loader::{MailField, search_key_in_mail_file, load_mail_file, parse_mail_file};


pub struct NamedContact {
    pub name: String,
    pub email: String,
}


pub enum Contact {
    Email(String),
    Named(NamedContact),
}


/**
Send `.smail` files identified by command line arguments.
*/
pub fn send(
    arguments: &Vec<String>,
    configuration: &Vec<ConfigItem>,
    stdin_override: &String
) -> Option<()> {
    let mut to_send: Vec<Vec<MailField>> = Vec::new();

    /* Load and parse files/stdin. */
    if arguments.len() > 0 {
        for current_index in 0..arguments.len() {
            to_send.push(load_mail_file(arguments.get(current_index)?)?);
        }
    } else if stdin_override.len() != 0 {
        output::warning("Overriding stdin with flag data");
        to_send.push(parse_mail_file(&stdin_override.replace("\\n", "\n"))?);
    } else {
        output::warning("Reading from stdin...");
        let mut input = String::new();
        let stdin = stdin();
        let mut handle = stdin.lock();
        match handle.read_to_string(&mut input) {
            Ok(_) => {},
            Err(_) => { output::error("Failed reading from stdin"); return None; }
        };
        to_send.push(parse_mail_file(&input)?);
    }

    /* Initiate connection to the server. */
    let mut stream = smtp::handshake(
        &search_key_in_config(configuration, "host")?,
        &search_key_in_config(configuration, "port")?,
    )?;

    /* Authenticate. */
    smtp::auth_login(
        &mut stream,
        &search_key_in_config(configuration, "username")?,
        &search_key_in_config(configuration, "password")?,
    )?;

    /* Send mail(s). */
    for current_mail in to_send {
        let mut mail_body = search_key_in_config(configuration, "=")?;
        for current_mail_field in &current_mail {
            mail_body = mail_body.replace(&(String::from("{") + &current_mail_field.flag + "}"), &current_mail_field.content);
        }
        smtp::send_mail(
            &mut stream,
            parse_single(&search_key_in_mail_file(&current_mail, "FROM")?)?,
            parse_multiple(&search_key_in_mail_file(&current_mail, "TO")?)?,
            &search_key_in_mail_file(&current_mail, "SUBJECT")?,
            &mail_body,
        )?;
    }

    smtp::close(stream);

    return Some(());
}


/**
True if the payload should be okay, False if not.
 */
fn check_email_payload(payload: &String) -> bool {
    for current_char in payload.chars() {
        match current_char {
            '<' => return false,
            '>' => return false,
            '\r' => return false,
            '\n' => return false,
            ':' => return false,
            ',' => return false,
            _ => {},
        }
    }
    return true;
}


/**
Check if provided string should be a valid e-mail.
*/
pub fn check_email(payload: &str) -> bool {
    let at_split: Vec<&str> = payload.split("@").collect();
    if at_split.len() != 2 || at_split[0].len() == 0 || at_split[1].len() == 0 {
        return false;
    }
    let domain_split: Vec<&str> = at_split[1].split(".").collect();
    if domain_split.len() < 2 {
        return false;
    }
    for domain_part in domain_split {
        if domain_part.len() == 0 {
            return false;
        }
    }
    return true;
}


/**
Parse single line contact.

Example: "John Doe;john@doe.domain"
*/
pub fn parse_single(subject: &String) -> Option<Contact> {
    let sender_split: Vec<&str> = subject.split(";").collect();
    if !check_email_payload(subject) {
        output::error("Invalid contact:");
        println!("{:?}", subject);
        return None;
    }
    if sender_split.len() == 1 {
        if !check_email(subject) {
            output::error("Invalid contact:");
            println!("{:?}", subject);
            return None;
        }
        return Some(Contact::Email(subject.clone()));
    }
    if sender_split.len() != 2 || sender_split[0].len() == 0 || sender_split[1].len() == 0 {
        output::error("Invalid contact:");
        println!("{:?}", subject);
        return None;
    }
    if !check_email(sender_split[1]) {
        output::error("Invalid contact:");
        println!("{:?}", subject);
        return None;
    }
    return Some(Contact::Named(NamedContact {
        name: sender_split[0].to_string(),
        email: sender_split[1].to_string(),
    }));
}


/**
Parse multi line contact.

Example: "John Doe;john@doe.domain\nAlice Smith;alice@smith.domain"
 */
pub fn parse_multiple(recipients_raw: &String) -> Option<Vec<Contact>> {
    let recipients_split: Vec<&str> = recipients_raw.split("\n").collect();
    let mut recipients: Vec<Contact> = Vec::new();
    for recipients_line in recipients_split {
        recipients.push(parse_single(&recipients_line.to_string())?);
    }
    return Some(recipients);
}
