const XML_PROJECT = `<?xml version="1.0" standalone="no"?>
<mlt LC_NUMERIC="C" version="7.24.0" title="Shotcut project">
  <profile description="HD 1080p 30 fps (Broadcast)" width="1920" height="1080" frame_rate_num="30" frame_rate_den="1" display_aspect_num="16" display_aspect_den="9" progressive="1" colorspace="709"/>
  <producer id="prod_cam_a" in="0" out="65">
    <property name="resource">Interview_Camera_A.mp4</property>
  </producer>
  <producer id="prod_broll" in="0" out="59">
    <property name="resource">B-Roll_Landscape.mp4</property>
  </producer>
  <producer id="prod_overlay" in="0" out="59">
    <property name="resource">Title_Overlay.png</property>
  </producer>
  <playlist id="playlist_v1">
    <entry producer="prod_cam_a" in="0" out="65"/>
    <entry producer="prod_broll" in="0" out="59"/>
  </playlist>
  <playlist id="playlist_v2">
    <blank length="30"/>
    <entry producer="prod_overlay" in="0" out="59"/>
  </playlist>
  <tractor id="tractor0" title="Master Timeline">
    <track producer="playlist_v1"/>
    <track producer="playlist_v2"/>
    <transition id="trans_dissolve">
      <property name="a_track">0</property>
      <property name="b_track">1</property>
      <property name="in">54</property>
      <property name="out">72</property>
      <property name="mlt_service">luma</property>
    </transition>
  </tractor>
</mlt>`;

document.addEventListener("DOMContentLoaded", () => {
  initTimeline();
  initXmlInspector();
});

function initTimeline() {
  const slider = document.getElementById("timeline-slider");
  const playBtn = document.getElementById("play-btn");
  const stepBackBtn = document.getElementById("step-back-btn");
  const stepFwdBtn = document.getElementById("step-fwd-btn");
  const timecodeDisp = document.getElementById("timecode-display");
  const monitorContent = document.getElementById("monitor-content");
  const hudFrame = document.getElementById("hud-frame");
  const playheadLine = document.getElementById("playhead-line");

  let isPlaying = false;
  let playInterval = null;

  function renderFrame(frameIdx) {
    const totalFrames = 120;
    const pct = frameIdx / totalFrames;

    // Timecode calculation
    const fps = 30;
    const totalSecs = Math.floor(frameIdx / fps);
    const frames = frameIdx % fps;
    const mins = Math.floor(totalSecs / 60);
    const secs = totalSecs % 60;
    const tc = `00:${String(mins).padStart(2, '0')}:${String(secs).padStart(2, '0')}:${String(frames).padStart(2, '0')}`;
    timecodeDisp.textContent = tc;
    hudFrame.textContent = `Frame: ${frameIdx} / ${totalFrames}`;

    // Playhead line position
    playheadLine.style.left = `calc(140px + (100% - 140px) * ${pct})`;

    // Simulated multi-track composite
    // 0..54: Cam A
    // 54..72: Dissolve between Cam A & B-Roll
    // 72..120: B-Roll
    // Overlay is active from frame 30..90
    const hasOverlay = frameIdx >= 30 && frameIdx <= 90;

    let baseBg = "#1e3a8a"; // Cam A (Blue)
    let label = "Interview_Camera_A.mp4";

    if (frameIdx >= 72) {
      baseBg = "#b45309"; // B-Roll (Amber)
      label = "B-Roll_Landscape.mp4";
    } else if (frameIdx >= 54) {
      // Transition range
      const t = (frameIdx - 54) / 18;
      baseBg = `color-mix(in srgb, #b45309 ${Math.round(t * 100)}%, #1e3a8a)`;
      label = `Cross Dissolve (${Math.round(t * 100)}%)`;
    }

    monitorContent.style.background = baseBg;
    monitorContent.innerHTML = `
      <div style="text-align: center; color: #fff;">
        <div>${label}</div>
        ${hasOverlay ? '<div style="margin-top: 10px; font-size: 0.9rem; background: rgba(6, 182, 212, 0.4); padding: 4px 12px; border-radius: 4px; border: 1px solid #06b6d4;">[V2 Title Overlay]</div>' : ''}
      </div>
    `;
  }

  slider.addEventListener("input", e => {
    renderFrame(parseInt(e.target.value));
  });

  playBtn.addEventListener("click", () => {
    if (isPlaying) {
      clearInterval(playInterval);
      isPlaying = false;
      playBtn.textContent = "▶ Play";
    } else {
      isPlaying = true;
      playBtn.textContent = "⏸ Pause";
      playInterval = setInterval(() => {
        let curr = parseInt(slider.value) + 1;
        if (curr >= 120) curr = 0;
        slider.value = curr;
        renderFrame(curr);
      }, 33);
    }
  });

  stepBackBtn.addEventListener("click", () => {
    let curr = Math.max(0, parseInt(slider.value) - 1);
    slider.value = curr;
    renderFrame(curr);
  });

  stepFwdBtn.addEventListener("click", () => {
    let curr = Math.min(119, parseInt(slider.value) + 1);
    slider.value = curr;
    renderFrame(curr);
  });

  renderFrame(45);
}

function initXmlInspector() {
  const xmlDisplay = document.getElementById("xml-display");
  const copyBtn = document.getElementById("copy-xml-btn");

  xmlDisplay.textContent = XML_PROJECT;

  copyBtn.addEventListener("click", () => {
    navigator.clipboard.writeText(XML_PROJECT);
    copyBtn.textContent = "✅ Copied!";
    setTimeout(() => copyBtn.textContent = "📋 Copy XML", 2000);
  });
}
