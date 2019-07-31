use std::io::{Read, Write};

use yaserde::{YaDeserialize, YaSerialize};

use crate::utils;

#[derive(Debug, YaDeserialize, YaSerialize)]
#[yaserde(
  root = "smil",
  prefix = "smil"
  namespace = "smil: http://www.w3.org/2001/SMIL20/Language"
)]
pub struct Smil {
  head: Head,
  body: Body,
}

impl Smil {
  pub fn get_audio_stream_sources(&self) -> Vec<String> {
    utils::remove_duplicates(
      self
        .body
        .switch
        .audio
        .iter()
        .map(|audio| audio.source.clone())
        .collect(),
    )
  }

  pub fn get_video_stream_sources(&self) -> Vec<String> {
    utils::remove_duplicates(
      self
        .body
        .switch
        .video
        .iter()
        .map(|video| video.source.clone())
        .collect(),
    )
  }

  pub fn get_text_stream_sources(&self) -> Vec<String> {
    utils::remove_duplicates(
      self
        .body
        .switch
        .text
        .iter()
        .map(|text| text.source.clone())
        .collect(),
    )
  }
}

#[derive(Debug, Default, YaDeserialize, YaSerialize)]
#[yaserde(
  prefix = "smil"
  namespace = "smil: http://www.w3.org/2001/SMIL20/Language"
)]
struct Head {
  meta: Vec<Meta>,
  #[yaserde(rename = "paramGroup")]
  param_group: Vec<ParamGroup>,
}

#[derive(Debug, YaDeserialize, YaSerialize)]
#[yaserde(
  prefix = "smil"
  namespace = "smil: http://www.w3.org/2001/SMIL20/Language"
)]
struct Meta {
  #[yaserde(attribute, rename = "name")]
  name_: String,
  #[yaserde(attribute)]
  content: String,
}

#[derive(Debug, YaDeserialize, YaSerialize)]
#[yaserde(
  prefix = "smil"
  namespace = "smil: http://www.w3.org/2001/SMIL20/Language"
)]
struct ParamGroup {
  meta: Vec<Meta>,
}

#[derive(Debug, Default, YaDeserialize, YaSerialize)]
#[yaserde(
  prefix = "smil"
  namespace = "smil: http://www.w3.org/2001/SMIL20/Language"
)]
struct Body {
  switch: Switch,
}

#[derive(Debug, Default, YaDeserialize, YaSerialize)]
#[yaserde(
  prefix = "smil"
  namespace = "smil: http://www.w3.org/2001/SMIL20/Language"
)]
struct Switch {
  audio: Vec<Audio>,
  video: Vec<Video>,
  #[yaserde(rename = "textstream")]
  text: Vec<TextStream>,
}

#[derive(Debug, YaDeserialize, YaSerialize)]
#[yaserde(
  prefix = "smil"
  namespace = "smil: http://www.w3.org/2001/SMIL20/Language"
)]
struct Audio {
  #[yaserde(attribute, rename = "src")]
  source: String,
  #[yaserde(attribute, rename = "systemBitrate")]
  system_bit_rate: String,
  #[yaserde(attribute, rename = "systemLanguage")]
  system_language: String,
  params: Vec<Param>,
}

#[derive(Debug, YaDeserialize, YaSerialize)]
#[yaserde(
  prefix = "smil"
  namespace = "smil: http://www.w3.org/2001/SMIL20/Language"
)]
struct Video {
  #[yaserde(attribute, rename = "src")]
  source: String,
  #[yaserde(attribute, rename = "systemBitrate")]
  system_bit_rate: String,
  params: Vec<Param>,
}

#[derive(Debug, YaDeserialize, YaSerialize)]
#[yaserde(
  prefix = "smil"
  namespace = "smil: http://www.w3.org/2001/SMIL20/Language"
)]
struct TextStream {
  #[yaserde(attribute, rename = "src")]
  source: String,
  #[yaserde(attribute, rename = "systemBitrate")]
  system_bit_rate: String,
  #[yaserde(attribute, rename = "systemLanguage")]
  system_language: String,
  params: Vec<Param>,
}

#[derive(Debug, YaDeserialize, YaSerialize)]
#[yaserde(
  prefix = "smil"
  namespace = "smil: http://www.w3.org/2001/SMIL20/Language"
)]
struct Param {
  #[yaserde(attribute, rename = "name")]
  name_: String,
  #[yaserde(attribute)]
  value: String,
  #[yaserde(attribute, rename = "valueType")]
  value_type: String,
}
