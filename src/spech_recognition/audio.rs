use core::time;
use std::{sync::mpsc, thread};

use anyhow::{anyhow, Result};
use cpal::{
    traits::{DeviceTrait, HostTrait, StreamTrait},
    Device, SampleRate, SupportedStreamConfig,
};
use dasp_interpolate::linear::Linear;
use dasp_signal::{from_iter, interpolate::Converter, Signal};

pub struct Audio {
    device: Device,
    config: SupportedStreamConfig,
}

impl Audio {
    pub fn init() -> Result<Audio> {
        let host = cpal::default_host();
        let device = host
            .default_input_device()
            .expect("Can't get default input device");
        match get_config(&device) {
            Ok(config) => Ok(Audio { device, config }),
            Err(e) => Err(anyhow!("Error on get Audio Stream Config {}", e)),
        }
    }

    pub fn open_input_stream(self, sn_spoken_text: mpsc::Sender<Vec<i16>>) {
        let stream = self
            .device
            .build_input_stream(
                &self.config.config().into(),
                move |data: &[f32], _: &cpal::InputCallbackInfo| {
                    let buffer = data.to_vec();

                    let audio_buf = buffer.into_iter().map(|a| (a * i16::MAX as f32) as i16);

                    let src_sample_rate = self.config.sample_rate().0;
                    let dest_sample_rate = 16000 as u32;

                    // if src_sample_rate != dest_sample_rate {
                    let interpolator = Linear::new([0i16], [0]);
                    // We need to interpolate to the target sample rate

                    let conv = Converter::from_hz_to_hz(
                        from_iter(audio_buf.map(|s| [s])),
                        interpolator,
                        src_sample_rate as f64,
                        dest_sample_rate as f64,
                    );
                    let buff: Vec<_> = conv.until_exhausted().map(|v| v[0]).collect();

                    sn_spoken_text.send(buff).unwrap();
                    // } else {
                    //     Ok(audio_buf.collect::<Vec<_>>())
                    // }
                },
                move |err| println!("Audio stream read error: {}", err),
            )
            .unwrap();
        stream.play().unwrap();
        loop {
            thread::sleep(time::Duration::from_millis(10));
        }
    }
}

fn get_config(device: &Device) -> Result<SupportedStreamConfig> {
    let mut config = device.default_input_config()?;
    if config.channels() != 1 {
        let mut supported_configs_range = device.supported_input_configs()?;
        config = match supported_configs_range.next() {
            Some(conf) => {
                conf.with_sample_rate(SampleRate(16000)) //16K from deepspeech
            }
            None => config,
        };
    }
    Ok(config)
}
