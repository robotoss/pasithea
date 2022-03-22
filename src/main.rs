mod help;
mod audio_host;
use crate::audio_host::audio_stream;

mod spech_recognition;

fn main() {    
    let args = help::get_args();
    let _model_dir_str =  args.model_dir;
    let _audio_file_path = args.audio_file_path;

    let x = audio_stream::record_audio(args.silence_level, args.pause_length, args.debug_mode);
    println!("{:?}", x);

    // spech_recognition::initial_spech_recognition(_model_dir_str, _audio_file_path);
}
