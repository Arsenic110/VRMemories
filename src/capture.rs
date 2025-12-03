use std::time::Instant;
use windows_capture::{
    capture::{Context, GraphicsCaptureApiHandler},
    encoder::{ImageEncoder, VideoEncoder}, frame::ImageFormat, settings::ColorFormat
};

pub struct Capture {
}

impl GraphicsCaptureApiHandler for Capture {
    type Flags = String;
    type Error = Box<dyn std::error::Error + Send + Sync>;

    fn new(_: Context<Self::Flags>) -> Result<Self, Self::Error> {
        Ok(Self { })
    }

    fn on_frame_arrived(
            &mut self,
            frame: &mut windows_capture::frame::Frame,
            _capture_control: windows_capture::graphics_capture_api::InternalCaptureControl,
        ) -> Result<(), Self::Error> {
            
        let dirs = directories::UserDirs::new().unwrap();

        let filename = format!("{}/VRMemories/{}.png",
            dirs.picture_dir().unwrap().display(),
            chrono::Local::now().format("%Y-%m-%d_%H-%M-%S-%f")
        );
        
        frame.buffer_without_title_bar()?.save_as_image(filename, ImageFormat::Png)?;

        Ok(())
    }
}