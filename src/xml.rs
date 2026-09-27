use serde::{Deserialize, Serialize};
use crate::error::Result;
use crate::profile::Profile;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MltXmlProfile {
    #[serde(rename = "@description")]
    pub description: String,
    #[serde(rename = "@width")]
    pub width: u32,
    #[serde(rename = "@height")]
    pub height: u32,
    #[serde(rename = "@frame_rate_num")]
    pub frame_rate_num: u32,
    #[serde(rename = "@frame_rate_den")]
    pub frame_rate_den: u32,
    #[serde(rename = "@display_aspect_num")]
    pub display_aspect_num: u32,
    #[serde(rename = "@display_aspect_den")]
    pub display_aspect_den: u32,
    #[serde(rename = "@progressive")]
    pub progressive: u32,
    #[serde(rename = "@colorspace")]
    pub colorspace: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MltXmlProperty {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "$value")]
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MltXmlProducer {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "@in")]
    pub in_point: i64,
    #[serde(rename = "@out")]
    pub out_point: i64,
    #[serde(rename = "property", default)]
    pub properties: Vec<MltXmlProperty>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MltXmlEntry {
    #[serde(rename = "@producer")]
    pub producer: String,
    #[serde(rename = "@in")]
    pub in_point: i64,
    #[serde(rename = "@out")]
    pub out_point: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MltXmlPlaylist {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "entry", default)]
    pub entries: Vec<MltXmlEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MltXmlTrack {
    #[serde(rename = "@producer")]
    pub producer: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MltXmlTractor {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "track", default)]
    pub tracks: Vec<MltXmlTrack>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename = "mlt")]
pub struct MltDocument {
    #[serde(rename = "@version", default = "default_version")]
    pub version: String,
    pub profile: MltXmlProfile,
    #[serde(rename = "producer", default)]
    pub producers: Vec<MltXmlProducer>,
    #[serde(rename = "playlist", default)]
    pub playlists: Vec<MltXmlPlaylist>,
    #[serde(rename = "tractor", default)]
    pub tractors: Vec<MltXmlTractor>,
}

fn default_version() -> String {
    "7.24.0".to_string()
}

impl MltDocument {
    pub fn new(profile: &Profile) -> Self {
        Self {
            version: "7.24.0".to_string(),
            profile: MltXmlProfile {
                description: profile.description.clone(),
                width: profile.width,
                height: profile.height,
                frame_rate_num: profile.frame_rate_num,
                frame_rate_den: profile.frame_rate_den,
                display_aspect_num: profile.display_aspect_num,
                display_aspect_den: profile.display_aspect_den,
                progressive: if profile.progressive { 1 } else { 0 },
                colorspace: profile.colorspace,
            },
            producers: Vec::new(),
            playlists: Vec::new(),
            tractors: Vec::new(),
        }
    }

    pub fn to_xml(&self) -> Result<String> {
        let xml = quick_xml::se::to_string(self)?;
        Ok(format!("<?xml version=\"1.0\" standalone=\"no\"?>\n{}", xml))
    }

    pub fn from_xml(xml: &str) -> Result<Self> {
        let doc: Self = quick_xml::de::from_str(xml)?;
        Ok(doc)
    }

    pub fn to_profile(&self) -> Profile {
        Profile {
            description: self.profile.description.clone(),
            width: self.profile.width,
            height: self.profile.height,
            frame_rate_num: self.profile.frame_rate_num,
            frame_rate_den: self.profile.frame_rate_den,
            sample_aspect_num: 1,
            sample_aspect_den: 1,
            display_aspect_num: self.profile.display_aspect_num,
            display_aspect_den: self.profile.display_aspect_den,
            progressive: self.profile.progressive == 1,
            colorspace: self.profile.colorspace,
        }
    }
}
