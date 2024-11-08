//use log::info;
use proxy_wasm::traits::*;
use proxy_wasm::types::*;
//use bitvec::prelude::*;
//use std::any::type_name;

#[repr(u16)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CoreResponseFlag {
    NoError = 0,
    FailedLocalHealthCheck = 1,
    NoHealthyUpstream = 2,
    UpstreamRequestTimeout = 3,
    LocalReset = 4,
    UpstreamRemoteReset = 5,
    UpstreamConnectionFailure = 6,
    UpstreamConnectionTermination = 7,
    UpstreamOverflow = 8,
    NoRouteFound = 9,
    DelayInjected = 10,
    FaultInjected = 11,
    RateLimited = 12,
    UnauthorizedExternalService = 13,
    RateLimitServiceError = 14,
    DownstreamConnectionTermination = 15,
    UpstreamRetryLimitExceeded = 16,
    StreamIdleTimeout = 17,
    InvalidEnvoyRequestHeaders = 18,
    DownstreamProtocolError = 19,
    UpstreamMaxStreamDurationReached = 20,
    ResponseFromCacheFilter = 21,
    NoFilterConfigFound = 22,
    DurationTimeout = 23,
    UpstreamProtocolError = 24,
    NoClusterFound = 25,
    OverloadManager = 26,
    DnsResolutionFailed = 27,
    DropOverLoad = 28,
    DownstreamRemoteReset = 29,
    //LastFlag = DownstreamRemoteReset,
}

#[no_mangle]
pub fn _start() {
  proxy_wasm::set_http_context(|_, _| -> Box<dyn HttpContext> { Box::new(DemoPlugin::new()) });
}

// struct DemoPlugin;
struct DemoPlugin {
   // Define request_id as a mutable field within the DemoPlugin struct
   request_id: String,
   // Define vectors to store response flags for different status codes
   //response_flags_404: &'static [&'static str],
   //response_flags_4xx: &'static [&'static str],
   //response_flags_5xx: &'static [&'static str],
}

impl DemoPlugin {
   fn new() -> Self {
       DemoPlugin {
           request_id: String::new(),
           //response_flags_404: &["FI","NR","NC"],
           //response_flags_4xx: &["DC","DPE","DT","IH","RL","RLSE","UAEX"],
           //response_flags_5xx: &["LH","LR","OM","NFCF","SI","UC","UF","UH","UMSDR","UO","UPE","UR","URX","UT"],
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
      
      let flags_path = vec!["response", "flags"];
      let flags_result = self.get_property(flags_path).expect("REASON");

      // create a function to convert the vector to a byte 
      fn vec_to_byte_reversed(bits: Vec<u8>) -> Option<u8> {
         if bits.len() != 8 {
             return None; // We need exactly 8 bits
         }
     
         let mut byte = 0u8;
         for (i, bit) in bits.iter().enumerate() {
             if *bit > 1 {
                 return None; // Invalid bit value
             }
             byte |= (*bit as u8) << i;
         }
     
         Some(byte)
      }

      let flag_byte = vec_to_byte_reversed(flags_result.clone()).unwrap_or_default();
      let flag_byte_as_u16 = flag_byte as u16;
      eprintln!("flags_result: {:?}", flags_result);
      eprintln!("flag_byte: {:b}", flag_byte);
      eprintln!("as integer: {}", flag_byte_as_u16);

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