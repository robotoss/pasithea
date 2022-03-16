use clap::Parser;

/// Software for communicating with viewers on streaming broadcasts
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
    /// Path to the folder with voice recognition models
    #[clap(short, long, default_value = "models")]
    pub model_dir: String,

    /// Path to the audio file
    #[clap(short, long, default_value = "audio/2830-3980-0043.wav")]
    pub audio_file_path: String,


}