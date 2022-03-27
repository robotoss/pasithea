use super::audio::Audio;
use coqui_stt::{Model, Stream};
use core::time;
use std::{
    path::Path,
    sync::{
        self,
        atomic::{AtomicBool, Ordering},
        mpsc::{channel, Receiver, Sender},
    },
};

enum SpeechState {
    Listening,
    Ready,
    Complete,
}

pub struct SpechRecognition {
    // model: Model,
    stream: Stream,
    state: SpeechState,
}

impl SpechRecognition {
    pub fn init(model_dir: String) -> SpechRecognition {
        let dir_path = Path::new(&model_dir);
        let mut _model_name: Box<Path> = dir_path.join("output_graph.pb").into_boxed_path();
        let mut _scorer_name: Option<Box<Path>> = None;
        // search for model in model directory

        for file in dir_path
            .read_dir()
            .expect("Specified model dir is not a dir")
        {
            if let Ok(f) = file {
                let file_path = f.path();
                if file_path.is_file() {
                    if let Some(ext) = file_path.extension() {
                        if ext == "pb" || ext == "pbmm" || ext == "tflite" {
                            _model_name = file_path.into_boxed_path();
                        } else if ext == "scorer" {
                            _scorer_name = Some(file_path.into_boxed_path());
                        }
                    }
                }
            }
        }
        let mut model =
            Model::new(_model_name.to_str().expect("invalid utf-8 found in path")).unwrap();

        if let Some(scorer) = _scorer_name {
            let scorer = scorer.to_str().expect("invalid utf-8 found in path");
            println!("Using external scorer `{}`", scorer);
            model.enable_external_scorer(scorer).unwrap();
        }
        let state = SpeechState::Listening;

        let stream = model
            .into_streaming()
            .expect("Can't change model into Stream");

        SpechRecognition {
            // model,
            stream,
            state,
        }
    }

    pub fn run_spech_recognition(&mut self, sn_spoken_text: Sender<String>) {
        let audio = Audio::init().expect("msg");

        let (tx, rc) = channel();
        let tx1 = Sender::clone(&tx);

        std::thread::spawn(move || {
            audio.open_input_stream(tx1);
        });

        self.start_stream(String::from("hi"), sn_spoken_text, rc);
    }

    fn start_stream(
        &mut self,
        wake_word: String,
        sn_spoken_text: Sender<String>,
        buffer_rc: Receiver<Vec<i16>>,
    ) {
        println!("Starting speech stream...");

        let mut prev_text = String::from("");
        let mut timer = Timer::new();
        let mut rc_complete: Option<Receiver<bool>> = None;

        loop {
            match rc_complete {
                Some(ref r) => {
                    let complete = r.recv().unwrap();
                    if complete == true {
                        if timer.alive.load(Ordering::SeqCst) {
                            timer.stop();
                        }
                        self.state = SpeechState::Complete;
                    }
                }
                None => (),
            }

            let buffer = buffer_rc.recv().unwrap();
            let buffer_slice: &[i16] = buffer.as_ref();
            self.stream.feed_audio(buffer_slice);

            let decoded = self.stream.intermediate_decode();

            match decoded {
                Ok(text) => match self.state {
                    SpeechState::Listening => {
                        println!("Text {}", text);
                        if text.contains(&wake_word) {
                            self.state = SpeechState::Ready;
                        }
                    }
                    SpeechState::Ready => {
                        if text != prev_text.to_string() {
                            prev_text = text;
                            if timer.alive.load(Ordering::SeqCst) {
                                timer.stop();
                            }
                            rc_complete = Some(timer.start());
                        }
                    }
                    SpeechState::Complete => {
                        // let text = self.stream.finish_stream().unwrap();
                        sn_spoken_text.send(String::from("text")).unwrap();
                        break;
                    }
                },
                Err(err) => eprintln!("{}", err),
            }
        }

        println!("Stopping speech stream...");
        self.state = SpeechState::Listening;
        self.start_stream(wake_word, sn_spoken_text, buffer_rc);
    }
}

struct Timer {
    handle: Option<std::thread::JoinHandle<()>>,
    alive: sync::Arc<AtomicBool>,
}

impl Timer {
    fn new() -> Timer {
        Timer {
            handle: None,
            alive: sync::Arc::new(AtomicBool::new(false)),
        }
    }

    fn start(&mut self) -> Receiver<bool> {
        println!("Starting timer...");
        self.alive.store(true, Ordering::SeqCst);
        let alive = self.alive.clone();

        let (s, r) = channel();

        self.handle = Some(std::thread::spawn(move || {
            while alive.load(Ordering::SeqCst) {
                std::thread::sleep(time::Duration::from_secs(2));
                s.send(true).unwrap();
            }
        }));

        r
    }

    fn stop(&mut self) {
        println!("Stopping timer...");
        self.alive.store(false, Ordering::SeqCst);
        self.handle
            .take()
            .expect("Called stop on non-running thread")
            .join()
            .expect("Could not joing spawned thread");
    }
}
