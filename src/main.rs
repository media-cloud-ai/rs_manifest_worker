#[macro_use]
extern crate yaserde_derive;

use mcai_worker_sdk::{
  error,
  info,
  job::{Job, JobResult},
  start_worker,
  worker::{Parameter, ParameterType},
  McaiChannel,
  MessageError,
  MessageEvent,
  Version,
};
use std::env;
use std::process::exit;

mod dash;
mod ism;
mod utils;

macro_rules! crate_version {
  () => {
    env!("CARGO_PKG_VERSION")
  };
}

#[derive(Debug)]
struct DashManifestEvent {}

impl MessageEvent for DashManifestEvent {
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

  fn get_parameters(&self) -> Vec<Parameter> {
    vec![
      Parameter {
        identifier: "source_path".to_string(),
        label: "Source Path of the Manifest".to_string(),
        kind: vec![ParameterType::String],
        required: true,
      },
      Parameter {
        identifier: "ttml_path".to_string(),
        label: "Subtitle source path".to_string(),
        kind: vec![ParameterType::String],
        required: true,
      },
      Parameter {
        identifier: "ttml_language".to_string(),
        label: "Subtitle language".to_string(),
        kind: vec![ParameterType::String],
        required: true,
      },
      Parameter {
        identifier: "ttml_role".to_string(),
        label: "Subtitle role".to_string(),
        kind: vec![ParameterType::String],
        required: true,
      },
      Parameter {
        identifier: "replace".to_string(),
        label: "Replace the subtitle (default: false)".to_string(),
        kind: vec![ParameterType::Boolean],
        required: false,
      },
    ]
  }

  fn process(
    &self,
    channel: Option<McaiChannel>,
    job: &Job,
    job_result: JobResult,
  ) -> Result<JobResult, MessageError> {
    dash::message::process(channel, job, job_result)
  }
}

#[derive(Debug)]
struct IsmManifestEvent {}

impl MessageEvent for IsmManifestEvent {
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

  fn get_parameters(&self) -> Vec<Parameter> {
    vec![Parameter {
      identifier: "source_path".to_string(),
      label: "Source Path".to_string(),
      kind: vec![ParameterType::String],
      required: true,
    }]
  }

  fn process(
    &self,
    channel: Option<McaiChannel>,
    job: &Job,
    job_result: JobResult,
  ) -> Result<JobResult, MessageError> {
    ism::message::process(channel, job, job_result)
  }
}

static DASH_MANIFEST_EVENT: DashManifestEvent = DashManifestEvent {};
static ISM_MANIFEST_EVENT: IsmManifestEvent = IsmManifestEvent {};

const ISM: &str = "ISM";
const DASH: &str = "DASH";

fn main() {
  match env::var("MANIFEST_MODE")
    .unwrap_or_else(|_| DASH.to_string())
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
