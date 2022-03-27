// mod audio_host;
mod help;
// use crate::audio_host::audio_stream;

// mod spech_recognition;

mod spech_recognition;
use spech_recognition::recognition::SpechRecognition;

use core::time;
use std::{
    sync::mpsc::{channel, Sender},
    thread,
};

fn main() {
    let args = help::get_args();

    let (sn_spoken_text, rc_spoken_text) = channel::<String>();

    thread::spawn(move || {
        let mut _recognition = SpechRecognition::init(args.model_dir);
        _recognition.run_spech_recognition(Sender::clone(&sn_spoken_text));
    });

    loop {
        let text = rc_spoken_text.recv().unwrap();
        println!("Final text: {:?}", text);
        thread::sleep(time::Duration::from_millis(10));
    }
}
