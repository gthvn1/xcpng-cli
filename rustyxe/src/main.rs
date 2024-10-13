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

    let mut recv_buf = Vec::new();
    let mut recv_tmp = [0; 1024];
    loop {
        match stream.read(&mut recv_tmp) {
            Ok(0) => break, // connection is closed and all data are read
            Ok(n) => {
                recv_buf.extend_from_slice(&recv_tmp[..n]);
                // It looks like the server doesn't close the connection so
                // we check if we received the </methodResponse> the indicates
                // that we have all data.
                let recv_str = String::from_utf8_lossy(&recv_buf);
                if recv_str.find("</methodResponse>").is_some() {
                    break;
                }
            }
            Err(_) => {
                error!("Failed to read stream");
                std::process::exit(1);
            }
        };
    }

    debug!("Response received");
    let mut session_id: Option<String> = None;
    // Check that we only have valid utf8
    if let Ok(recv_str) = std::str::from_utf8(&recv_buf) {
        match xml::extract_result(recv_str) {
            (None, _) => error!("Failed to get status\nresponse: {}", recv_str),
            (Some(s), None) => error!("status is {} but value is None.\nresponse: {}", s, recv_str),
            (Some(s), Some(v)) => {
                if s == *"Success" {
                    info!("{}: value is {}", s, v);
                    session_id = Some(v);
                } else {
                    error!(
                        "Satus {}, Success was expected. Found value {}\nresponse: {}",
                        s, v, recv_str
                    );
                }
            }
        }
    } else {
        error!("invalid utf8: {:?}", &recv_buf);
    }

    // !!! POC !!! WIP !!! REFACTOR !!!
    // From here we just want to check that we can use the session_id
    // !!! POC !!! WIP !!! REFACTOR !!!
    let session_id = match session_id {
        None => {
            error!("Failed to get a session id");
            std::process::exit(1);
        }
        Some(v) => v,
    };

    // Prepare the XML-RPC request
    let xmlrpc_request = xml::create_xmlrpc_request("host.get_all", vec![session_id.as_str()]);

    debug!("Request sent");
    debug!("{}", xmlrpc_request);
    stream.write_all(xmlrpc_request.as_bytes())?;

    let mut recv_buf = Vec::new();
    let mut recv_tmp = [0; 1024];
    loop {
        match stream.read(&mut recv_tmp) {
            Ok(0) => break, // connection is closed and all data are read
            Ok(n) => {
                recv_buf.extend_from_slice(&recv_tmp[..n]);
                // It looks like the server doesn't close the connection so
                // we check if we received the </methodResponse> the indicates
                // that we have all data.
                let recv_str = String::from_utf8_lossy(&recv_buf);
                if recv_str.find("</methodResponse>").is_some() {
                    break;
                }
            }
            Err(_) => {
                error!("Failed to read stream");
                std::process::exit(1);
            }
        };
    }

    debug!("Response received");
    debug!("{}", String::from_utf8_lossy(&recv_buf));

    Ok(())
}
