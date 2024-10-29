//use log::info;
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
   // Define vectors to store response flags for different status codes
   response_flags_404: &'static [&'static str],
   response_flags_4xx: &'static [&'static str],
   response_flags_5xx: &'static [&'static str],
}

impl DemoPlugin {
   fn new() -> Self {
       DemoPlugin {
           request_id: String::new(),
           response_flags_404: &["FI","NR","NC"],
           response_flags_4xx: &["DC","DPE","DT","IH","RL","RLSE","UAEX"],
           response_flags_5xx: &["LH","LR","OM","NFCF","SI","UC","UF","UH","UMSDR","UO","UPE","UR","URX","UT"],
       }
   }
}

impl HttpContext for DemoPlugin {

   fn on_http_request_headers(&mut self, _: usize, _: bool) -> Action {
      // log request headers
      for (name, value) in &self.get_http_request_headers() {
         //info!("#{} <- {}: {}", self.context_id, name, value);
         //info!("{}: {}", name, value);
         eprint!("{0}: {1}\n", name, value);
      }
      // get request_id header
      if let Some(_header_field) = self.get_http_request_header("x-request-id") {
         self.request_id = _header_field.to_string(); // Assign to the struct field
      }
      Action::Continue
   }
   fn on_http_response_headers(&mut self, _: usize, _: bool) -> Action {
      // add a header
      self.set_http_response_header("x-hello", Some("wasm"));
      self.set_http_response_header("x-test1", Some("hello world"));
      self.set_http_response_header("x-request-id", Some(&self.request_id)); 
      // test message
      eprintln!("hello error from wasm");
      // log response headers
      for (name, value) in &self.get_http_response_headers() {
         //info!("#{} <- {}: {}", self.context_id, name, value);
         //info!("{}: {}", name, value);
         eprint!("{0}: {1}\n", name, value);
      }
      Action::Continue
   }
}

impl Context for DemoPlugin {}