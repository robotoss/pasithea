use coqui_stt::Model;
use std::path::Path;

pub fn initial_spech_recognition(model_dir: String, audio: Vec<i16>) {
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
    let mut m = Model::new(_model_name.to_str().expect("invalid utf-8 found in path")).unwrap();

    if let Some(scorer) = _scorer_name {
        let scorer = scorer.to_str().expect("invalid utf-8 found in path");
        println!("Using external scorer `{}`", scorer);
        m.enable_external_scorer(scorer).unwrap();
    }

    let result = m.speech_to_text(&audio).unwrap();

    println!("My result: {}", result);
}
