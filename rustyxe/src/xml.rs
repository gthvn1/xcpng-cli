use super::config;
use base64::{engine::general_purpose, Engine};

pub fn create_xmlrpc_request(method: &str, params: Vec<&str>) -> String {
    // Encode the username and password in Base64 for the Authorization header
    let credentials = format!("{}:{}", config::USERNAME, config::PASSWORD);
    let encoded_credentials = general_purpose::STANDARD.encode(credentials);

    // Prepare the XML-RPC request payload
    let mut xml_body = "<?xml version='1.0'?>".to_owned();
    xml_body.push_str("<methodCall>");
    xml_body.push_str(format!("<methodName>{}</methodName>", method).as_str());
    xml_body.push_str("<params>");
    for param in params {
        xml_body.push_str(format!("<param><value>{}</value></param>", param).as_str());
    }
    xml_body.push_str("<param><value>1.0</value></param>");
    xml_body.push_str("</params>");
    xml_body.push_str("</methodCall>");

    let content_length = xml_body.len();

    // Prepare a simple XML-RPC request
    let mut xml_req = "POST /RPC2 HTTP/1.1\r\n".to_owned();
    xml_req.push_str(format!("Host: {}\r\n", config::HOSTIP).as_str());
    xml_req.push_str(format!("Authorization: Basic {}\r\n", encoded_credentials).as_str());
    // xml_rpc_request.push_str("Accept: */*\r\n");
    xml_req.push_str("Content-Type: text/xml\r\n");
    xml_req.push_str(format!("Content-Length: {}\r\n", content_length).as_str());
    xml_req.push_str("\r\n"); // Empty line before body
    xml_req.push_str(xml_body.as_str());

    debug!("XML-RPC request created");
    xml_req
}
