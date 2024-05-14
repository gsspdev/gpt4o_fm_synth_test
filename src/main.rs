use clap::Parser;
use hound;

#[derive(Parser, Debug)]
#[command(name = "FM Synth")]
#[command(author = "Your Name")]
#[command(version = "1.0")]
#[command(about = "A basic FM synthesizer")]
struct Args {
    #[arg(short, long, default_value = "440")]
    carrier: f32,

    #[arg(short, long, default_value = "220")]
    modulator: f32,

    #[arg(short, long, default_value = "1.0")]
    index: f32,

    #[arg(short, long, default_value = "256.0")]
    end: f32,

    #[arg(long, default_value = "5")]
    duration: u32,

    #[arg(short, long, default_value = "output.wav")]
    output: String,
} // carrier, modulator, index, end. duration, output

fn main() {
    let args = Args::parse();

    #[derive(Debug)]
    struct KeyFrames {
        x: f32,
        y: f32
    }
    #[derive(Debug)]

    struct Frame {
        key_frames: KeyFrames,
    }

    impl KeyFrames {
        pub fn new(x: f32, y: f32) -> Self {
            KeyFrames { x, y, number: 0 }
        }
    }

    enum KeyFrames {
        KeyFrame { x: f32, y: f32 },
    }


    let wavet1 = KeyFrame::new(0.0, 5.0,);
    let x_step = duration / 256.0;
    let y_step = (wavet1.end - wavet1.start) / 256.0;
    let frames: Vec<{ AutomationFrame { auto_frame: auto_frame { x: f32, y: f32 } } }> = Vec::new();

    println!("{}", wavet1.start);

    const SAMPLE_RATE: u32 = 44100;
    let num_samples = SAMPLE_RATE * args.duration;
    let amplitude = i16::MAX as f32;

    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: SAMPLE_RATE,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };

    let mut writer = hound::WavWriter::create(&args.output, spec).unwrap();

    for t in 0..num_samples {
        let time = t as f32 / SAMPLE_RATE as f32;
        let modulator = (2.0 * std::f32::consts::PI * args.modulator * time).sin();
        let carrier = (2.0 * std::f32::consts::PI * args.carrier * time
            + args.index * modulator)
            .sin();
        let sample = (carrier * amplitude) as i16;
        writer.write_sample(sample).unwrap();
    }

    writer.finalize().unwrap();
    println!("Wav file {} generated successfully!", args.output);
}
