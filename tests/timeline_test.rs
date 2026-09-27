use mlt_rs::filter::{BrightnessFilter, OpacityFilter, VolumeFilter, Filter};
use mlt_rs::frame::Frame;
use mlt_rs::playlist::Playlist;
use mlt_rs::producer::{ClipProducer, ColorProducer};
use mlt_rs::profile::Profile;
use mlt_rs::properties::Properties;
use mlt_rs::tractor::{Tractor, TractorTransition};
use mlt_rs::transition::DissolveTransition;

#[test]
fn test_properties_animation() {
    let mut props = Properties::new();
    props.set("opacity", "0=0.0;30=1.0;60=0.5");

    assert_eq!(props.get_animated_float("opacity", 0, 0.0), 0.0);
    assert_eq!(props.get_animated_float("opacity", 30, 0.0), 1.0);
    assert_eq!(props.get_animated_float("opacity", 15, 0.0), 0.5); // Midpoint
    assert_eq!(props.get_animated_float("opacity", 60, 0.0), 0.5);
    assert_eq!(props.get_animated_float("opacity", 100, 0.0), 0.5); // Clamped
}

#[test]
fn test_single_track_playlist() -> anyhow::Result<()> {
    let profile = Profile::hd_1080_30p();
    let mut playlist = Playlist::new("track0");

    playlist.append(Box::new(ClipProducer::new("c1", "clip1.mp4", 0, 29)), 0, 29);
    playlist.append(Box::new(ClipProducer::new("c2", "clip2.mp4", 10, 39)), 10, 39);

    assert_eq!(playlist.total_frames(), 60);

    let f0 = playlist.get_frame(0, &profile)?;
    assert_eq!(f0.width, 1920);
    assert_eq!(f0.height, 1080);

    let f50 = playlist.get_frame(50, &profile)?;
    assert_eq!(f50.width, 1920);
    Ok(())
}

#[test]
fn test_tractor_dissolve_transition() -> anyhow::Result<()> {
    let profile = Profile::hd_1080_30p();
    let mut tractor = Tractor::new("main");

    // Track 0: Solid red (255, 0, 0)
    let mut t0 = Playlist::new("track0");
    t0.append(Box::new(ColorProducer::new("red", 255, 0, 0, 255, 61)), 0, 60);

    // Track 1: Solid blue (0, 0, 255)
    let mut t1 = Playlist::new("track1");
    t1.append(Box::new(ColorProducer::new("blue", 0, 0, 255, 255, 61)), 0, 60);

    tractor.add_track(t0);
    tractor.add_track(t1);

    // Crossfade dissolve between frame 0 and 60
    tractor.add_transition(TractorTransition {
        a_track: 0,
        b_track: 1,
        in_frame: 0,
        out_frame: 60,
        transition: Box::new(DissolveTransition),
    });

    // Frame 0 (Start): pure red
    let f_start = tractor.get_frame(0, &profile)?;
    assert_eq!(f_start.image_data[0], 255); // R
    assert_eq!(f_start.image_data[2], 0);   // B

    // Frame 30 (Middle 50%): purple blend ~ (127, 0, 127)
    let f_mid = tractor.get_frame(30, &profile)?;
    assert!((f_mid.image_data[0] as i32 - 127).abs() <= 2);
    assert!((f_mid.image_data[2] as i32 - 127).abs() <= 2);

    // Frame 60 (End): pure blue
    let f_end = tractor.get_frame(60, &profile)?;
    assert_eq!(f_end.image_data[0], 0);   // R
    assert_eq!(f_end.image_data[2], 255); // B

    Ok(())
}

#[test]
fn test_filters() -> anyhow::Result<()> {
    let mut frame = Frame::new_color(0, 10, 10, 100, 100, 100, 255);

    // Brightness filter
    let bright = BrightnessFilter::new(1.5);
    bright.process_frame(&mut frame, 0)?;
    assert_eq!(frame.image_data[0], 150);

    // Opacity filter
    let op = OpacityFilter::new(0.5);
    op.process_frame(&mut frame, 0)?;
    assert_eq!(frame.image_data[3], 127);

    // Volume filter
    frame.audio_samples = vec![1.0, -1.0, 0.5];
    let vol = VolumeFilter::new(0.5);
    vol.process_frame(&mut frame, 0)?;
    assert_eq!(frame.audio_samples, vec![0.5, -0.5, 0.25]);

    Ok(())
}
