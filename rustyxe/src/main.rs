mod config;
mod xml;

use log::{debug, error};
use openssl::ssl::{SslConnector, SslMethod};
use std::io::{Read, Write};
use std::net::TcpStream;

#[macro_use]
extern crate log;

fn main() -> std::io::Result<()> {
    env_logger::init();

    // Setup the SSL connector
    let mut builder = SslConnector::builder(SslMethod::tls()).unwrap();
    builder.set_verify(openssl::ssl::SslVerifyMode::NONE);
    let connector = builder.build();

    let stream = TcpStream::connect(format!("{}:443", config::HOSTIP))?;

    // Upgrade the stream with SSL connector
    let mut stream = connector.connect(config::HOSTIP, stream).unwrap();

    // Prepare the XML-RPC request
    let xmlrpc_request = xml::create_xmlrpc_request(
        "session.login_with_password",
        vec![config::USERNAME, config::PASSWORD, "1.0"],
    );

    debug!("Request sent");
    debug!("{}", xmlrpc_request);
    stream.write_all(xmlrpc_request.as_bytes())?;

    let mut recv_buf = [0; 1024];
    let recv_bytes = match stream.read(&mut recv_buf) {
        Ok(v) => v,
        Err(_) => {
            error!("Failed to read stream");
            std::process::exit(1);
        }
    };

    debug!("Response received");
    if let Ok(recv_str) = std::str::from_utf8(&recv_buf[..recv_bytes]) {
        debug!("{}", recv_str)
    } else {
        error!("ERROR: invalid utf8")
    }

    Ok(())
}
