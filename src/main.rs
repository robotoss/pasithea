mod audio_host;
mod help;
use crate::audio_host::audio_stream;

mod spech_recognition;

fn main() {
    loop {
        let args = help::get_args();
        let _model_dir_str = args.model_dir;

        let _audio =
            audio_stream::record_audio(args.silence_level, args.pause_length, args.debug_mode);

        match _audio {
            Ok(audio) => spech_recognition::initial_spech_recognition(_model_dir_str, audio),
            Err(err) => println!("Some Error Text {}", err),
        }
    }
}
