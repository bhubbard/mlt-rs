use crate::error::Result;
use crate::frame::Frame;
use crate::profile::Profile;
use crate::properties::Properties;

pub trait Producer: Send + Sync {
    fn id(&self) -> &str;
    fn get_frame(&mut self, position: i64, profile: &Profile) -> Result<Frame>;
    fn length(&self) -> i64;
    fn properties(&self) -> &Properties;
    fn properties_mut(&mut self) -> &mut Properties;
}

pub struct ClipProducer {
    pub id: String,
    pub resource: String,
    pub in_point: i64,
    pub out_point: i64,
    pub properties: Properties,
}

impl ClipProducer {
    pub fn new<S: Into<String>>(id: S, resource: S, in_point: i64, out_point: i64) -> Self {
        Self {
            id: id.into(),
            resource: resource.into(),
            in_point,
            out_point,
            properties: Properties::new(),
        }
    }
}

impl Producer for ClipProducer {
    fn id(&self) -> &str {
        &self.id
    }

    fn get_frame(&mut self, position: i64, profile: &Profile) -> Result<Frame> {
        let actual_pos = self.in_point + position;
        // Generate test frame with dynamic pattern
        let r = ((actual_pos * 5) % 255) as u8;
        let g = ((actual_pos * 3) % 255) as u8;
        let b = ((actual_pos * 7) % 255) as u8;
        Ok(Frame::new_color(actual_pos, profile.width, profile.height, r, g, b, 255))
    }

    fn length(&self) -> i64 {
        (self.out_point - self.in_point + 1).max(0)
    }

    fn properties(&self) -> &Properties {
        &self.properties
    }

    fn properties_mut(&mut self) -> &mut Properties {
        &mut self.properties
    }
}

pub struct ColorProducer {
    pub id: String,
    pub color_rgba: (u8, u8, u8, u8),
    pub length_frames: i64,
    pub properties: Properties,
}

impl ColorProducer {
    pub fn new<S: Into<String>>(id: S, r: u8, g: u8, b: u8, a: u8, length_frames: i64) -> Self {
        Self {
            id: id.into(),
            color_rgba: (r, g, b, a),
            length_frames,
            properties: Properties::new(),
        }
    }
}

impl Producer for ColorProducer {
    fn id(&self) -> &str {
        &self.id
    }

    fn get_frame(&mut self, position: i64, profile: &Profile) -> Result<Frame> {
        let (r, g, b, a) = self.color_rgba;
        Ok(Frame::new_color(position, profile.width, profile.height, r, g, b, a))
    }

    fn length(&self) -> i64 {
        self.length_frames
    }

    fn properties(&self) -> &Properties {
        &self.properties
    }

    fn properties_mut(&mut self) -> &mut Properties {
        &mut self.properties
    }
}
