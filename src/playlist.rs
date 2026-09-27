use crate::error::Result;
use crate::frame::Frame;
use crate::producer::Producer;
use crate::profile::Profile;

pub struct PlaylistEntry {
    pub producer: Box<dyn Producer>,
    pub in_point: i64,
    pub out_point: i64,
    pub start_timecode: i64,
}

pub struct Playlist {
    pub id: String,
    pub entries: Vec<PlaylistEntry>,
}

impl Playlist {
    pub fn new<S: Into<String>>(id: S) -> Self {
        Self {
            id: id.into(),
            entries: Vec::new(),
        }
    }

    pub fn append(&mut self, producer: Box<dyn Producer>, in_point: i64, out_point: i64) {
        let start_timecode = self.total_frames();
        self.entries.push(PlaylistEntry {
            producer,
            in_point,
            out_point,
            start_timecode,
        });
    }

    pub fn total_frames(&self) -> i64 {
        self.entries
            .iter()
            .map(|e| (e.out_point - e.in_point + 1).max(0))
            .sum()
    }

    pub fn get_frame(&mut self, position: i64, profile: &Profile) -> Result<Frame> {
        let mut offset = 0;
        for entry in &mut self.entries {
            let len = (entry.out_point - entry.in_point + 1).max(0);
            if position >= offset && position < offset + len {
                let local_pos = position - offset;
                return entry.producer.get_frame(local_pos, profile);
            }
            offset += len;
        }

        // Return black frame if past playlist duration
        Ok(Frame::new_black(position, profile.width, profile.height))
    }
}
