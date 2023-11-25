use std::io::Read;
use std::net::TcpStream;
use std::time::Duration;
use native_tls::{TlsConnector, TlsStream};
use crate::io::output;


/**
Maximum number of iterations to perform when getting data from stream.
*/
const MAX_INCOMING_ITERATIONS: u16 = 1;


/**
Create a TCP connection. The TCP connection will timeout all events after 3 seconds.
*/
pub fn make_connection(host: &String, port: &String) -> Option<TcpStream> {
    let mut target = host.clone() + ":";
    target.push_str(port);
    return match TcpStream::connect(&target) {
        Ok(stream) => {
            output::success("Connected");
            stream.set_read_timeout(Option::from(Duration::from_secs(3))).unwrap();
            stream.set_write_timeout(Option::from(Duration::from_secs(3))).unwrap();
            Some(stream)
        },
        Err(_) => {
            output::error("Could not connect to destination:");
            println!("{}", target);
            None
        },
    };
}


/**
Wrap existing TCP connection in TLS/SSL. This will perform a handshake and return the wrapper.

Note that this will consume the TcpStream.
*/
pub fn wrap_connection(raw_stream: TcpStream, host: &String) -> Option<TlsStream<TcpStream>> {
    let connector = match TlsConnector::new() {
        Ok(value) => value,
        Err(_) => { return None; }
    };
    let stream = match connector.connect(host, raw_stream) {
        Ok(value) => value,
        Err(_) => { return None; }
    };
    return Some(stream);
}


macro_rules! receive_from_stream {
    ($stream:ident) => {
        let buffer: &mut [u8] = &mut [0; 1024];
        let mut result: String = String::new();
        for _ in 0..MAX_INCOMING_ITERATIONS {
            let received_length = match $stream.read(buffer) {
                Ok(value) => value,
                Err(_) => { output::error("Timeout when getting data"); return None; },
            };
            result.push_str(&String::from_utf8_lossy(&buffer[0..received_length]));
            if received_length == 0 { break; }
        }
        return Some(result);
    };
}


/**
Receive data from the TcpStream, decode them and return them as String.
*/
pub fn receive_raw(raw_stream: &mut TcpStream) -> Option<String> {
    receive_from_stream!(raw_stream);
}


/**
Receive data from the TlsStream, decode them and return them as String.
 */
pub fn receive_tls(tls_stream: &mut TlsStream<TcpStream>) -> Option<String> {
    receive_from_stream!(tls_stream);
}
