use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Frame {
    pub position: i64,
    pub width: u32,
    pub height: u32,
    pub image_data: Vec<u8>,
    pub audio_samples: Vec<f32>,
    pub sample_rate: u32,
    pub channels: u16,
}

impl Frame {
    pub fn new_black(position: i64, width: u32, height: u32) -> Self {
        let size = (width * height * 4) as usize;
        let mut image_data = vec![0u8; size];
        // Set alpha channel to 255
        for i in (3..size).step_by(4) {
            image_data[i] = 255;
        }

        Self {
            position,
            width,
            height,
            image_data,
            audio_samples: vec![0.0; 1600], // ~1 frame at 48kHz (48000 / 30 = 1600)
            sample_rate: 48000,
            channels: 2,
        }
    }

    pub fn new_color(position: i64, width: u32, height: u32, r: u8, g: u8, b: u8, a: u8) -> Self {
        let size = (width * height * 4) as usize;
        let mut image_data = Vec::with_capacity(size);
        for _ in 0..(width * height) {
            image_data.push(r);
            image_data.push(g);
            image_data.push(b);
            image_data.push(a);
        }

        Self {
            position,
            width,
            height,
            image_data,
            audio_samples: vec![0.0; 1600],
            sample_rate: 48000,
            channels: 2,
        }
    }
}
