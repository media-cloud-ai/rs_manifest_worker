use std::io::{Read, Write};

use yaserde::{YaDeserialize, YaSerialize};

use crate::utils;

#[derive(Debug, YaDeserialize, YaSerialize)]
#[yaserde(root = "smil", namespace = "http://www.w3.org/2001/SMIL20/Language")]
pub struct Smil {
  head: Head,
  body: Body,
}

impl Smil {
  pub fn get_audio_stream_sources(&self) -> Vec<String> {
    utils::remove_duplicates(self.body.switch.audio.iter()
        .map(|audio| audio.source.clone())
        .collect())
  }

  pub fn get_video_stream_sources(&self) -> Vec<String> {
    utils::remove_duplicates(self.body.switch.video.iter()
        .map(|video| video.source.clone())
        .collect())
  }

  pub fn get_text_stream_sources(&self) -> Vec<String> {
    utils::remove_duplicates(self.body.switch.text.iter()
        .map(|text| text.source.clone())
        .collect())
  }
}

#[derive(Debug, YaDeserialize, YaSerialize)]
struct Head {
  meta: Vec<Meta>,
  #[yaserde(rename = "paramGroup")]
  param_group: Vec<ParamGroup>,
}

impl Default for Head {
  fn default() -> Self {
    Head {
      meta: vec![],
      param_group: vec![],
    }
  }
}

#[derive(Debug, YaDeserialize, YaSerialize)]
struct Meta {
  #[yaserde(attribute, rename = "name")]
  name_: String,
  #[yaserde(attribute)]
  content: String,
}

#[derive(Debug, YaDeserialize, YaSerialize)]
struct ParamGroup {
  meta: Vec<Meta>,
}

#[derive(Debug, YaDeserialize, YaSerialize)]
struct Body {
  switch: Switch,
}

impl Default for Body {
  fn default() -> Self {
    Body {
      switch: Switch::default(),
    }
  }
}

#[derive(Debug, YaDeserialize, YaSerialize)]
struct Switch {
  audio: Vec<Audio>,
  video: Vec<Video>,
  #[yaserde(rename = "textstream")]
  text: Vec<TextStream>,
}

impl Default for Switch {
  fn default() -> Self {
    Switch {
      audio: vec![],
      video: vec![],
      text: vec![],
    }
  }
}

#[derive(Debug, YaDeserialize, YaSerialize)]
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
struct Video {
  #[yaserde(attribute, rename = "src")]
  source: String,
  #[yaserde(attribute, rename = "systemBitrate")]
  system_bit_rate: String,
  params: Vec<Param>,
}

#[derive(Debug, YaDeserialize, YaSerialize)]
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
struct Param {
  #[yaserde(attribute, rename = "name")]
  name_: String,
  #[yaserde(attribute)]
  value: String,
  #[yaserde(attribute, rename = "valueType")]
  value_type: String,
}
