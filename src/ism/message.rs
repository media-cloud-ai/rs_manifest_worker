
use crate::ism::manifest::Smil;
use mcai_worker_sdk::{
  job::{Job, JobResult, JobStatus},
  parameter::container::ParametersContainer,
  McaiChannel,
  MessageError,
  Parameter,
};
use std::fs;
use yaserde::de::from_str;

pub fn process(
  _channel: Option<McaiChannel>,
  job: &Job,
  job_result: JobResult,
) -> Result<JobResult, MessageError> {
  let source_path = job.get_string_parameter("source_path");

  if source_path.is_none() {
    return Err(MessageError::ProcessingError(
      job_result
        .with_status(JobStatus::Error)
        .with_message("missing source path parameter"),
    ));
  }

  let mut sources = get_manifest_sources(job_result.clone(), source_path.unwrap().as_str())?;

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
        .with_message(&message.to_string()),
    )
  })?;

  let mut sources = vec![];
  sources.push(Parameter::ArrayOfStringsParam {
    id: "audio".to_string(),
    default: None,
    value: Some(manifest.get_audio_stream_sources()),
  });

  sources.push(Parameter::ArrayOfStringsParam {
    id: "video".to_string(),
    default: None,
    value: Some(manifest.get_video_stream_sources()),
  });

  sources.push(Parameter::ArrayOfStringsParam {
    id: "subtitles".to_string(),
    default: None,
    value: Some(manifest.get_text_stream_sources()),
  });

  Ok(sources)
}

#[test]
fn get_manifest_sources_test() {
  let job_result = JobResult::new(123);
  let result = get_manifest_sources(job_result, "tests/sample.ism");
  assert!(result.is_ok());
  let parameters = result.unwrap();
  for param in parameters {
    if let Parameter::ArrayOfStringsParam {
      id,
      default: _,
      value,
    } = param
    {
      match id.as_str() {
        "audio" => assert_eq!(value, Some(vec!["test_file.isma".to_string()])),
        "video" => assert_eq!(value, Some(vec!["test_file.ismv".to_string()])),
        "subtitles" => assert_eq!(value, Some(vec!["test_file.ismt".to_string()])),
        _ => assert!(false),
      }
    } else {
      assert!(false);
    }
  }
}

#[test]
fn ack_message_test() {
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
  let result = process(None, &job, job_result);

  assert!(result.is_ok());
  let job_result = result.unwrap();
  assert_eq!(job_result.get_job_id(), 690);
  let audio_sources = job_result.get_array_of_strings_parameter("audio");
  assert_eq!(audio_sources, Some(vec!["test_file.isma".to_string()]));
  let video_sources = job_result.get_array_of_strings_parameter("video");
  assert_eq!(video_sources, Some(vec!["test_file.ismv".to_string()]));
  let subtitles_sources = job_result.get_array_of_strings_parameter("subtitles");
  assert_eq!(subtitles_sources, Some(vec!["test_file.ismt".to_string()]));
}
