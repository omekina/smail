use std::io::Write;
use std::net::TcpStream;
use base64::Engine;
use native_tls::TlsStream;
use super::socket_handler;
use crate::io::output;
use base64::engine::general_purpose;
use crate::sender::Contact;


/**
Perform E SMTP handshake returning TlsStream on success.
*/
pub fn handshake(host: &String, port: &String) -> Option<TlsStream<TcpStream>> {

    /* Create connection to specified host and port. */
    let mut stream = socket_handler::make_connection(&host, &port)?;

    /* Wait for the server welcome message. */
    let received = socket_handler::receive_raw(&mut stream)?;
    if received.chars().take(3).collect::<String>() != "220" {
        output::error("Server was not ready for handshake");
        return None;
    }

    /* Inform the server that we want to connect via TLS. */
    stream.write_all("STARTTLS\r\n".as_bytes()).ok()?;
    let received = socket_handler::receive_raw(&mut stream)?;
    if received.chars().take(3).collect::<String>() != "220" {
        output::error("Server did not accept TLS handshake");
        return None;
    }

    /* Initiate TLS. */
    let mut stream = socket_handler::wrap_connection(stream, &host)?;

    /* Send client hello. */
    stream.write_all("EHLO [::1]\r\n".as_bytes()).ok()?;
    let received = socket_handler::receive_tls(&mut stream)?;
    if received.chars().take(3).collect::<String>() != "250" {
        output::error("Server did not accept the welcome message.");
        return None;
    }

    output::success("Completed ESMTP handshake");

    return Some(stream);
}


/**
SMTP login authentication.

Returns Some(()) on success and None on fail.
*/
pub fn auth_login(
    mut stream: &mut TlsStream<TcpStream>,
    username: &String,
    password: &String
) -> Option<()> {

    /* Send login command. */
    stream.write_all(&"AUTH LOGIN\r\n".as_bytes()).ok()?;
    let received = socket_handler::receive_tls(&mut stream)?;
    if received != "334 VXNlcm5hbWU6\r\n" {
        output::error("Authentication handshake failed");
        return None;
    }

    /* Send username. */
    let username_encoded = general_purpose::STANDARD.encode(username);
    stream.write_all((username_encoded + "\r\n").as_bytes()).ok()?;
    let received = socket_handler::receive_tls(&mut stream)?;
    if received != "334 UGFzc3dvcmQ6\r\n" {
        output::error("Authentication failed after username was sent");
        return None;
    }

    /* Send password. */
    let password_encoded = general_purpose::STANDARD.encode(password);
    stream.write_all((password_encoded + "\r\n").as_bytes()).ok()?;
    let received = socket_handler::receive_tls(&mut stream)?;
    if received.chars().take(3).collect::<String>() != "235" {
        output::error("Authentication failed:");
        println!("{:?}", received);
        return None;
    }

    output::success("Logged in");

    return Some(());
}


/**
This function checks if a field is valid http header field.

Returns True if the payload should be okay, False if not.
*/
fn check_email_payload(payload: &String) -> bool {
    for current_char in payload.chars() {
        match current_char {
            '\r' => return false,
            '\n' => return false,
            _ => {},
        }
    }
    return true;
}


/**
Escape e-mail body.
*/
fn escape_email_body(body: &String) -> Option<String> {

    /* Carriage returns are not allowed and should not be used with Linux. */
    if body.contains("\r") {
        return None;
    }

    /* Process the e-mail body into lines. */
    let mut result: String = String::new();
    let body_lines = body.split("\n");
    for current_line in body_lines {

        /* Allow blank lines. */
        if current_line.len() == 0 {
            result += "\r\n";
            continue;
        }

        /* Escape lines beginning with dot. */
        if current_line.chars().nth(0)? == '.' {
            result += ".";
        }

        /* Append current line to the result. */
        result += &(String::from(current_line) + "\r\n");
    }

    return Some(result);
}


/**
Prepare data for transmission.

Name from, e-mail from and to as well as subject should be already checked.
*/
fn format_data(
    email_from: Contact,
    emails_to: Vec<Contact>,
    subject: &String,
    html_content: &String
) -> Option<String> {
    /* Subject check. */
    if !check_email_payload(&subject) {
        output::error("Subject contains invalid characters");
        return None;
    }

    let mut result = String::new();

    /* Add some general information. */
    result += "User-Agent: smail (https://github.com/omekina/smail)\r\nContent-Type: text/html; charset=UTF-8\r\nContent-Transfer-Encoding: 8bit\r\n";

    /* Add sender info. */
    result += &(String::from("From: ") + &match email_from {
        Contact::Named(value) => value.name.clone() + " <" + &value.email + ">",
        Contact::Email(value) => value,
    } + "\r\n");

    /* Add recipient info. */
    let mut recipients: String = String::new();
    for current_recipient in emails_to {
        if recipients.len() != 0 { recipients += ", "; }
        recipients += &match current_recipient {
            Contact::Named(value) => value.name + " <" + &value.email + ">",
            Contact::Email(value) => value,
        };
    }
    result += &(String::from("To: ") + &recipients + "\r\n");

    /* Add subject and end head. */
    result += &(String::from("Subject: ") + &subject + "\r\n\r\n");

    /* Add e-mail body and data end. */
    result += &(escape_email_body(html_content)? + "\r\n.\r\n");

    return Some(result);
}


/**
Send mail in open and authenticated state.
*/
pub fn send_mail(
    stream: &mut TlsStream<TcpStream>,
    email_from: Contact,
    emails_to: Vec<Contact>,
    subject: &String,
    html_content: &String,
) -> Option<()> {

    /* Send from e-mail. */
    stream.write_all((String::from("MAIL FROM: <") + match &email_from {
        Contact::Email(value) => value,
        Contact::Named(value) => &value.email,
    } + ">\r\n").as_bytes()).ok()?;
    let received = socket_handler::receive_tls(stream)?;
    if received.chars().take(3).collect::<String>() != "250" {
        output::error("Server did not accept the sender e-mail");
        return None;
    }

    /* Send recipient e-mail(s). */
    for current_recipient in &emails_to {
        stream.write_all((String::from("RCPT TO: <") + &match current_recipient {
            Contact::Email(value) => value,
            Contact::Named(value) => &value.email,
        } + ">\r\n").as_bytes()).ok()?;
        let received = socket_handler::receive_tls(stream)?;
        if received.chars().take(3).collect::<String>() != "250" {
            output::error("Server did not accept recipient e-mail(s)");
            return None;
        }
    }

    /* Begin data transmission. */
    stream.write_all("DATA\r\n".as_bytes()).ok()?;
    let received = socket_handler::receive_tls(stream)?;
    if received.chars().take(3).collect::<String>() != "354" {
        output::error("Server pre-rejected e-mail data");
        return None;
    }

    /* Send data. */
    stream.write_all(format_data(email_from, emails_to, subject, html_content)?.as_bytes()).ok()?;
    let received = socket_handler::receive_tls(stream)?;
    if received.chars().take(3).collect::<String>() != "250" {
        output::error("Server rejected the e-mail content");
        return None;
    }

    output::success("\nMail sent:");
    println!("{}", subject);

    return Some(());
}


/**
Gracefully close the connection and consume the TlsStream.
*/
pub fn close(mut stream: TlsStream<TcpStream>) {
    match stream.write_all("QUIT\r\n".as_bytes()) {
        Ok(_) => {},
        Err(_) => return,
    };
    match socket_handler::receive_tls(&mut stream) {
        Some(_) => {},
        None => return,
    };
    let _ = stream.shutdown();
}
