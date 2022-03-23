use clap::Parser;

/// Software for communicating with viewers on streaming broadcasts
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
    /// Path to the folder with voice recognition models
    #[clap(short, long, default_value = "models")]
    pub model_dir: String,

     /// The length of the pause between the text to start recognizing the text in seconds
     #[clap(short, long, default_value = "1.0")]
     pub pause_length: f32,

     /// The length of the pause between the text to start recognizing the text in seconds
     #[clap(short, long, default_value = "200")]
     pub silence_level: i32,

    /// Show debug prints
    #[clap(short, long, parse(try_from_str), default_value = "true")]
    pub debug_mode: bool,

}

pub fn get_args() -> Cli {
    Cli::parse()
}