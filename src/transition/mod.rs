use crate::error::Result;
use crate::frame::Frame;

pub trait Transition: Send + Sync {
    fn process(&self, frame_a: &Frame, frame_b: &Frame, progress: f32) -> Result<Frame>;
}

pub struct DissolveTransition;

impl Transition for DissolveTransition {
    fn process(&self, frame_a: &Frame, frame_b: &Frame, progress: f32) -> Result<Frame> {
        let t = progress.clamp(0.0, 1.0);
        let inv_t = 1.0 - t;

        let width = frame_a.width;
        let height = frame_a.height;
        let len = (width * height * 4) as usize;
        let mut image_data = vec![0u8; len];

        for i in 0..len {
            let val_a = frame_a.image_data.get(i).copied().unwrap_or(0) as f32;
            let val_b = frame_b.image_data.get(i).copied().unwrap_or(0) as f32;
            image_data[i] = ((inv_t * val_a + t * val_b).clamp(0.0, 255.0)) as u8;
        }

        // Crossfade audio
        let audio_len = frame_a.audio_samples.len().min(frame_b.audio_samples.len());
        let mut audio_samples = Vec::with_capacity(audio_len);
        for i in 0..audio_len {
            audio_samples.push(inv_t * frame_a.audio_samples[i] + t * frame_b.audio_samples[i]);
        }

        Ok(Frame {
            position: frame_a.position,
            width,
            height,
            image_data,
            audio_samples,
            sample_rate: frame_a.sample_rate,
            channels: frame_a.channels,
        })
    }
}
