use std::time::Instant;
use windows_capture::{
    capture::{Context, GraphicsCaptureApiHandler},
    encoder::{ImageEncoder, VideoEncoder}, frame::ImageFormat, settings::ColorFormat,
    graphics_capture_api::InternalCaptureControl,
    frame::Frame
};

pub struct Capture {
    savepath: String
}

impl GraphicsCaptureApiHandler for Capture {
    type Flags = String;
    type Error = Box<dyn std::error::Error + Send + Sync>;

    fn new(ctx: Context<Self::Flags>) -> Result<Self, Self::Error> {
        Ok(Self { savepath: ctx.flags })
    }

    fn on_frame_arrived(&mut self, frame: &mut Frame, _capture_control: InternalCaptureControl) -> Result<(), Self::Error> {
        let now = chrono::Local::now();

        let savefolder = format!("{}/{}", 
            self.savepath,
            now.format("%Y-%m")
        );

        let savefolder = savefolder.as_str();

        match std::fs::create_dir_all(savefolder) {
            Ok(_) => println!("Save directory didn't exist. It now does!"),
            Err(_) => {}
        }

        let filename = format!("{}/{}.png", 
            savefolder,
            now.format("%Y-%m-%d_%H-%M-%S-%f")
        );

        println!("Saved {}", filename);
        frame.buffer_without_title_bar()?.save_as_image(filename, ImageFormat::Png)?;

        Ok(())
    }
}