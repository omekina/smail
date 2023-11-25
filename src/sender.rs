use crate::config::loader::{ConfigItem, search_key_in_config};
use crate::connection::smtp;
use crate::io::output;
use crate::mail_file::loader::{MailField, search_key_in_mail_file, load_mail_file};


/**
Send `.smail` files identified by command line arguments.
*/
pub fn send(arguments: &Vec<String>, configuration: &Vec<ConfigItem>) -> Option<()> {
    if arguments.len() < 3 {
        output::warning("No .smail file to send was specified");
        return None;
    }

    /* Fetch the .smail files. */
    let mut to_send: Vec<Vec<MailField>> = Vec::new();
    for current_index in 2..arguments.len() {
        to_send.push(load_mail_file(arguments.get(current_index)?)?);
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

    /* Send mails. */
    for current_mail in to_send {
        let mut mail_body = search_key_in_config(configuration, "=")?;
        for current_mail_field in &current_mail {
            mail_body = mail_body.replace(&(String::from("{") + &current_mail_field.flag + "}"), &current_mail_field.content);
        }
        smtp::send_mail(
            &mut stream,
            &search_key_in_mail_file(&current_mail, "FROM-NAME")?,
            &search_key_in_mail_file(&current_mail, "FROM-MAIL")?,
            &search_key_in_mail_file(&current_mail, "TO")?.split("\n").collect::<Vec<&str>>(),
            &search_key_in_mail_file(&current_mail, "SUBJECT")?,
            &mail_body,
        )?;
    }

    smtp::close(stream);

    return Some(());
}
