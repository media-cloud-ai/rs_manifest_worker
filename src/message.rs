
use amqp_worker::*;
use crate::manifest::{AdaptationSet, Manifest};
use serde_json;
use std::fs;
use std::fs::File;
use std::io::{Read, Write};
use yaserde::de::from_str;
use yaserde::ser::to_string;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Parameter {
  #[serde(rename="type")]
  kind: String,
  id: String,
  default: Option<String>,
  value: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Job {
  job_id: u64,
  parameters: Vec<Parameter>
}

fn get_parameter(params: &Vec<Parameter>, key: &str) -> Option<String> {
  for param in params.iter() {
    if param.id == key {
      if let Some(ref value) = param.value {
        return Some(value.clone())
      } else {
        return param.default.clone()
      }
    }
  }
  None
}

pub fn process(message: &str) -> Result<u64, MessageError> {
  let parsed: Result<Job, _> = serde_json::from_str(message);

  match parsed {
    Ok(content) => {
      warn!("{:?}", content);
      let manifest_path = get_parameter(&content.parameters, "manifest_path");
      let ttml_path = get_parameter(&content.parameters, "ttml_path");
      let ttml_language = get_parameter(&content.parameters, "ttml_language");
      let ttml_role = get_parameter(&content.parameters, "ttml_role");

      if manifest_path == None {
        return Err(MessageError::ProcessingError(content.job_id,
          "missing \"manifest_path\" parameter".to_string()
        ));
      }
      if ttml_path == None {
        return Err(MessageError::ProcessingError(content.job_id,
          "missing \"ttml_path\" parameter".to_string()
        ));
      }
      if ttml_language == None {
        return Err(MessageError::ProcessingError(content.job_id,
          "missing \"ttml_language\" parameter".to_string()
        ));
      }
      if ttml_role == None {
        return Err(MessageError::ProcessingError(content.job_id,
          "missing \"ttml_role\" parameter".to_string()
        ));
      }

      add_ttml_subtitle(
        content.job_id,
        &manifest_path.unwrap(),
        &ttml_path.unwrap(),
        &ttml_language.unwrap(),
        &ttml_role.unwrap()
        )?;

      Ok(content.job_id)
    },
    Err(msg) => {
      error!("{:?}", msg);
      return Err(MessageError::RuntimeError("bad input message".to_string()));
    }
  }
}

fn add_ttml_subtitle(job_id: u64, manifest_path: &str, ttml_path: &str, ttml_language: &str, ttml_role: &str) -> Result<(), MessageError> {
  let mut file = File::open(manifest_path).map_err(|e| MessageError::ProcessingError(job_id, e.to_string()))?;
  let mut contents = String::new();
  file.read_to_string(&mut contents).map_err(|e| MessageError::ProcessingError(job_id, e.to_string()))?;

  let mut manifest: Manifest = from_str(&contents).map_err(|e| MessageError::ProcessingError(job_id, e))?;
  let metadata = fs::metadata(ttml_path).map_err(|e| MessageError::ProcessingError(job_id, e.to_string()))?;

  let adaptation_set = AdaptationSet::new_ttml_subtitle(ttml_path, ttml_language, ttml_role, metadata.len());
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

  let mut file = File::open("tests/sample_1_for_validation.mpd").unwrap();
  let mut content = String::new();
  file.read_to_string(&mut content).unwrap();

  assert!(content == reference);
  fs::remove_file("tests/sample_1_updated.mpd").unwrap();
}
