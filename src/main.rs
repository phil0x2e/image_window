extern crate image_window;
use image_window::{FilterType, ImageWindow, Key, MouseMode, Scale, ScaleMode, WindowOptions};

fn main() {
    let mut window = ImageWindow::new(
        "image_window example",
        800,
        600,
        WindowOptions {
            resize: true,
            scale_mode: ScaleMode::Center,
            ..WindowOptions::default()
        },
        None,
    )
    .unwrap();

    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    window.set_image_from_path_fit("uv.png").unwrap();

    while window.is_open() && !window.is_key_down(Key::Escape) {
        if window.is_key_down(Key::Z) {
            window.zoom((200, 200), 0.5);
        }
        window.get_scroll_wheel().map(|scroll| {
            let scrl = scroll.1;
            window.get_mouse_pos(MouseMode::Clamp).map(|mouse| {
                println!("x {} y {}", mouse.0, mouse.1);
            });
        });
        window.fit_to_screen();
        window.update();
    }
}
