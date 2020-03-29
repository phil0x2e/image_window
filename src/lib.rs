extern crate image;
extern crate minifb;
use core::ffi::c_void;
use core::time::Duration;
pub use image::imageops::FilterType;
use image::{DynamicImage, RgbImage};
pub use minifb::{
    CursorStyle, InputCallback, Key, KeyRepeat, Menu, MenuHandle, MouseButton, MouseMode,
    Result as MiniResult, ScaleMode, UnixMenu, Window, WindowOptions,
};
use std::io;

pub struct ImageWindow {
    window: Window,
    buffer: Vec<u32>,
    buffer_width: usize,
    buffer_height: usize,
    raw_image: Option<DynamicImage>,
    filter: FilterType,
}

impl ImageWindow {
    pub fn new(
        name: &str,
        width: usize,
        height: usize,
        opts: WindowOptions,
        filter: Option<FilterType>,
    ) -> MiniResult<Self> {
        let window = Window::new(name, width, height, opts);
        let filter = match filter {
            Some(f) => f,
            None => FilterType::Triangle,
        };

        match window {
            Err(e) => Err(e),
            Ok(w) => Ok(ImageWindow {
                window: w,
                buffer: Vec::new(),
                buffer_width: 0,
                buffer_height: 0,
                raw_image: None,
                filter,
            }),
        }
    }

    pub fn set_from_image(&mut self, img: &DynamicImage) {
        self.set_from_rgb_image(img.to_rgb());
    }

    pub fn set_from_image_fit(&mut self, img: &DynamicImage) {
        let size = self.window.get_size();
        let scaled = img.resize(size.0 as u32, size.1 as u32, self.filter);
        self.set_from_rgb_image(scaled.to_rgb());
    }

    fn set_from_rgb_image(&mut self, rgb_img: RgbImage) {
        let img_dim = rgb_img.dimensions();
        let mut buf = Vec::with_capacity((img_dim.0 * img_dim.1) as usize);
        for pixel in rgb_img.enumerate_pixels() {
            let r = pixel.2[0];
            let g = pixel.2[1];
            let b = pixel.2[2];
            let rgb = from_u8_rgb(r, g, b);
            buf.push(rgb);
        }
        self.buffer = buf;
        self.buffer_width = rgb_img.dimensions().0 as usize;
        self.buffer_height = rgb_img.dimensions().1 as usize;
    }

    pub fn set_image_from_path(&mut self, path: &str) -> Result<(), io::Error> {
        let img =
            match image::open(path) {
                Ok(i) => i,
                Err(_e) => return Err(io::Error::new(
                    std::io::ErrorKind::NotFound,
                    "File could not be opened. It may not exist or may not be of a supported type.",
                )),
            };
        self.set_from_image(&img);
        self.raw_image = Some(img);
        Ok(())
    }

    pub fn set_image_from_path_fit(&mut self, path: &str) -> Result<(), io::Error> {
        let img =
            match image::open(path) {
                Ok(i) => i,
                Err(_e) => return Err(io::Error::new(
                    std::io::ErrorKind::NotFound,
                    "File could not be opened. It may not exist or may not be of a supported type.",
                )),
            };
        self.set_from_image_fit(&img);
        self.raw_image = Some(img);
        Ok(())
    }

    pub fn fit_to_screen(&mut self) {
        if self.buffer_width != self.window.get_size().0
            && self.buffer_height != self.window.get_size().1
        {
            if let Some(img) = &self.raw_image {
                let size = self.window.get_size();
                let scaled = img.resize(size.0 as u32, size.1 as u32, self.filter);
                self.set_from_image(&scaled);
            }
        }
    }

    pub fn update(&mut self) {
        if self.buffer.len() > 0 && self.buffer_width > 0 && self.buffer_height > 0 {
            self.window
                .update_with_buffer(&self.buffer, self.buffer_width, self.buffer_height)
                .unwrap();
        } else {
            self.window.update();
        }
    }

    // Window methods
    pub fn add_menu(&mut self, menu: &Menu) -> MenuHandle {
        self.window.add_menu(menu)
    }
    pub fn get_keys(&self) -> Option<Vec<Key>> {
        self.window.get_keys()
    }
    pub fn get_keys_pressed(&self, repeat: KeyRepeat) -> Option<Vec<Key>> {
        self.window.get_keys_pressed(repeat)
    }
    pub fn get_mouse_down(&self, button: MouseButton) -> bool {
        self.window.get_mouse_down(button)
    }
    pub fn get_mouse_pos(&self, mode: MouseMode) -> Option<(f32, f32)> {
        self.window.get_mouse_pos(mode)
    }
    pub fn get_scroll_wheel(&self) -> Option<(f32, f32)> {
        self.window.get_scroll_wheel()
    }
    pub fn get_size(&self) -> (usize, usize) {
        self.window.get_size()
    }
    pub fn get_unix_menus(&self) -> Option<&Vec<UnixMenu>> {
        self.window.get_unix_menus()
    }
    pub fn get_unscaled_mouse_pos(&self, mode: MouseMode) -> Option<(f32, f32)> {
        self.window.get_unscaled_mouse_pos(mode)
    }
    pub fn get_window_handle(&self) -> *mut c_void {
        self.window.get_window_handle()
    }
    pub fn is_active(&mut self) -> bool {
        self.window.is_active()
    }
    pub fn is_key_down(&self, key: Key) -> bool {
        self.window.is_key_down(key)
    }
    pub fn is_key_pressed(&self, key: Key, repeat: KeyRepeat) -> bool {
        self.window.is_key_pressed(key, repeat)
    }
    pub fn is_key_released(&self, key: Key) -> bool {
        self.window.is_key_released(key)
    }
    pub fn is_menu_pressed(&mut self) -> Option<usize> {
        self.window.is_menu_pressed()
    }
    pub fn is_open(&self) -> bool {
        self.window.is_open()
    }
    pub fn limit_update_rate(&mut self, time: Option<Duration>) {
        self.window.limit_update_rate(time)
    }
    pub fn remove_menu(&mut self, handle: MenuHandle) {
        self.window.remove_menu(handle);
    }
    pub fn set_background_color(&mut self, red: usize, green: usize, blue: usize) {
        self.window.set_background_color(red, green, blue);
    }
    pub fn set_cursor_style(&mut self, cursor: CursorStyle) {
        self.window.set_cursor_style(cursor);
    }
    pub fn set_input_callback(&mut self, callback: Box<dyn InputCallback>) {
        self.window.set_input_callback(callback);
    }
    pub fn set_key_repeat_delay(&mut self, delay: f32) {
        self.window.set_key_repeat_delay(delay);
    }
    pub fn set_key_repeat_rate(&mut self, rate: f32) {
        self.window.set_key_repeat_rate(rate);
    }
    pub fn set_position(&mut self, x: isize, y: isize) {
        self.window.set_position(x, y);
    }
    pub fn set_title(&mut self, title: &str) {
        self.window.set_title(title);
    }
    pub fn update_with_buffer(
        &mut self,
        buffer: &[u32],
        width: usize,
        height: usize,
    ) -> MiniResult<()> {
        self.window.update_with_buffer(buffer, width, height)
    }
}

fn from_u8_rgb(r: u8, g: u8, b: u8) -> u32 {
    let (r, g, b) = (r as u32, g as u32, b as u32);
    (r << 16) | (g << 8) | b
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
