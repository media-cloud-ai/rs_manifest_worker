use crate::dash::manifest::{AdaptationSet, Manifest};
use amqp_worker::job::*;
use amqp_worker::*;
use std::fs;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use yaserde::de::from_str;
use yaserde::ser::to_string;

pub fn process(message: &str) -> Result<JobResult, MessageError> {
  println!("{:?}", message);
  let job = Job::new(message)?;

  warn!("{:?}", job);
  job.check_requirements()?;

  let manifest_path = job.get_string_parameter("source_path");
  let ttml_path = job.get_string_parameter("ttml_path");
  let ttml_language = job.get_string_parameter("ttml_language");
  let ttml_role = job.get_string_parameter("ttml_role");
  let replace = job.get_boolean_parameter("replace").unwrap_or(false);

  if manifest_path == None {
    return Err(MessageError::ProcessingError(
      JobResult::from(job).with_message("missing \"manifest_path\" parameter".to_string()),
    ));
  }
  if ttml_path == None {
    return Err(MessageError::ProcessingError(
      JobResult::from(job).with_message("missing \"ttml_path\" parameter".to_string()),
    ));
  }
  if ttml_language == None {
    return Err(MessageError::ProcessingError(
      JobResult::from(job).with_message("missing \"ttml_language\" parameter".to_string()),
    ));
  }
  if ttml_role == None {
    return Err(MessageError::ProcessingError(
      JobResult::from(job).with_message("missing \"ttml_role\" parameter".to_string()),
    ));
  }
  let manifest_path = manifest_path.unwrap();

  let destination_path = job
    .get_string_parameter("destination_path")
    .unwrap_or_else(|| manifest_path.clone());
  let reference_url = job.get_string_parameter("reference_url");

  add_ttml_subtitle(
    &job,
    &manifest_path,
    &destination_path,
    &ttml_path.unwrap(),
    &ttml_language.unwrap(),
    &ttml_role.unwrap(),
    &reference_url,
    replace,
  )?;

  Ok(JobResult::from(job).with_status(JobStatus::Completed))
}

fn add_ttml_subtitle(
  job: &Job,
  manifest_path: &str,
  destination_manifest_path: &str,
  ttml_path: &str,
  ttml_language: &str,
  ttml_role: &str,
  reference_url: &Option<String>,
  replace: bool,
) -> Result<(), MessageError> {
  let mp_folder = Path::new(manifest_path).parent();

  if mp_folder.is_none() {
    return Err(MessageError::ProcessingError(
      JobResult::from(job)
        .with_message("unable to found folder directory of the manifest".to_string()),
    ));
  }

  let reference_ttml_path = if let Ok(path) = Path::new(ttml_path).strip_prefix(mp_folder.unwrap())
  {
    path.to_str().unwrap()
  } else {
    ttml_path
  };

  let mut file = File::open(manifest_path)
    .map_err(|e| MessageError::ProcessingError(JobResult::from(job).with_message(e.to_string())))?;
  let mut contents = String::new();
  file
    .read_to_string(&mut contents)
    .map_err(|e| MessageError::ProcessingError(JobResult::from(job).with_message(e.to_string())))?;

  let mut manifest: Manifest = from_str(&contents)
    .map_err(|message| MessageError::ProcessingError(JobResult::from(job).with_message(message)))?;
  let ttml_file_size = if let Ok(metadata) = fs::metadata(&ttml_path) {
    metadata.len()
  } else {
    0
  };

  if let Some(url) = reference_url {
    manifest.prefix_urls(&url);
  }

  if replace {
    manifest.remove_adaptation_set(ttml_language, ttml_role);
  }
  let adaptation_set = AdaptationSet::new_ttml_subtitle(
    &reference_ttml_path,
    ttml_language,
    ttml_role,
    ttml_file_size,
  );
  manifest.add_adaptation_set(adaptation_set);

  let updated_manifest = to_string(&manifest)
    .map_err(|message| MessageError::ProcessingError(JobResult::from(job).with_message(message)))?;

  let mut output_file = File::create(destination_manifest_path)
    .map_err(|e| MessageError::ProcessingError(JobResult::from(job).with_message(e.to_string())))?;
  output_file
    .write_all(&updated_manifest.into_bytes())
    .map_err(|e| MessageError::ProcessingError(JobResult::from(job).with_message(e.to_string())))?;

  Ok(())
}

#[test]
fn add_subtitle_ttml_track() {
  let job = Job::new(r#"{"job_id": 666, "parameters": []}"#).unwrap();
  add_ttml_subtitle(
    &job,
    "tests/sample_1.mpd",
    "tests/sample_1_updated.mpd",
    "tests/sample_subtitle.ttml",
    "fra",
    "subtitle",
    &None,
    false,
  )
  .unwrap();

  let mut file_reference = File::open("tests/sample_1_for_validation.mpd").unwrap();
  let mut reference = String::new();
  file_reference.read_to_string(&mut reference).unwrap();

  let mut file = File::open("tests/sample_1_updated.mpd").unwrap();
  let mut content = String::new();
  file.read_to_string(&mut content).unwrap();

  println!("{}", content);
  assert!(content == reference);
  fs::remove_file("tests/sample_1_updated.mpd").unwrap();
}

#[test]
fn replace_subtitle_ttml_track_with_reference() {
  let job = Job::new(r#"{"job_id": 666, "parameters": []}"#).unwrap();
  add_ttml_subtitle(
    &job,
    "tests/sample_1.mpd",
    "tests/sample_1_replaced.mpd",
    "tests/sample_subtitle.ttml",
    "qaa",
    "subtitle",
    &Some("http://server.com/dash/manifest.mpd".to_string()),
    false,
  )
  .unwrap();

  let mut file_reference = File::open("tests/sample_1_for_replacement.mpd").unwrap();
  let mut reference = String::new();
  file_reference.read_to_string(&mut reference).unwrap();

  let mut file = File::open("tests/sample_1_replaced.mpd").unwrap();
  let mut content = String::new();
  file.read_to_string(&mut content).unwrap();

  println!("{}", content);
  assert!(content == reference);
  fs::remove_file("tests/sample_1_replaced.mpd").unwrap();
}

#[test]
fn add_http_subtitle_ttml_track() {
  let job = Job::new(r#"{"job_id": 666, "parameters": []}"#).unwrap();
  add_ttml_subtitle(
    &job,
    "tests/sample_1.mpd",
    "tests/sample_1_http_ttml.mpd",
    "http://server/static/sample_subtitle.ttml",
    "fra",
    "subtitle",
    &None,
    false,
  )
  .unwrap();

  let mut file_reference = File::open("tests/sample_1_for_http.mpd").unwrap();
  let mut reference = String::new();
  file_reference.read_to_string(&mut reference).unwrap();

  let mut file = File::open("tests/sample_1_http_ttml.mpd").unwrap();
  let mut content = String::new();
  file.read_to_string(&mut content).unwrap();

  println!("{}", content);
  assert!(content == reference);
  fs::remove_file("tests/sample_1_http_ttml.mpd").unwrap();
}
