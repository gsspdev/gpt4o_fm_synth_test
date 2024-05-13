use clap::Parser;
use hound;

#[derive(Parser, Debug)]
#[command(name = "FM Synth")]
#[command(author = "Your Name")]
#[command(version = "1.0")]
#[command(about = "A basic FM synthesizer")]
struct Args {
    /// Carrier frequency in Hz
    #[arg(short, long, default_value = "440")]
    carrier_freq: f32,

    /// Modulator frequency in Hz
    #[arg(short, long, default_value = "220")]
    modulator_freq: f32,

    /// Modulation index
    #[arg(short, long, default_value = "1.0")]
    index: f32,

    /// Duration in seconds
    #[arg(long, default_value = "5")]
    duration: u32,

    /// Output file name
    #[arg(short, long, default_value = "output.wav")]
    output: String,
}

fn main() {
    let args = Args::parse();

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
        let modulator_signal = (2.0 * std::f32::consts::PI * args.modulator_freq * time).sin();
        let carrier_signal = (2.0 * std::f32::consts::PI * args.carrier_freq * time
            + args.index * modulator_signal)
            .sin();
        let sample = (carrier_signal * amplitude) as i16;
        writer.write_sample(sample).unwrap();
    }

    writer.finalize().unwrap();
    println!("Wav file {} generated successfully!", args.output);
}
