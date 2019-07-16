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

use amqp_worker::job::*;
use amqp_worker::*;
use log::Level;

mod dash;
mod ism;
mod utils;

#[derive(Debug)]
struct DashManifestEvent {}

impl MessageEvent for DashManifestEvent {
  fn process(&self, message: &str) -> Result<JobResult, MessageError> {
    dash::message::process(message)
  }
}

#[derive(Debug)]
struct IsmManifestEvent {}

impl MessageEvent for IsmManifestEvent {
  fn process(&self, message: &str) -> Result<JobResult, MessageError> {
    ism::message::process(message)
  }
}

static DASH_MANIFEST_EVENT: DashManifestEvent = DashManifestEvent {};
static ISM_MANIFEST_EVENT: IsmManifestEvent = IsmManifestEvent {};

const ISM: &str = "ISM";
const DASH: &str = "DASH";

fn main() {
  if let Ok(_) = env::var("VERBOSE") {
    simple_logger::init_with_level(Level::Debug).unwrap();
  } else {
    simple_logger::init_with_level(Level::Warn).unwrap();
  }

  match env::var("MANIFEST_MODE")
    .unwrap_or(DASH.to_string())
    .as_str()
  {
    ISM => {
      info!("Start worker with ISM mode...");
      start_worker(&ISM_MANIFEST_EVENT)
    }
    _ => {
      info!("Start worker with DASH mode...");
      start_worker(&DASH_MANIFEST_EVENT)
    }
  }
}
