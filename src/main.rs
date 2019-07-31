extern crate amqp_worker;
#[macro_use]
extern crate log;
extern crate serde_json;
extern crate xml;
extern crate yaserde;
#[macro_use]
extern crate yaserde_derive;

use std::env;

use amqp_worker::job::*;
use amqp_worker::*;
use std::process::exit;

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
  match env::var("MANIFEST_MODE")
    .unwrap_or(DASH.to_string())
    .as_str()
  {
    ISM => {
      info!("Start worker with ISM mode...");
      start_worker(&ISM_MANIFEST_EVENT)
    }
    DASH => {
      info!("Start worker with DASH mode...");
      start_worker(&DASH_MANIFEST_EVENT)
    }
    value => {
      error!("Unsupported mode: {:?}", value);
      exit(1);
    }
  }
}
