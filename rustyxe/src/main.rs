mod config;

use base64::{engine::general_purpose, Engine};
use openssl::ssl::{SslConnector, SslMethod};
use std::io::{Read, Write};
use std::net::TcpStream;

fn main() -> std::io::Result<()> {
    // Setup the SSL connector
    let mut builder = SslConnector::builder(SslMethod::tls()).unwrap();
    builder.set_verify(openssl::ssl::SslVerifyMode::NONE);
    let connector = builder.build();

    // Connect to the server (replace with the server's IP and HTTP port for non-TLS)
    let stream = TcpStream::connect(format!("{}:443", config::HOSTIP))?; // format!("172.16.211.36:443")?;
    // Upgrade the stream with SSL connector
    let mut stream = connector.connect(config::HOSTIP, stream).unwrap();

    // Encode the username and password in Base64 for the Authorization header
    let credentials = format!("{}:{}", config::USERNAME, config::PASSWORD);
    let encoded_credentials = general_purpose::STANDARD.encode(credentials);

    // Prepare the XML-RPC request payload
    let mut xml_body = "<?xml version='1.0'?>".to_owned();
    xml_body.push_str("<methodCall>");
    xml_body.push_str("<methodName>session.login_with_password</methodName>");
    xml_body.push_str("<params>");
    xml_body.push_str(format!("<param><value>{}</value></param>", config::USERNAME).as_str());
    xml_body.push_str(format!("<param><value>{}</value></param>", config::PASSWORD).as_str());
    xml_body.push_str("<param><value>1.0</value></param>");
    xml_body.push_str("</params>");
    xml_body.push_str("</methodCall>");

    let content_length = xml_body.len();

    // Prepare a simple XML-RPC request
    let mut xml_rpc_request = "POST /RPC2 HTTP/1.1\r\n".to_owned();
    xml_rpc_request.push_str(format!("Host: {}\r\n", config::HOSTIP).as_str());
    xml_rpc_request.push_str(format!("Authorization: Basic {}\r\n", encoded_credentials).as_str());
    // xml_rpc_request.push_str("Accept: */*\r\n");
    xml_rpc_request.push_str("Content-Type: text/xml\r\n");
    xml_rpc_request.push_str(format!("Content-Length: {}\r\n", content_length).as_str());
    xml_rpc_request.push_str("\r\n"); // Empty line before body
    xml_rpc_request.push_str(xml_body.as_str());

    println!("Send the request\n{}", xml_rpc_request);
    stream.write_all(xml_rpc_request.as_bytes())?;

    let mut recv_buf = [0; 1024];
    let recv_bytes = match stream.read(&mut recv_buf) {
        Ok(v) => v,
        Err(_) => panic!("ERROR: Failed to read stream"),
    };

    println!("Print the response");
    if let Ok(recv_str) = std::str::from_utf8(&recv_buf[..recv_bytes]) {
        println!("{}", recv_str)
    } else {
        println!("ERROR: invalid utf8")
    }

    Ok(())
}
