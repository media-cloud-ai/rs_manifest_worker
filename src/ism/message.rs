use crate::ism::manifest::Smil;
use crate::IsmManifestParameters;
use mcai_worker_sdk::{
  job::{JobResult, JobStatus},
  McaiChannel, MessageError, Parameter, ParameterValue,
};
use std::fs;
use yaserde::de::from_str;

pub fn process(
  _channel: Option<McaiChannel>,
  parameters: IsmManifestParameters,
  job_result: JobResult,
) -> Result<JobResult, MessageError> {
  let mut sources = get_manifest_sources(job_result.clone(), &parameters.source_path)?;

  Ok(
    job_result
      .with_status(JobStatus::Completed)
      .with_parameters(&mut sources),
  )
}

fn get_manifest_sources(job_result: JobResult, path: &str) -> Result<Vec<Parameter>, MessageError> {
  let contents = fs::read_to_string(path).map_err(|e| {
    MessageError::ProcessingError(
      job_result
        .clone()
        .with_status(JobStatus::Error)
        .with_message(&e.to_string()),
    )
  })?;

  let manifest: Smil = from_str(&contents).map_err(|message| {
    MessageError::ProcessingError(
      job_result
        .with_status(JobStatus::Error)
        .with_message(&message),
    )
  })?;

  let mut sources = vec![];
  sources.push(Parameter {
    id: "audio".to_string(),
    kind: Vec::<String>::get_type_as_string(),
    store: None,
    default: None,
    value: serde_json::to_value(manifest.get_audio_stream_sources()).ok(),
  });

  sources.push(Parameter {
    id: "video".to_string(),
    kind: Vec::<String>::get_type_as_string(),
    store: None,
    default: None,
    value: serde_json::to_value(manifest.get_video_stream_sources()).ok(),
  });

  sources.push(Parameter {
    id: "subtitles".to_string(),
    kind: Vec::<String>::get_type_as_string(),
    store: None,
    default: None,
    value: serde_json::to_value(manifest.get_text_stream_sources()).ok(),
  });

  Ok(sources)
}

#[test]
fn get_manifest_sources_test() {
  use serde_json::Value;

  let job_result = JobResult::new(123);
  let result = get_manifest_sources(job_result, "tests/sample.ism");
  assert!(result.is_ok());
  let parameters = result.unwrap();
  let expected_kind = Vec::<String>::get_type_as_string();
  for param in parameters {
    assert_eq!(param.kind, expected_kind);

    match param.id.as_str() {
      "audio" => assert_eq!(
        param.value,
        Some(Value::Array(vec![Value::String(
          "test_file.isma".to_string()
        )]))
      ),
      "video" => assert_eq!(
        param.value,
        Some(Value::Array(vec![Value::String(
          "test_file.ismv".to_string()
        )]))
      ),
      "subtitles" => assert_eq!(
        param.value,
        Some(Value::Array(vec![Value::String(
          "test_file.ismt".to_string()
        )]))
      ),
      _ => assert!(false),
    }
  }
}

#[test]
fn ack_message_test() {
  use mcai_worker_sdk::{job::Job, parameter::container::ParametersContainer};

  let message = r#"{
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

  let job = Job::new(message).unwrap();
  let job_result = JobResult::new(job.job_id);
  let parameters: IsmManifestParameters = job.get_parameters().unwrap();
  let result = process(None, parameters, job_result);

  assert!(result.is_ok());
  let job_result = result.unwrap();
  assert_eq!(job_result.get_job_id(), 690);
  let audio_sources = job_result.get_parameter::<Vec<String>>("audio");
  assert_eq!(audio_sources, Ok(vec!["test_file.isma".to_string()]));
  let video_sources = job_result.get_parameter::<Vec<String>>("video");
  assert_eq!(video_sources, Ok(vec!["test_file.ismv".to_string()]));
  let subtitles_sources = job_result.get_parameter::<Vec<String>>("subtitles");
  assert_eq!(subtitles_sources, Ok(vec!["test_file.ismt".to_string()]));
}
