use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::time::Instant;
use crate::error::Result;
use crate::profile::Profile;
use crate::tractor::Tractor;

#[derive(Debug, Clone)]
pub struct RenderProgress {
    pub current_frame: i64,
    pub total_frames: i64,
    pub percent: f32,
    pub fps: f32,
    pub eta_seconds: u64,
}

pub struct RenderConsumer;

impl RenderConsumer {
    pub fn render_to_file<P: AsRef<Path>, F>(
        tractor: &mut Tractor,
        profile: &Profile,
        destination: P,
        mut progress_callback: F,
    ) -> Result<()>
    where
        F: FnMut(&RenderProgress),
    {
        let total_frames = tractor.total_frames().max(1);
        let start_time = Instant::now();

        if let Some(parent) = destination.as_ref().parent() {
            std::fs::create_dir_all(parent)?;
        }

        let mut out_file = File::create(destination)?;
        let header = format!(
            "MLT_RENDER_V1 | Resolution: {}x{} | FPS: {:.2} | Frames: {}\n",
            profile.width,
            profile.height,
            profile.fps(),
            total_frames
        );
        out_file.write_all(header.as_bytes())?;

        for f in 0..total_frames {
            let _frame = tractor.get_frame(f, profile)?;
            let elapsed = start_time.elapsed().as_secs_f32();
            let fps = if elapsed > 0.05 { (f + 1) as f32 / elapsed } else { 0.0 };
            let remaining = total_frames - (f + 1);
            let eta = if fps > 0.1 { (remaining as f32 / fps).round() as u64 } else { 0 };

            progress_callback(&RenderProgress {
                current_frame: f + 1,
                total_frames,
                percent: (((f + 1) as f32 / total_frames as f32) * 100.0).clamp(0.0, 100.0),
                fps,
                eta_seconds: eta,
            });

            // Write frame marker
            let marker = format!("[FRAME:{:06}|TIME:{:.3}s]\n", f, f as f64 / profile.fps());
            out_file.write_all(marker.as_bytes())?;
        }

        out_file.flush()?;
        Ok(())
    }
}
