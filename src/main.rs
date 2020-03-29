extern crate image_window;
use image_window::{FilterType, ImageWindow, Key, ScaleMode, WindowOptions};

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
        None
    )
    .unwrap();

    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));
    let i1 = image::open("uv.png").unwrap();
    let i2 = image::open("uv2.png").unwrap();

    window.set_image_from_path("uv.png").unwrap();

    while window.is_open() && !window.is_key_down(Key::Escape) {
        if window.is_key_released(Key::Key1) {
            window.set_from_image_fit(&i1);
        }
        if window.is_key_released(Key::Key2) {
            window.set_from_image_fit(&i2);
        }
        window.fit_to_screen();
        window.update();
    }
}
