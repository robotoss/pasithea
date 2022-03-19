use anyhow::{anyhow, Result};
use cpal::{Device, SupportedStreamConfig, SampleRate};
use cpal::traits::{HostTrait};
use cpal::traits::{DeviceTrait};

const SAMPLE_RATE: u32 = 16000;

///
/// # Input device configuration
/// Gets data ready to begin recording

pub(crate) struct StreamConfig {
    device: Device,
    config: SupportedStreamConfig,
    silence_level: i32,
}

impl StreamConfig {
    pub fn init_autdio_settings(silence_level: i32) -> Result<Self>{
        let device = get_default_device()?;
        match get_config(&device) {
            Ok(config) => Ok(StreamConfig {
                device,
                config,
                silence_level,
            }),
            Err(e) => {
                println!("{}", e);
                Err(anyhow!(e))
            }
        }
    }

    /// Returns the device in use:
    pub fn device(&self) -> &Device {
        &self.device
    }

    /// Returns SupportedStreamConfig (.config() returns the configuration we use on our stream)
    pub fn supported_config(&self) -> &SupportedStreamConfig {
        &self.config
    }

    /// Returns the silence level
    pub fn silence_level(&self) -> i32 {
        self.silence_level
    }
}

fn get_config(device: &Device) -> Result<SupportedStreamConfig> {
    let mut config = device.default_input_config()?;
    if config.channels() != 1 {
        let mut supported_configs_range = device.supported_input_configs()?;
        config = match supported_configs_range.next() {
            Some(conf) => {
                conf.with_sample_rate(SampleRate(SAMPLE_RATE)) //16K from deepspeech
            }
            None => config,
        };
    }
    Ok(config)
}

fn get_default_device() -> Result<Device> {
    let host = cpal::default_host();
    match host.default_input_device() {
        Some(device) => Ok(device),
        None => Err(anyhow!("no input device found")),
    }
}