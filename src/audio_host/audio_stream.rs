use std::{
    ops::Neg,
    sync::mpsc::{channel, Receiver},
    time::Instant,
};

use super::audio_settings::AppStreamConfig;
use anyhow::Result;
use cpal::traits::{DeviceTrait, StreamTrait};
use dasp_interpolate::linear::Linear;
use dasp_signal::{from_iter, interpolate::Converter, Signal};
use nnnoiseless::DenoiseState;

pub fn record_audio(
    silence_level: i32,
    pause_length: f32,
    show_amplitude: bool,
) -> Result<Vec<i16>> {
    let config = AppStreamConfig::init_autdio_settings(silence_level)?;

    let device = config.device();
    let (sound_sender, sound_receiver) = channel();
    let stream_config = config.supported_config().config();

    let stream = device.build_input_stream(
        &stream_config,
        move |data: &[f32], _: &_| {
            if let Err(e) = sound_sender.send(data.to_owned()) {
                println!("{}", e);
            }
        },
        move |err| println!("Stream read error: {}", err),
    )?;

    match stream.play() {
        Ok(()) => {
            let denoised_stream = start(
                &sound_receiver,
                config.silence_level(),
                show_amplitude,
                pause_length,
            )?;
            let audio_buf = denoised_stream
                .into_iter()
                .map(|a| (a * i16::MAX as f32) as i16);

            // input audio must be mono and usually at 16KHz, but this depends on 16000

            let src_sample_rate = stream_config.sample_rate.0;
            let dest_sample_rate = 16000 as u32;

            if (src_sample_rate != dest_sample_rate) {
                let interpolator = Linear::new([0i16], [0]);
                // We need to interpolate to the target sample rate

                let conv = Converter::from_hz_to_hz(
                    from_iter(audio_buf.map(|s| [s])),
                    interpolator,
                    src_sample_rate as f64,
                    dest_sample_rate as f64,
                );

                Ok(conv.until_exhausted().map(|v| v[0]).collect())
            } else {
                Ok(audio_buf.collect::<Vec<_>>())
            }
        }
        Err(err) => {
            println!("Failed to start the stream: {}", err);
            Err(anyhow::anyhow!(err))
        }
    }
}

fn start(
    sound_receiver: &Receiver<Vec<f32>>,
    silence_level: i32,
    show_amplitude: bool,
    pause_length: f32,
) -> Result<Vec<f32>> {
    let mut silence_start = None;
    let mut sound_from_start_till_pause = vec![];

    loop {
        let small_sound_chunk = sound_receiver.recv()?;
        sound_from_start_till_pause.extend(&small_sound_chunk);
        let sound_as_ints = small_sound_chunk.iter().map(|f| (*f * 1000.0) as i32);
        let max_amplitude = sound_as_ints.clone().max().unwrap_or(0);
        let min_amplitude = sound_as_ints.clone().min().unwrap_or(0);
        if show_amplitude {
            println!("Min is {}, Max is {}", min_amplitude, max_amplitude);
        }
        let silence_detected = max_amplitude < silence_level && min_amplitude > silence_level.neg();
        if silence_detected {
            match silence_start {
                None => silence_start = Some(Instant::now()),
                Some(s) => {
                    if s.elapsed().as_secs_f32() > pause_length {
                        return Ok(denoise(sound_from_start_till_pause));
                    }
                }
            }
        } else {
            silence_start = None;
        }
    }
}

fn denoise(sound_from_start_till_pause: Vec<f32>) -> Vec<f32> {
    let mut output = Vec::new();
    let mut out_buf = [0.0; DenoiseState::FRAME_SIZE];
    let mut denoise = DenoiseState::new();
    let mut first = true;
    for chunk in sound_from_start_till_pause.chunks_exact(DenoiseState::FRAME_SIZE) {
        denoise.process_frame(&mut out_buf[..], chunk);
        if !first {
            output.extend_from_slice(&out_buf[..]);
        }
        first = false;
    }
    output
}
