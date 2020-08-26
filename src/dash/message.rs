use crate::dash::manifest::{AdaptationSet, Manifest};
use crate::DashManifestParameters;
use mcai_worker_sdk::{
  job::{JobResult, JobStatus},
  McaiChannel, MessageError,
};
use std::fs;
use std::path::Path;
use yaserde::{de::from_str, ser::to_string};

pub fn process(
  _channel: Option<McaiChannel>,
  parameters: DashManifestParameters,
  job_result: JobResult,
) -> Result<JobResult, MessageError> {
  add_ttml_subtitle(job_result.clone(), parameters)?;

  Ok(job_result.with_status(JobStatus::Completed))
}

fn add_ttml_subtitle(
  job_result: JobResult,
  parameters: DashManifestParameters,
) -> Result<(), MessageError> {
  let mp_folder = Path::new(&parameters.source_path).parent();

  if mp_folder.is_none() {
    return Err(MessageError::ProcessingError(
      job_result
        .with_status(JobStatus::Error)
        .with_message("unable to found folder directory of the manifest"),
    ));
  }

  let reference_ttml_path =
    if let Ok(path) = Path::new(&parameters.ttml_path).strip_prefix(mp_folder.unwrap()) {
      path.to_str().unwrap()
    } else {
      &parameters.ttml_path
    };

  let contents = fs::read_to_string(&parameters.source_path).map_err(|e| {
    MessageError::ProcessingError(
      job_result
        .clone()
        .with_status(JobStatus::Error)
        .with_message(&e.to_string()),
    )
  })?;

  let mut manifest: Manifest = from_str(&contents).map_err(|message| {
    MessageError::ProcessingError(
      job_result
        .clone()
        .with_status(JobStatus::Error)
        .with_message(&message),
    )
  })?;
  let ttml_file_size = if let Ok(metadata) = fs::metadata(&parameters.ttml_path) {
    metadata.len()
  } else {
    0
  };

  if let Some(url) = &parameters.reference_url {
    manifest.prefix_urls(&url);
  }

  if parameters.replace.unwrap_or(false) {
    manifest.remove_adaptation_set(&parameters.ttml_language, &parameters.ttml_role);
  }
  let adaptation_set = AdaptationSet::new_ttml_subtitle(
    &reference_ttml_path,
    &parameters.ttml_language,
    &parameters.ttml_role,
    ttml_file_size,
  );
  manifest.add_adaptation_set(adaptation_set);

  let updated_manifest = to_string(&manifest).map_err(|message| {
    MessageError::ProcessingError(
      job_result
        .clone()
        .with_status(JobStatus::Error)
        .with_message(&message),
    )
  })?;

  let manifest_path = parameters.source_path.clone();
  let destination_manifest_path = parameters.destination_path.unwrap_or_else(|| manifest_path);
  fs::write(destination_manifest_path, &updated_manifest.into_bytes()).map_err(|e| {
    MessageError::ProcessingError(
      job_result
        .clone()
        .with_status(JobStatus::Error)
        .with_message(&e.to_string()),
    )
  })?;

  Ok(())
}

#[test]
fn add_subtitle_ttml_track() {
  let parameters = DashManifestParameters {
    source_path: "tests/sample_1.mpd".to_string(),
    ttml_path: "tests/sample_subtitle.ttml".to_string(),
    ttml_language: "fra".to_string(),
    ttml_role: "subtitle".to_string(),
    replace: None,
    destination_path: Some("tests/sample_1_updated.mpd".to_string()),
    reference_url: None,
  };
  let job_result = JobResult::new(666);
  add_ttml_subtitle(job_result, parameters).unwrap();

  let reference = fs::read_to_string("tests/sample_1_for_validation.mpd").unwrap();
  let content = fs::read_to_string("tests/sample_1_updated.mpd").unwrap();

  assert_eq!(content, reference);
}

#[test]
fn replace_subtitle_ttml_track_with_reference() {
  let parameters = DashManifestParameters {
    source_path: "tests/sample_1.mpd".to_string(),
    ttml_path: "tests/sample_subtitle.ttml".to_string(),
    ttml_language: "qaa".to_string(),
    ttml_role: "subtitle".to_string(),
    replace: Some(false),
    destination_path: Some("tests/sample_1_replaced.mpd".to_string()),
    reference_url: Some("http://server.com/dash/manifest.mpd".to_string()),
  };
  let job_result = JobResult::new(666);

  add_ttml_subtitle(job_result, parameters).unwrap();

  let reference = fs::read_to_string("tests/sample_1_for_replacement.mpd").unwrap();
  let content = fs::read_to_string("tests/sample_1_replaced.mpd").unwrap();

  assert_eq!(content, reference);
}

#[test]
fn add_http_subtitle_ttml_track() {
  let parameters = DashManifestParameters {
    source_path: "tests/sample_1.mpd".to_string(),
    ttml_path: "http://server/static/sample_subtitle.ttml".to_string(),
    ttml_language: "fra".to_string(),
    ttml_role: "subtitle".to_string(),
    replace: Some(false),
    destination_path: Some("tests/sample_1_http_ttml.mpd".to_string()),
    reference_url: None,
  };
  let job_result = JobResult::new(666);

  add_ttml_subtitle(job_result, parameters).unwrap();

  let reference = fs::read_to_string("tests/sample_1_for_http.mpd").unwrap();
  let content = fs::read_to_string("tests/sample_1_http_ttml.mpd").unwrap();

  assert_eq!(content, reference);
}
