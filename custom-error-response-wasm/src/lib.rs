//use log::info;
use proxy_wasm::traits::*;
use proxy_wasm::types::*;
use bitvec::prelude::*;

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CoreResponseFlag {
    FailedLocalHealthCheck = 0,
    NoHealthyUpstream = 1,
    UpstreamRequestTimeout = 2,
    LocalReset = 3,
    UpstreamRemoteReset = 4,
    UpstreamConnectionFailure = 5,
    UpstreamConnectionTermination = 6,
    UpstreamOverflow = 7,
    NoRouteFound = 8,
    DelayInjected = 9,
    FaultInjected = 10,
    RateLimited = 11,
    UnauthorizedExternalService = 12,
    RateLimitServiceError = 13,
    DownstreamConnectionTermination = 14,
    UpstreamRetryLimitExceeded = 15,
    StreamIdleTimeout = 16,
    InvalidEnvoyRequestHeaders = 17,
    DownstreamProtocolError = 18,
    UpstreamMaxStreamDurationReached = 19,
    ResponseFromCacheFilter = 20,
    NoFilterConfigFound = 21,
    DurationTimeout = 22,
    UpstreamProtocolError = 23,
    NoClusterFound = 24,
    OverloadManager = 25,
    DnsResolutionFailed = 26,
    DropOverLoad = 27,
    DownstreamRemoteReset = 28,
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
      /*if let Some(response_flags) = self.get_property(vec!["response", "flags"]) {
         //let flag_str = String::from_utf8_lossy(&response_flags);
         eprintln!("response_flags: {:?}", response_flags);
      } else {
         eprintln!("Failed to get response flags");
      }*/
      if let Some(response_flags_bool) = self.get_property(vec!["response", "flags"]) {
         eprintln!("{}", format!("response_flags_bool: {:?}", response_flags_bool));

         let response_flags: u32 = if let Ok(bitvector) = response_flags_bool.try_into() {
            let mut bv = BitVec::<u32, Msb0>::from_vec(bitvector);
            bv.load_be()
         } else {
            eprintln!("Failed to convert response flags to bytes");
            return Action::Continue;
         };
      } else {
         eprintln!("Failed to get response flags");
      }
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