extern crate amqp_worker;
#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate simple_logger;
#[macro_use]
extern crate yaserde_derive;
extern crate xml;

use amqp_worker::*;
use std::env;
use log::Level;

mod manifest;
mod message;

#[derive(Debug)]
struct HttpEvent {
}

impl MessageEvent for HttpEvent {
  fn process(&self, message: &str) -> Result<u64, MessageError> {
    message::process(message)
  }
}

fn main() {
  if let Ok(_)= env::var("VERBOSE") {
    simple_logger::init_with_level(Level::Debug).unwrap();
  } else {
    simple_logger::init_with_level(Level::Warn).unwrap();
  }

  let http_event = HttpEvent{};
  start_worker(&http_event);
}