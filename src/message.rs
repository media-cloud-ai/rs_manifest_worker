
use amqp_worker::*;
use amqp_worker::job::*;
use crate::manifest::{AdaptationSet, Manifest};
use serde_json;
use std::fs;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use yaserde::de::from_str;
use yaserde::ser::to_string;

pub fn process(message: &str) -> Result<u64, MessageError> {
  println!("{:?}", message);
  let job = Job::new(message)?;

  warn!("{:?}", job);
  let _ = job.check_requirements()?;

  let manifest_path = job.get_string_parameter("manifest_path");
  let ttml_path = job.get_string_parameter("ttml_path");
  let ttml_language = job.get_string_parameter("ttml_language");
  let ttml_role = job.get_string_parameter("ttml_role");

  if manifest_path == None {
    return Err(MessageError::ProcessingError(job.job_id,
      "missing \"manifest_path\" parameter".to_string()
    ));
  }
  if ttml_path == None {
    return Err(MessageError::ProcessingError(job.job_id,
      "missing \"ttml_path\" parameter".to_string()
    ));
  }
  if ttml_language == None {
    return Err(MessageError::ProcessingError(job.job_id,
      "missing \"ttml_language\" parameter".to_string()
    ));
  }
  if ttml_role == None {
    return Err(MessageError::ProcessingError(job.job_id,
      "missing \"ttml_role\" parameter".to_string()
    ));
  }

  add_ttml_subtitle(
    job.job_id,
    &manifest_path.unwrap(),
    &ttml_path.unwrap(),
    &ttml_language.unwrap(),
    &ttml_role.unwrap()
    )?;

  Ok(job.job_id)
}

fn add_ttml_subtitle(job_id: u64, manifest_path: &str, ttml_path: &str, ttml_language: &str, ttml_role: &str) -> Result<(), MessageError> {
  
  let mp_folder = Path::new(manifest_path).parent();

  if mp_folder.is_none() {
    return Err(MessageError::ProcessingError(job_id, "unable to found folder directory of the manifest".to_string()));
  }

  let reference_ttml_path = 
    if let Ok(path) = Path::new(ttml_path).strip_prefix(mp_folder.unwrap()) {
      path.to_str().unwrap()
    } else {
      ttml_path
    };

  let mut file = File::open(manifest_path).map_err(|e| MessageError::ProcessingError(job_id, e.to_string()))?;
  let mut contents = String::new();
  file.read_to_string(&mut contents).map_err(|e| MessageError::ProcessingError(job_id, e.to_string()))?;

  let mut manifest: Manifest = from_str(&contents).map_err(|e| MessageError::ProcessingError(job_id, e))?;
  let ttml_file_size = 
    if let Ok(metadata) = fs::metadata(&ttml_path) {
      metadata.len()
    } else {
      0
    };

  let adaptation_set = AdaptationSet::new_ttml_subtitle(&reference_ttml_path, ttml_language, ttml_role, ttml_file_size);
  manifest.add_adaptation_set(adaptation_set);

  let updated_manifest = to_string(&manifest).map_err(|e| MessageError::ProcessingError(job_id, e))?;

  let mut output_file = File::create(manifest_path).map_err(|e| MessageError::ProcessingError(job_id, e.to_string()))?;
  output_file.write_all(&updated_manifest.into_bytes()).map_err(|e| MessageError::ProcessingError(job_id, e.to_string()))?;

  Ok(())
}

#[test]
fn add_subtitle_ttml_track() {
  use std::fs;
  fs::copy("tests/sample_1.mpd", "tests/sample_1_updated.mpd").unwrap();
  add_ttml_subtitle(666, "tests/sample_1_updated.mpd", "tests/sample_subtitle.ttml", "fra", "subtitle").unwrap();

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
fn add_http_subtitle_ttml_track() {
  use std::fs;
  fs::copy("tests/sample_1.mpd", "tests/sample_1_http_ttml.mpd").unwrap();

  add_ttml_subtitle(666, "tests/sample_1_http_ttml.mpd", "http://server/static/sample_subtitle.ttml", "fra", "subtitle").unwrap();

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
