use proxy_wasm::traits::*;
use proxy_wasm::types::*;

#[no_mangle]
pub fn _start() {
  proxy_wasm::set_http_context(|_, _| -> Box<dyn HttpContext> { Box::new(DemoPlugin::new()) });
}

// struct DemoPlugin;
struct DemoPlugin {
   // Define request_id as a mutable field within the DemoPlugin struct
   request_id: String,
}

impl DemoPlugin {
   fn new() -> Self {
       DemoPlugin {
           request_id: String::new(),
       }
   }
}

impl HttpContext for DemoPlugin {

   fn on_http_request_headers(&mut self, _: usize, _: bool) -> Action {
      // log request headers
      /*for (name, value) in &self.get_http_request_headers() {
         //info!("#{} <- {}: {}", self.context_id, name, value);
         //info!("{}: {}", name, value);
         eprint!("{0}: {1}\n", name, value);
      }*/
      // get request_id header
      if let Some(_header_field) = self.get_http_request_header("x-request-id") {
         self.request_id = _header_field.to_string(); // Assign to the struct field
      }
      Action::Continue
   }

   fn on_http_response_headers(&mut self, _: usize, _: bool) -> Action {
      // add a header
      /*self.set_http_response_header("x-hello", Some("wasm"));
      self.set_http_response_header("x-test1", Some("hello world"));
      self.set_http_response_header("x-request-id", Some(&self.request_id)); */

      // capture HTTP status code from headers
      if let Some(status_code) = self.get_http_response_header(":status") {

         if status_code.to_string() == "404" {
            self.set_http_response_header("Content-Type", Some("text/html"));
            let headers = self.get_http_response_headers();

            // super hacky
            // Convert Vec<(String, String)> to Vec<(&str, &str)>
            let headers_converted: Vec<(&str, &str)> = headers
                                                .iter()
                                                .map(|(k, v)| (k.as_str(), v.as_str()))
                                                .collect();
            let new_body = format!("<h1>404 Not Found</h1>\n<p>Request ID: {}</p>", &self.request_id);
            self.send_http_response(404, headers_converted, Some(new_body.as_bytes()));
            return Action::Pause; // Stop further processing
         }
      }

      // log response headers
      /*
      for (name, value) in &self.get_http_response_headers() {
         //info!("#{} <- {}: {}", self.context_id, name, value);
         //info!("{}: {}", name, value);
         eprint!("{0}: {1}\n", name, value);
      }*/
      Action::Continue
   }
}

impl Context for DemoPlugin {}