use crate::error::Result;
use crate::frame::Frame;

pub trait Filter: Send + Sync {
    fn process_frame(&self, frame: &mut Frame, position: i64) -> Result<()>;
}

pub struct BrightnessFilter {
    pub factor: f32, // 1.0 = normal, 0.5 = dark, 1.5 = bright
}

impl BrightnessFilter {
    pub fn new(factor: f32) -> Self {
        Self { factor }
    }
}

impl Filter for BrightnessFilter {
    fn process_frame(&self, frame: &mut Frame, _position: i64) -> Result<()> {
        for pixel in frame.image_data.chunks_exact_mut(4) {
            pixel[0] = ((pixel[0] as f32 * self.factor).clamp(0.0, 255.0)) as u8;
            pixel[1] = ((pixel[1] as f32 * self.factor).clamp(0.0, 255.0)) as u8;
            pixel[2] = ((pixel[2] as f32 * self.factor).clamp(0.0, 255.0)) as u8;
        }
        Ok(())
    }
}

pub struct VolumeFilter {
    pub gain: f32, // 1.0 = 0dB, 0.0 = mute
}

impl VolumeFilter {
    pub fn new(gain: f32) -> Self {
        Self { gain }
    }
}

impl Filter for VolumeFilter {
    fn process_frame(&self, frame: &mut Frame, _position: i64) -> Result<()> {
        for sample in &mut frame.audio_samples {
            *sample *= self.gain;
        }
        Ok(())
    }
}

pub struct OpacityFilter {
    pub opacity: f32, // 0.0 to 1.0
}

impl OpacityFilter {
    pub fn new(opacity: f32) -> Self {
        Self { opacity }
    }
}

impl Filter for OpacityFilter {
    fn process_frame(&self, frame: &mut Frame, _position: i64) -> Result<()> {
        for pixel in frame.image_data.chunks_exact_mut(4) {
            pixel[3] = ((pixel[3] as f32 * self.opacity).clamp(0.0, 255.0)) as u8;
        }
        Ok(())
    }
}
