mod config;
mod xml;

use openssl::ssl::{SslConnector, SslMethod};
use std::io::{Read, Write};
use std::net::TcpStream;

fn main() -> std::io::Result<()> {
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

    println!("== Request sent ==\n{}", xmlrpc_request);
    stream.write_all(xmlrpc_request.as_bytes())?;

    let mut recv_buf = [0; 1024];
    let recv_bytes = match stream.read(&mut recv_buf) {
        Ok(v) => v,
        Err(_) => panic!("ERROR: Failed to read stream"),
    };

    println!("\n== Response received==");
    if let Ok(recv_str) = std::str::from_utf8(&recv_buf[..recv_bytes]) {
        println!("{}", recv_str)
    } else {
        println!("ERROR: invalid utf8")
    }

    Ok(())
}
