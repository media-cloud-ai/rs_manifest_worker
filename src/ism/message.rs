use std::fs::File;
use std::io::Read;
use std::io::Write;

use amqp_worker::*;
use yaserde::de::from_str;
use yaserde::{YaDeserialize, YaSerialize};

use crate::ism::manifest::Smil;

/// Process the incoming message.
pub fn process(message: &str) -> Result<ManifestSources, MessageError> {
  let job = job::Job::new(message)?;
  debug!("reveived message: {:?}", job);

  match job.check_requirements() {
    Ok(_) => {}
    Err(message) => {
      return Err(message);
    }
  }

  let source_path = job.get_string_parameter("source_path");

  if source_path.is_none() {
    return Err(MessageError::ProcessingError(
      job.job_id,
      "missing source path parameter".to_string(),
    ));
  }

  let sources = get_manifest_sources(job.job_id, source_path.unwrap().as_str())?;

  Ok(sources)
}

#[derive(Debug, YaDeserialize, YaSerialize)]
pub struct ManifestSources {
  job_id: u64,
  audio: Vec<String>,
  video: Vec<String>,
  subtitles: Vec<String>,
}

/// Extract the stream sources from the ISM manifest file
fn get_manifest_sources(job_id: u64, path: &str) -> Result<ManifestSources, MessageError> {
  let mut file =
    File::open(path).map_err(|e| MessageError::ProcessingError(job_id, e.to_string()))?;
  let mut contents = String::new();
  file
    .read_to_string(&mut contents)
    .map_err(|e| MessageError::ProcessingError(job_id, e.to_string()))?;

  let manifest: Smil = from_str(&contents).map_err(|e| MessageError::ProcessingError(job_id, e))?;

  Ok(ManifestSources {
    job_id,
    audio: manifest.get_audio_stream_sources(),
    video: manifest.get_video_stream_sources(),
    subtitles: manifest.get_text_stream_sources(),
  })
}

#[test]
fn get_manifest_sources_test() {
  let job_id = 123;
  let result = get_manifest_sources(123, "tests/sample.ism");
  assert!(result.is_ok());
  let manifest_sources = result.unwrap();
  assert_eq!(manifest_sources.job_id, job_id);
  assert_eq!(manifest_sources.audio, ["test_file.isma"]);
  assert_eq!(manifest_sources.video, ["test_file.ismv"]);
  assert_eq!(manifest_sources.subtitles, ["test_file.ismt"]);
}

#[test]
fn ack_message_test() {
  let msg = r#"{
    "parameters": [
      {
        "id": "requirements",
        "type": "requirements",
        "value": {"paths": [
          "tests/sample.ism"
        ]}
      },
      {
        "id": "source_path",
        "type": "string",
        "value": "tests/sample.ism"
      }
    ],
    "job_id":690
  }"#;

  let result = process(msg);
  assert!(result.is_ok());
  let manifest_sources = result.unwrap();
  assert_eq!(manifest_sources.job_id, 690);
  assert_eq!(manifest_sources.audio, ["test_file.isma"]);
  assert_eq!(manifest_sources.video, ["test_file.ismv"]);
  assert_eq!(manifest_sources.subtitles, ["test_file.ismt"]);
}
