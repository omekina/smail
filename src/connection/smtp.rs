use std::io::Write;
use std::net::TcpStream;
use base64::Engine;
use native_tls::TlsStream;
use super::socket_handler;
use crate::io::output;
use base64::engine::general_purpose;


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


/* Example communication:

= TCP handshake =
< b'220 nonexistent-mail.mekina.cz Very good imaginary mail server\r\n'
> b'STARTTLS\r\n'
< b'220 2.0.0 Go ahead\r\n'
= TLS handshake =
> b'EHLO [::1]\r\n'
< b'250-nonexistent-mail.mekina.cz\r\n250 SOME EXTENSION\r\n250 AUTH LOGIN\r\n'
> b'AUTH LOGIN\r\n'
< b'334 VXNlcm5hbWU6\r\n' (meaning 'Username:')
> USERNAME
< b'334 UGFzc3dvcmQ6\r\n' (meaning 'Password:')
> PASSWORD
< b'235 2.7.0 Authentication successful\r\n'
> b'MAIL FROM:<ondrej@mekina.cz>\r\n'
< b'250 2.1.0 Ok\r\n'
> b'RCPT TO:<ondrej@mekina.cz>\r\n'
< b'250 2.1.5 Ok\r\n'
> b'DATA\r\n'
< b'354 End data with <CR><LF>.<CR><LF>\r\n'
> b'Hello, world!\r\n'
> b'\r\n.\r\n'
< b'250 2.0.0 Ok: queued as somethingnice\r\n'
> b'QUIT\r\n'
< b'221 2.0.0 Have a nice one\r\n'

 */
