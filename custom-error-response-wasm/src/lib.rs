use proxy_wasm::traits::*;
use proxy_wasm::types::*;

#[no_mangle]
pub fn _start() {
  proxy_wasm::set_http_context(|_, _| -> Box<dyn HttpContext> { Box::new(DemoPlugin) });
}

struct DemoPlugin;

impl HttpContext for DemoPlugin {
   fn on_http_response_headers(&mut self, _: usize, _: bool) -> Action {
      self.set_http_response_header("x-hello", Some("wasm"));
      Action::Continue
   }
}

impl Context for DemoPlugin {}