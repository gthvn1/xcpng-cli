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

//
// Extract the value from the XML-RPC response
// Here is an example of the XML-RPC response:
// <?xml version="1.0"?>
// <methodResponse>
//    <params>
//      <param>
//        <value>
//          <struct>
//            <member>
//              <name>Status</name>
//              <value>Success</value>
//            </member>
//            <member>
//              <name>Value</name>
//              <value>OpaqueRef:123456789</value>
//            </member>
//          </struct>
//        </value>
//      </param>
//    </params>
//  </methodResponse>
// Return the Status and the Value
fn extract_value_by_name(xml_response: &str, tag: &str) -> Option<String> {
    let tag_str = format!("<name>{}</name>", tag);

    if let Some(tag_start) = xml_response.find(&tag_str) {
        let value_start_tag = "<value>";
        let value_end_tag = "</value>";

        if let Some(value_start) = xml_response[tag_start..].find(value_start_tag) {
            let value_start_pos = tag_start + value_start + value_start_tag.len();
            if let Some(value_end) = xml_response[value_start_pos..].find(value_end_tag) {
                let value_end_pos = value_start_pos + value_end;
                return Some(xml_response[value_start_pos..value_end_pos].to_string());
            }
        }
    }

    None
}

pub fn extract_result(xml_response: &str) -> (Option<String>, Option<String>) {
    (
        extract_value_by_name(xml_response, "Status"),
        extract_value_by_name(xml_response, "Value"),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    const XML_RESPONSE: &str = r#"
        <?xml version="1.0"?>
        <methodResponse>
           <params>
             <param>
               <value>
                 <struct>
                   <member>
                     <name>Status</name>
                     <value>Success</value>
                   </member>
                   <member>
                     <name>Value</name>
                     <value>OpaqueRef:123456789</value>
                   </member>
                 </struct>
               </value>
             </param>
           </params>
         </methodResponse>
    "#;

    #[test]
    fn extract_status() {
        let v = match extract_value_by_name(XML_RESPONSE, "Status") {
            None => String::new(),
            Some(val) => val,
        };

        assert_eq!(v, "Success");
    }

    #[test]
    fn extract_value() {
        let v = match extract_value_by_name(XML_RESPONSE, "Value") {
            None => String::new(),
            Some(val) => val,
        };

        assert_eq!(v, "OpaqueRef:123456789");
    }
}
