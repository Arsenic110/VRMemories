use std::time::Duration;
use std::thread;

use windows_capture:: {
    capture::GraphicsCaptureApiHandler, settings::*, window::{self, Window}
};

mod example;
mod capture;

fn main() {
    let target_title = "VRChat";
    let capture_interval = Duration::from_secs(60);

    loop {
        //attempt to find window
        let window = match Window::from_name(target_title) {
            Ok(window) => window,
            Err(_) => {
                println!("Window not found. Next attempt in {}", capture_interval.as_secs());
                thread::sleep(capture_interval);
                continue;
            }
        };

        let settings = Settings::new(
            window,
            CursorCaptureSettings::WithoutCursor,
            DrawBorderSettings::WithoutBorder,
            SecondaryWindowSettings::Exclude,
            MinimumUpdateIntervalSettings::Custom(capture_interval),
            DirtyRegionSettings::Default,
            ColorFormat::Rgba8,
            "".into()
        );

        capture::Capture::start(settings).expect("Capture Failed");
    }
}
