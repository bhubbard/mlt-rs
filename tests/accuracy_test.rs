//! Analytical and Timeline Accuracy Verification Tests for mlt-rs
//!
//! Validates:
//! 1. Broadcast profile rational framerate and aspect ratio mathematical exactness.
//! 2. Multi-segment playlist discrete timecode offset mapping without frame drift.
//! 3. Tractor multi-track parallel timeline duration max-invariance.
//! 4. Color producer RGBA byte-exact buffer synthesis.

use mlt_rs::playlist::Playlist;
use mlt_rs::producer::{ClipProducer, ColorProducer, Producer};
use mlt_rs::profile::Profile;
use mlt_rs::tractor::Tractor;

#[test]
fn test_broadcast_profile_framerate_and_dar_exactness() {
    let p24 = Profile::hd_1080_24p();
    assert_eq!(p24.fps(), 24.0);
    assert!((p24.dar() - 16.0 / 9.0).abs() < 1e-6);

    let p30 = Profile::hd_1080_30p();
    assert_eq!(p30.fps(), 30.0);
    assert!((p30.dar() - 16.0 / 9.0).abs() < 1e-6);

    let p60 = Profile::hd_1080_60p();
    assert_eq!(p60.fps(), 60.0);
    assert!((p60.dar() - 16.0 / 9.0).abs() < 1e-6);

    let uhd = Profile::uhd_2160_60p();
    assert_eq!(uhd.width, 3840);
    assert_eq!(uhd.height, 2160);
    assert_eq!(uhd.fps(), 60.0);
    assert!((uhd.dar() - 16.0 / 9.0).abs() < 1e-6);
}

#[test]
fn test_playlist_temporal_frame_mapping_exactness() {
    let profile = Profile::hd_1080_30p();
    let mut playlist = Playlist::new("main_edit");

    // Clip 1: 50 frames (in 20 to out 69)
    let clip1 = ClipProducer::new("clip1", "file1.mov", 20, 69);
    playlist.append(Box::new(clip1), 20, 69);

    // Clip 2: 100 frames (in 100 to out 199)
    let clip2 = ClipProducer::new("clip2", "file2.mov", 100, 199);
    playlist.append(Box::new(clip2), 100, 199);

    assert_eq!(playlist.total_frames(), 150);

    // Verify exact frame mapping
    let f0 = playlist.get_frame(0, &profile).expect("frame 0");
    assert_eq!(f0.position, 20, "Global 0 must map to Clip 1 start (20)");

    let f49 = playlist.get_frame(49, &profile).expect("frame 49");
    assert_eq!(f49.position, 69, "Global 49 must map to Clip 1 end (69)");

    let f50 = playlist.get_frame(50, &profile).expect("frame 50");
    assert_eq!(f50.position, 100, "Global 50 must map to Clip 2 start (100)");

    let f149 = playlist.get_frame(149, &profile).expect("frame 149");
    assert_eq!(f149.position, 199, "Global 149 must map to Clip 2 end (199)");

    // Frame past boundary must return clean black frame
    let f_past = playlist.get_frame(150, &profile).expect("frame 150");
    assert_eq!(f_past.position, 150);
    // Alpha channel must be 255, RGB must be 0
    assert_eq!(f_past.image_data[0], 0);
    assert_eq!(f_past.image_data[1], 0);
    assert_eq!(f_past.image_data[2], 0);
    assert_eq!(f_past.image_data[3], 255);
}

#[test]
fn test_tractor_multitrack_composition_duration() {
    let mut tractor = Tractor::new("composition");

    let mut track1 = Playlist::new("track1");
    let clip1 = ClipProducer::new("c1", "a.mov", 0, 74); // 75 frames
    track1.append(Box::new(clip1), 0, 74);

    let mut track2 = Playlist::new("track2");
    let clip2 = ClipProducer::new("c2", "b.mov", 0, 179); // 180 frames
    track2.append(Box::new(clip2), 0, 179);

    tractor.add_track(track1);
    tractor.add_track(track2);

    assert_eq!(
        tractor.total_frames(),
        180,
        "Tractor duration must equal maximum track length"
    );
}

#[test]
fn test_color_producer_pixel_exactness() {
    let profile = Profile::hd_1080_30p();
    let mut color = ColorProducer::new("solid_red", 220, 110, 45, 255, 30);
    let frame = color.get_frame(5, &profile).expect("color frame");

    assert_eq!(frame.width, 1920);
    assert_eq!(frame.height, 1080);
    assert_eq!(frame.image_data.len(), 1920 * 1080 * 4);

    // Verify first pixel and middle pixel
    assert_eq!(
        &frame.image_data[0..4],
        &[220, 110, 45, 255],
        "Pixel 0 color mismatch"
    );
    let mid_idx = (1920 * 540 + 960) * 4;
    assert_eq!(
        &frame.image_data[mid_idx..mid_idx + 4],
        &[220, 110, 45, 255],
        "Center pixel color mismatch"
    );
}
