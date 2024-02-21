use crate::app::App;
use cpal::{
    traits::{DeviceTrait, HostTrait},
    SizedSample,
};
use cpal::{FromSample, Sample};
use std::sync::{Arc, Mutex};

pub fn stream_setup_for(app: Arc<Mutex<App>>) -> Result<cpal::Stream, anyhow::Error> {
    let (_host, device, config) = host_device_setup()?;

    match config.sample_format() {
        cpal::SampleFormat::I8 => make_stream::<i8>(&device, &config.into(), app),
        cpal::SampleFormat::I16 => make_stream::<i16>(&device, &config.into(), app),
        cpal::SampleFormat::I32 => make_stream::<i32>(&device, &config.into(), app),
        cpal::SampleFormat::I64 => make_stream::<i64>(&device, &config.into(), app),
        cpal::SampleFormat::U8 => make_stream::<u8>(&device, &config.into(), app),
        cpal::SampleFormat::U16 => make_stream::<u16>(&device, &config.into(), app),
        cpal::SampleFormat::U32 => make_stream::<u32>(&device, &config.into(), app),
        cpal::SampleFormat::U64 => make_stream::<u64>(&device, &config.into(), app),
        cpal::SampleFormat::F32 => make_stream::<f32>(&device, &config.into(), app),
        cpal::SampleFormat::F64 => make_stream::<f64>(&device, &config.into(), app),
        sample_format => Err(anyhow::Error::msg(format!(
            "Unsupported sample format '{sample_format}'"
        ))),
    }
}

pub fn host_device_setup(
) -> Result<(cpal::Host, cpal::Device, cpal::SupportedStreamConfig), anyhow::Error> {
    let host = cpal::default_host();

    let device = host
        .default_output_device()
        .ok_or_else(|| anyhow::Error::msg("Default output device is not available"))?;

    let config = device.default_output_config()?;

    Ok((host, device, config))
}

pub fn make_stream<T>(
    device: &cpal::Device,
    config: &cpal::StreamConfig,
    app: Arc<Mutex<App>>,
) -> Result<cpal::Stream, anyhow::Error>
where
    T: SizedSample + FromSample<f32>,
{
    let num_channels = config.channels as usize;
    let err_fn = |err| eprintln!("Error building output sound stream: {}", err);

    app.lock()
        .unwrap()
        .set_sample_rates(config.sample_rate.0 as f32);

    let stream = device.build_output_stream(
        config,
        move |output: &mut [T], _: &cpal::OutputCallbackInfo| {
            process_frame(output, app.clone(), num_channels)
        },
        err_fn,
        None,
    )?;

    Ok(stream)
}

fn process_frame<SampleType>(output: &mut [SampleType], app: Arc<Mutex<App>>, num_channels: usize)
where
    SampleType: Sample + FromSample<f32>,
{
    for frame in output.chunks_mut(num_channels) {
        let (l,r) = app.lock().unwrap().tick_all();
        let left: SampleType = SampleType::from_sample(l);
        let right: SampleType = SampleType::from_sample(r);

        for (index, sample) in frame.iter_mut().enumerate() {
            match index {
                0 => *sample = left,
                1 => *sample = right,
                _ => {}
            }
        }
    }
}
