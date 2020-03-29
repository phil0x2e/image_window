extern crate image_window;
use image_window::{ImageWindow, Key, ScaleMode, WindowOptions};

fn main() {
    let mut window = ImageWindow::new(
        "Test",
        800,
        600,
        WindowOptions {
            resize: true,
            scale_mode: ScaleMode::Center,
            ..WindowOptions::default()
        },
    )
    .unwrap();

    // Limit to max ~60 fps update rate
    //window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    window.set_image_from_path("uv.png");

    while window.is_open() && !window.is_key_down(Key::Escape) {
        window.fit_to_screen();
        window.update();
    }
}
