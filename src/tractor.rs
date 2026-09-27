use crate::error::Result;
use crate::filter::Filter;
use crate::frame::Frame;
use crate::playlist::Playlist;
use crate::profile::Profile;
use crate::transition::Transition;

pub struct TractorTransition {
    pub a_track: usize,
    pub b_track: usize,
    pub in_frame: i64,
    pub out_frame: i64,
    pub transition: Box<dyn Transition>,
}

pub struct Tractor {
    pub id: String,
    pub tracks: Vec<Playlist>,
    pub transitions: Vec<TractorTransition>,
    pub filters: Vec<Box<dyn Filter>>,
}

impl Tractor {
    pub fn new<S: Into<String>>(id: S) -> Self {
        Self {
            id: id.into(),
            tracks: Vec::new(),
            transitions: Vec::new(),
            filters: Vec::new(),
        }
    }

    pub fn add_track(&mut self, track: Playlist) {
        self.tracks.push(track);
    }

    pub fn add_transition(&mut self, transition: TractorTransition) {
        self.transitions.push(transition);
    }

    pub fn add_filter(&mut self, filter: Box<dyn Filter>) {
        self.filters.push(filter);
    }

    pub fn total_frames(&self) -> i64 {
        self.tracks
            .iter()
            .map(|t| t.total_frames())
            .max()
            .unwrap_or(0)
    }

    /// Render a composited frame at the given timeline position
    pub fn get_frame(&mut self, position: i64, profile: &Profile) -> Result<Frame> {
        if self.tracks.is_empty() {
            return Ok(Frame::new_black(position, profile.width, profile.height));
        }

        // 1. Fetch frames from each track
        let mut track_frames: Vec<Frame> = Vec::new();
        for track in &mut self.tracks {
            track_frames.push(track.get_frame(position, profile)?);
        }

        // 2. Base frame is Track 0
        let mut composite = track_frames[0].clone();

        // 3. Process transitions across higher tracks
        for trans in &self.transitions {
            if position >= trans.in_frame && position <= trans.out_frame {
                if trans.b_track < track_frames.len() {
                    let frame_b = &track_frames[trans.b_track];
                    let progress = (position - trans.in_frame) as f32
                        / (trans.out_frame - trans.in_frame).max(1) as f32;
                    composite = trans.transition.process(&composite, frame_b, progress)?;
                }
            }
        }

        // 4. If no transition active but higher track has opaque content, composite on top
        if self.transitions.is_empty() && track_frames.len() > 1 {
            for track_frame in &track_frames[1..] {
                // Alpha composite onto base
                for (dest, src) in composite.image_data.chunks_exact_mut(4).zip(track_frame.image_data.chunks_exact(4)) {
                    let alpha = src[3] as f32 / 255.0;
                    let inv_alpha = 1.0 - alpha;
                    dest[0] = ((alpha * src[0] as f32 + inv_alpha * dest[0] as f32).clamp(0.0, 255.0)) as u8;
                    dest[1] = ((alpha * src[1] as f32 + inv_alpha * dest[1] as f32).clamp(0.0, 255.0)) as u8;
                    dest[2] = ((alpha * src[2] as f32 + inv_alpha * dest[2] as f32).clamp(0.0, 255.0)) as u8;
                    dest[3] = 255;
                }
            }
        }

        // 5. Apply master timeline filters
        for filter in &self.filters {
            filter.process_frame(&mut composite, position)?;
        }

        Ok(composite)
    }
}
