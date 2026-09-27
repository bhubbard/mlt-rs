use tempfile::tempdir;
use mlt_rs::consumer::RenderConsumer;
use mlt_rs::playlist::Playlist;
use mlt_rs::producer::ColorProducer;
use mlt_rs::profile::Profile;
use mlt_rs::tractor::Tractor;

#[test]
fn test_render_consumer() -> anyhow::Result<()> {
    let tmp = tempdir()?;
    let dest = tmp.path().join("render_output.dat");

    let profile = Profile::hd_1080_30p();
    let mut tractor = Tractor::new("render_tractor");
    let mut track = Playlist::new("track0");

    track.append(
        Box::new(ColorProducer::new("green", 0, 255, 0, 255, 30)),
        0,
        29,
    );
    tractor.add_track(track);

    let mut last_percent = 0.0;
    RenderConsumer::render_to_file(&mut tractor, &profile, &dest, |prog| {
        last_percent = prog.percent;
    })?;

    assert_eq!(last_percent, 100.0);
    assert!(dest.exists());

    let content = std::fs::read_to_string(&dest)?;
    assert!(content.contains("MLT_RENDER_V1"));
    assert!(content.contains("[FRAME:000029"));

    Ok(())
}
