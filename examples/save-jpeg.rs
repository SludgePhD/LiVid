//! Captures a video frame and writes it to a file.
//!
//! Uses the [`linuxvideo::stream::ReadStream`] returned by [`linuxvideo::VideoCaptureDevice::into_stream`]
//! to read image data.

use std::{env, fs::File, io::Write, path::Path};

use linuxvideo::{
    format::{PixFormat, Pixelformat},
    Device,
};

fn main() -> linuxvideo::Result<()> {
    env_logger::init();

    let mut args = env::args_os().skip(1);

    let (device, file_path) = match (args.next(), args.next()) {
        (Some(device), Some(file_path)) => (device, file_path),
        _ => {
            println!("usage: save-stream <device> <file>");
            std::process::exit(1);
        }
    };

    let mut file = File::create(file_path)?;

    let device = Device::open(Path::new(&device))?;

    println!(
        "capabilities: {:?}",
        device.capabilities()?.device_capabilities()
    );

    let capture = device.video_capture(PixFormat::new(u32::MAX, u32::MAX, Pixelformat::JPEG))?;
    println!("negotiated format: {:?}", capture.format());

    let mut stream = capture.into_stream(2)?;

    println!("stream started, waiting for data");
    stream.dequeue(|buf| {
        file.write_all(&*buf)?;
        println!("wrote file");
        Ok(())
    })?;

    Ok(())
}
