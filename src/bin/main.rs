use clap::Parser;
use indicatif::{ProgressBar, ProgressStyle};
use std::path::PathBuf;
use mlt_rs::consumer::RenderConsumer;
use mlt_rs::playlist::Playlist;
use mlt_rs::producer::{ClipProducer, ColorProducer};
use mlt_rs::profile::Profile;
use mlt_rs::tractor::Tractor;
use mlt_rs::xml::MltDocument;

#[derive(Parser, Debug)]
#[command(
    name = "melt",
    about = "Media Lovin' Toolkit (MLT) command-line playout and render engine in pure Rust",
    version
)]
struct Args {
    /// Input files, media clips, or .mlt XML project files
    #[arg(required = true)]
    inputs: Vec<String>,

    /// Render output destination file (e.g. output.mp4, render.mlt)
    #[arg(short = 'c', long = "consumer")]
    consumer: Option<PathBuf>,

    /// Video profile (hd1080p30, hd1080p60, uhd2160p60, vertical)
    #[arg(short = 'p', long = "profile", default_value = "hd1080p30")]
    profile: String,

    /// In point (starting frame)
    #[arg(long = "in", default_value_t = 0)]
    in_point: i64,

    /// Out point (ending frame)
    #[arg(long = "out")]
    out_point: Option<i64>,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let profile = Profile::from_name(&args.profile).unwrap_or_else(|| {
        eprintln!("Warning: Profile '{}' not recognized, defaulting to 1080p30", args.profile);
        Profile::hd_1080_30p()
    });

    println!("melt: Loaded Profile '{}' ({}x{} @ {:.2} fps)",
        profile.description, profile.width, profile.height, profile.fps()
    );

    let mut tractor = Tractor::new("main_tractor");
    let mut track0 = Playlist::new("track0");

    for input in &args.inputs {
        if input.ends_with(".mlt") || input.ends_with(".xml") {
            println!("melt: Parsing MLT XML document: {}", input);
            let xml_content = std::fs::read_to_string(input)?;
            let doc = MltDocument::from_xml(&xml_content)?;
            println!("melt: Loaded project with {} producers and {} tracks",
                doc.producers.len(), doc.tractors.len()
            );
            for p in &doc.producers {
                track0.append(
                    Box::new(ClipProducer::new(p.id.clone(), p.id.clone(), p.in_point, p.out_point)),
                    p.in_point,
                    p.out_point,
                );
            }
        } else if input.starts_with("color:") {
            let length = args.out_point.unwrap_or(90) - args.in_point;
            track0.append(
                Box::new(ColorProducer::new("color", 20, 30, 40, 255, length)),
                0,
                length - 1,
            );
        } else {
            let out_p = args.out_point.unwrap_or(90);
            track0.append(
                Box::new(ClipProducer::new(input.clone(), input.clone(), args.in_point, out_p)),
                args.in_point,
                out_p,
            );
        }
    }

    tractor.add_track(track0);
    let total = tractor.total_frames();
    println!("melt: Timeline duration: {} frames ({:.2}s)", total, total as f64 / profile.fps());

    if let Some(dest) = args.consumer {
        println!("melt: Rendering timeline to: {}", dest.display());

        let pb = ProgressBar::new(100);
        pb.set_style(
            ProgressStyle::default_bar()
                .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}% ({msg})")?
                .progress_chars("#>-"),
        );

        RenderConsumer::render_to_file(&mut tractor, &profile, &dest, |prog| {
            pb.set_position(prog.percent as u64);
            pb.set_message(format!("{:.1} fps | ETA: {}s | frame {}/{}",
                prog.fps, prog.eta_seconds, prog.current_frame, prog.total_frames
            ));
        })?;

        pb.finish_with_message("Render complete!");
        println!("\nmelt: Successfully exported: {}", dest.display());
    } else {
        println!("melt: Previewing first 5 frames:");
        for f in 0..5.min(total) {
            let frame = tractor.get_frame(f, &profile)?;
            println!("  • Frame {:04}: size {}x{}, {} image bytes, {} audio samples",
                f, frame.width, frame.height, frame.image_data.len(), frame.audio_samples.len()
            );
        }
        println!("melt: Playout simulation finished.");
    }

    Ok(())
}
