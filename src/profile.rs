use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Profile {
    pub description: String,
    pub width: u32,
    pub height: u32,
    pub frame_rate_num: u32,
    pub frame_rate_den: u32,
    pub sample_aspect_num: u32,
    pub sample_aspect_den: u32,
    pub display_aspect_num: u32,
    pub display_aspect_den: u32,
    pub progressive: bool,
    pub colorspace: u32,
}

impl Default for Profile {
    fn default() -> Self {
        Self::hd_1080_30p()
    }
}

impl Profile {
    pub fn fps(&self) -> f64 {
        self.frame_rate_num as f64 / self.frame_rate_den.max(1) as f64
    }

    pub fn dar(&self) -> f64 {
        self.display_aspect_num as f64 / self.display_aspect_den.max(1) as f64
    }

    pub fn hd_1080_24p() -> Self {
        Self {
            description: "HD 1080p 24 fps (Cinema)".to_string(),
            width: 1920,
            height: 1080,
            frame_rate_num: 24,
            frame_rate_den: 1,
            sample_aspect_num: 1,
            sample_aspect_den: 1,
            display_aspect_num: 16,
            display_aspect_den: 9,
            progressive: true,
            colorspace: 709,
        }
    }

    pub fn hd_1080_30p() -> Self {
        Self {
            description: "HD 1080p 30 fps (Broadcast)".to_string(),
            width: 1920,
            height: 1080,
            frame_rate_num: 30,
            frame_rate_den: 1,
            sample_aspect_num: 1,
            sample_aspect_den: 1,
            display_aspect_num: 16,
            display_aspect_den: 9,
            progressive: true,
            colorspace: 709,
        }
    }

    pub fn hd_1080_60p() -> Self {
        Self {
            description: "HD 1080p 60 fps (High Motion)".to_string(),
            width: 1920,
            height: 1080,
            frame_rate_num: 60,
            frame_rate_den: 1,
            sample_aspect_num: 1,
            sample_aspect_den: 1,
            display_aspect_num: 16,
            display_aspect_den: 9,
            progressive: true,
            colorspace: 709,
        }
    }

    pub fn uhd_2160_60p() -> Self {
        Self {
            description: "4K UHD 2160p 60 fps".to_string(),
            width: 3840,
            height: 2160,
            frame_rate_num: 60,
            frame_rate_den: 1,
            sample_aspect_num: 1,
            sample_aspect_den: 1,
            display_aspect_num: 16,
            display_aspect_den: 9,
            progressive: true,
            colorspace: 2020,
        }
    }

    pub fn vertical_hd_30p() -> Self {
        Self {
            description: "Vertical 9:16 (Shorts/TikTok/Reels)".to_string(),
            width: 1080,
            height: 1920,
            frame_rate_num: 30,
            frame_rate_den: 1,
            sample_aspect_num: 1,
            sample_aspect_den: 1,
            display_aspect_num: 9,
            display_aspect_den: 16,
            progressive: true,
            colorspace: 709,
        }
    }

    pub fn from_name(name: &str) -> Option<Self> {
        match name.to_lowercase().replace(['-', '_', ' '], "").as_str() {
            "hd1080p24" | "atsc1080p24" | "1080p24" => Some(Self::hd_1080_24p()),
            "hd1080p30" | "atsc1080p30" | "1080p30" => Some(Self::hd_1080_30p()),
            "hd1080p60" | "atsc1080p60" | "1080p60" => Some(Self::hd_1080_60p()),
            "uhd2160p60" | "4k60" | "2160p60" => Some(Self::uhd_2160_60p()),
            "vertical" | "shorts" | "tiktok" | "9x16" => Some(Self::vertical_hd_30p()),
            _ => None,
        }
    }
}
