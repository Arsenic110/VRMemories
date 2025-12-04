use std::time::Duration;
use std::thread;

use windows_capture:: {
    capture::GraphicsCaptureApiHandler, settings::*, window::Window
};

mod example;
mod capture;

use capture::Capture;

fn main() {
    let target_title = "VRChat";
    let capture_interval = Duration::from_secs(30);

    let dirs = directories::UserDirs::new().unwrap();
    //let savepath = format!("{}/VRMemories", dirs.picture_dir().unwrap().display());
    let savepath = "E:/AllStuff/Pictures/VRMemories".to_string();

    println!("Photos will be saved in: {}", savepath);

    loop {
        //attempt to find window
        let window = match Window::from_name(target_title) {
            Ok(window) => window,
            Err(_) => {
                println!("Window not found. Next attempt in {}s", capture_interval.as_secs());
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
            savepath.clone()
        );

        //start capturing
        Capture::start(settings).expect("Capture Failed");
        //once capture fails, returns to this thread.
        //the loop will ensure more attempts to find the window and try again
    }
}
