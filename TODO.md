# TODO: `mlt-rs` 🎬🦀

A safe, modern Rust rewrite of the Media Lovin' Toolkit ([mltframework/mlt](https://github.com/mltframework/mlt)) — the powerful multi-track non-linear video editing engine powering Shotcut, Kdenlive, and broadcast playout systems.

---

## 🎯 Mission & Goals

- **Safe Non-Linear Multimedia Architecture**: Eliminate raw pointer arithmetic, threading races, and memory corruption inherent in C-based MLT while preserving its proven object model.
- **Round-Trip MLT XML Compatibility**: Full parser and serializer for MLT XML project files created by Shotcut, Kdenlive, and automated rendering pipelines.
- **Frame-Accurate Evaluation**: Deterministic multi-track compositing graph with zero-cost frame caching and audio-video synchronization.
- **Headless Cloud & Edge Rendering**: Native ability to run headless in WebAssembly, Linux containers, or edge workers for programmatic video generation.

---

## 🏗️ Crates Architecture Plan

- [ ] `mlt-core`: Core abstractions (`Service`, `Producer`, `Filter`, `Transition`, `Tractor`, `Playlist`, `Consumer`, `Frame`, `Profile`).
- [ ] `mlt-xml`: Serialization and deserialization of the standard MLT XML schema.
- [ ] `mlt-producers`: Media file producers (FFmpeg, Image, Color, SVG/Text, Tone generator).
- [ ] `mlt-filters`: Video & audio filter stack (Affine, Opacity, Blur, Volume, Panning, EQ, Color Balance).
- [ ] `mlt-transitions`: Multi-track blending (Dissolve, Wipe, Matte, Affine/PIP compositor).
- [ ] `mlt-consumers`: Playout consumers (FFmpeg render consumer, Raw buffer consumer, SDL preview).
- [ ] `melt-rs`: Standalone CLI playback and rendering utility matching upstream `melt`.

---

## 📋 Implementation Checklist

### Phase 1: Core Service Model & Frame Pipeline
- [ ] Implement the fundamental MLT service hierarchy:
  ```rust
  pub trait Service: Send + Sync {
      fn id(&self) -> &str;
      fn properties(&self) -> &Properties;
      fn properties_mut(&mut self) -> &mut Properties;
  }

  pub trait Producer: Service {
      fn get_frame(&mut self, timecode: Position) -> Result<Arc<Frame>, MltError>;
      fn length(&self) -> Position;
  }
  ```
- [ ] Implement `Frame` buffer with copy-on-write image slices (`YUV420P`, `RGBA`, `YUYV`) and planar audio chunks.
- [ ] Implement `Profile` (resolution, aspect ratio, frame rate numerator/denominator, colorspace).
- [ ] Build property dictionary supporting keyframed animated properties.

### Phase 2: Playlists, Cuts & Tractors
- [ ] **Playlist**: Sequential timeline container managing multiple clips with `in` and `out` points, blank spans, and transitions between contiguous items.
- [ ] **Multitrack & Tractor**: Hierarchical container combining multiple tracks (playlists) evaluated simultaneously.
- [ ] **Transition Routing**: Layer-based blending connecting Track A and Track B using track indices.

### Phase 3: MLT XML Compatibility
- [ ] Parser for `<mlt>`, `<profile>`, `<producer>`, `<playlist>`, `<tractor>`, `<multitrack>`, `<filter>`, `<transition>`.
- [ ] XML Serializer generating 100% valid MLT XML loadable by Shotcut and Kdenlive.
- [ ] Support for inline filter parameters and `<property name="...">` keyframe strings.

### Phase 4: Filter & Transition Ecosystem
- [ ] **Transitions**:
  - [ ] `composite` / `affine`: 2D transformation, scale, rotate, opacity, blend modes.
  - [ ] `luma`: Luma-wipe video transitions with gradient wipe files.
  - [ ] `matte`: Alpha matte masking.
- [ ] **Video Filters**:
  - [ ] Brightness, contrast, color temperature, and 3D LUT application.
  - [ ] Box blur, gaussian blur, crop, and pad.
- [ ] **Audio Filters**:
  - [ ] Gain / volume with keyframed fade-in and fade-out.
  - [ ] Channel pan, stereo balance, and peak meter diagnostics.

### Phase 5: Producers & Render Pipeline
- [ ] Media File Producer using FFmpeg / `ffmpeg-next` / native demuxers.
- [ ] Color Producer (solid RGBA frames).
- [ ] Render Consumer: Output to MP4/ProRes/WebM via FFmpeg or native encoders.
- [ ] Frame cache with LRU eviction and memory bounds.

### Phase 6: `melt-rs` CLI
- [ ] Implement command line runner for `.mlt` files:
  ```bash
  melt-rs project.mlt -consumer avformat:output.mp4 vcodec=libx264 acodec=aac
  ```
- [ ] Progress reporting and multi-threaded parallel frame rendering.

### Phase 7: Benchmarks & Parity Tests
- [ ] Render speed comparison between C `melt` and `mlt-rs` on identical MLT XML timelines.
- [ ] Pixel-diff test harness ensuring 1:1 visual parity across transition boundaries.
