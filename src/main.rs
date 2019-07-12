extern crate amqp_worker;
#[macro_use]
extern crate log;
extern crate serde_json;
extern crate simple_logger;
extern crate xml;
extern crate yaserde;
#[macro_use]
extern crate yaserde_derive;

use std::env;

use amqp_worker::*;
use log::Level;

mod dash;

#[derive(Debug)]
struct DashManifestEvent {}

impl MessageEvent for DashManifestEvent {
  fn process(&self, message: &str) -> Result<u64, MessageError> {
    dash::message::process(message)
  }
}

static DASH_MANIFEST_EVENT: DashManifestEvent = DashManifestEvent {};

fn main() {
  if let Ok(_) = env::var("VERBOSE") {
    simple_logger::init_with_level(Level::Debug).unwrap();
  } else {
    simple_logger::init_with_level(Level::Warn).unwrap();
  }

  start_worker(&DASH_MANIFEST_EVENT);
}
