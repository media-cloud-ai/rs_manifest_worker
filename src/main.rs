#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate yaserde_derive;

use std::env;
use std::process::exit;

use mcai_worker_sdk::{
  error, info, job::JobResult, start_worker, McaiChannel, MessageError, MessageEvent, Version,
};
use schemars::JsonSchema;

mod dash;
mod ism;
mod utils;

macro_rules! crate_version {
  () => {
    env!("CARGO_PKG_VERSION")
  };
}

#[derive(Debug, Default)]
struct DashManifestEvent {}

#[derive(Clone, Debug, Deserialize, JsonSchema)]
pub struct DashManifestParameters {
  /// Source Path of the Manifest
  source_path: String,
  /// Subtitle source path
  ttml_path: String,
  /// Subtitle language
  ttml_language: String,
  /// Subtitle role
  ttml_role: String,
  /// Replace the subtitle (default: false)
  replace: Option<bool>,
  destination_path: Option<String>,
  reference_url: Option<String>,
}

impl MessageEvent<DashManifestParameters> for DashManifestEvent {
  fn get_name(&self) -> String {
    "DASH Manifest worker".to_string()
  }

  fn get_short_description(&self) -> String {
    "Parse and get some information from DASH Manifest files".to_string()
  }

  fn get_description(&self) -> String {
    r#"Parse DASH manifest file and extract related files.
    "#
    .to_string()
  }

  fn get_version(&self) -> Version {
    Version::parse(crate_version!()).expect("unable to locate Package version")
  }

  fn process(
    &self,
    channel: Option<McaiChannel>,
    parameters: DashManifestParameters,
    job_result: JobResult,
  ) -> Result<JobResult, MessageError> {
    dash::message::process(channel, parameters, job_result)
  }
}

#[derive(Debug, Default)]
struct IsmManifestEvent {}

#[derive(Clone, Debug, Deserialize, JsonSchema)]
pub struct IsmManifestParameters {
  /// Source Path of the Manifest
  source_path: String,
}

impl MessageEvent<IsmManifestParameters> for IsmManifestEvent {
  fn get_name(&self) -> String {
    "ISM Manifest worker".to_string()
  }

  fn get_short_description(&self) -> String {
    "Parse and get some information from ISM Manifest files".to_string()
  }

  fn get_description(&self) -> String {
    r#"Parse ISM manifest file and extract related files.
    It can be possible to filter per content-type (Audio, Video, Subtitle)
    "#
    .to_string()
  }

  fn get_version(&self) -> Version {
    Version::parse(crate_version!()).expect("unable to locate Package version")
  }

  fn process(
    &self,
    channel: Option<McaiChannel>,
    parameters: IsmManifestParameters,
    job_result: JobResult,
  ) -> Result<JobResult, MessageError> {
    ism::message::process(channel, parameters, job_result)
  }
}

const ISM: &str = "ISM";
const DASH: &str = "DASH";

fn main() {
  match env::var("MANIFEST_MODE")
    .unwrap_or_else(|_| DASH.to_string())
    .as_str()
  {
    ISM => {
      info!("Start worker with ISM mode...");
      let message_event = IsmManifestEvent::default();
      start_worker(message_event)
    }
    DASH => {
      info!("Start worker with DASH mode...");
      let message_event = DashManifestEvent::default();
      start_worker(message_event)
    }
    value => {
      error!("Unsupported mode: {:?}", value);
      exit(1);
    }
  }
}
