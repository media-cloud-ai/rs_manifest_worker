use std::fs::File;
use std::io::Read;

use amqp_worker::*;
use yaserde::de::from_str;

use crate::ism::manifest::Smil;
use amqp_worker::job::Job;
use amqp_worker::job::JobResult;
use amqp_worker::job::JobStatus;
use amqp_worker::job::Parameter;
use amqp_worker::job::ParametersContainer;

/// Process the incoming message.
pub fn process(message: &str) -> Result<JobResult, MessageError> {
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
      JobResult::from(job).with_message("missing source path parameter".to_string()),
    ));
  }

  let sources = get_manifest_sources(&job, source_path.unwrap().as_str())?;

  Ok(JobResult::new(job.job_id, JobStatus::Completed, sources))
}

/// Extract the stream sources from the ISM manifest file
fn get_manifest_sources(job: &Job, path: &str) -> Result<Vec<Parameter>, MessageError> {
  let mut file = File::open(path)
    .map_err(|e| MessageError::ProcessingError(JobResult::from(job).with_message(e.to_string())))?;
  let mut contents = String::new();
  file
    .read_to_string(&mut contents)
    .map_err(|e| MessageError::ProcessingError(JobResult::from(job).with_message(e.to_string())))?;

  let manifest: Smil = from_str(&contents)
    .map_err(|message| MessageError::ProcessingError(JobResult::from(job).with_message(message)))?;

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
  let job = Job::new(r#"{"job_id": 123, "parameters": []}"#).unwrap();
  let result = get_manifest_sources(&job, "tests/sample.ism");
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
  let job_result = result.unwrap();
  assert_eq!(job_result.job_id, 690);
  let audio_sources = job_result.get_array_of_strings_parameter("audio");
  assert_eq!(audio_sources, Some(vec!["test_file.isma".to_string()]));
  let video_sources = job_result.get_array_of_strings_parameter("video");
  assert_eq!(video_sources, Some(vec!["test_file.ismv".to_string()]));
  let subtitles_sources = job_result.get_array_of_strings_parameter("subtitles");
  assert_eq!(subtitles_sources, Some(vec!["test_file.ismt".to_string()]));
}
