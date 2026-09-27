# mlt-rs 🎬🦀

[![Crates.io](https://img.shields.io/badge/crates.io-v0.0.1-orange.svg)](https://crates.io/crates/mlt-rs)
[![Documentation](https://img.shields.io/badge/docs-GitHub_Pages-blue.svg)](http://code.brandonhubbard.com/mlt-rs/)
[![License](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](LICENSE-MIT)
[![MLT Framework Compatible](https://img.shields.io/badge/MLT-Framework_Compatible-yellow.svg)](https://github.com/mltframework/mlt)

High-performance, memory-safe non-linear video editing engine and `melt` CLI inspired by the [Media Lovin' Toolkit (MLT)](https://github.com/mltframework/mlt) in pure Rust.

The multimedia framework powering Shotcut and Kdenlive, completely modernized in safe, high-speed Rust with zero memory corruption, multi-track tractor compositing, keyframed animation interpolation, and native MLT XML interchange.

---

## 🚀 Features

- **Core Object Model**:
  - `Profile`: Broadcast standards (1080p24/30/60, 4K UHD, 720p, 9:16 vertical mobile) with DAR & PAR.
  - `Producer`: Media clips, solid colors, text overlays, and audio tone generators.
  - `Playlist`: Single-track linear cuts with in/out boundaries and blank spans.
  - `Tractor`: Multi-track compositor evaluating simultaneous layers and cross-track transitions.
  - `Transition`: Dissolve crossfades, affine compositing, and alpha blend modes.
  - `Filter`: Video brightness, volume gain, and opacity stack.
  - `Properties`: Timecode and frame-accurate keyframed animation interpolator (`0=0.0;30=1.0`).
- **MLT XML Interchange**: Full round-trip serializer and parser compatible with Shotcut and Kdenlive project files.
- **Headless CLI Playout**: `melt` CLI for rendering multi-track timelines to destination files or stdout.
- **Interactive Documentation**: Test multi-track scrubbing and inspect MLT XML at [code.brandonhubbard.com/mlt-rs](http://code.brandonhubbard.com/mlt-rs/).

---

## 📦 Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
mlt-rs = "0.0.1"
```

Or install the standalone `melt` tool:

```bash
cargo install mlt-rs
```

---

## 🛠️ CLI Usage

```bash
# Play / render an MLT XML project
melt project.mlt --profile hd1080p30 -consumer render_output.mp4

# Generate color test pattern on vertical TikTok profile
melt color:red --profile vertical --out 90
```

---

## 💻 Rust SDK Example

```rust
use mlt_rs::{Tractor, Playlist, ClipProducer, Profile, DissolveTransition, TractorTransition};

fn main() -> anyhow::Result<()> {
    let profile = Profile::hd_1080_30p();
    let mut tractor = Tractor::new("master_timeline");

    // Track 0: Main Video
    let mut track0 = Playlist::new("track0");
    track0.append(Box::new(ClipProducer::new("c1", "interview.mp4", 0, 59)), 0, 59);

    // Track 1: B-Roll overlay with dissolve
    let mut track1 = Playlist::new("track1");
    track1.append(Box::new(ClipProducer::new("c2", "broll.mp4", 0, 59)), 0, 59);

    tractor.add_track(track0);
    tractor.add_track(track1);

    tractor.add_transition(TractorTransition {
        a_track: 0,
        b_track: 1,
        in_frame: 45,
        out_frame: 60,
        transition: Box::new(DissolveTransition),
    });

    let frame = tractor.get_frame(50, &profile)?;
    println!("Rendered frame at t=50: {}x{} RGBA", frame.width, frame.height);
    Ok(())
}
```

---

## 🧪 Testing

Run all unit and integration tests:

```bash
cargo test
```

---

## 📄 License

Licensed under either of:
- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT license ([LICENSE-MIT](LICENSE-MIT))
