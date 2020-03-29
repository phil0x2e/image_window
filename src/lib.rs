extern crate minifb;
use core::ffi::c_void;
use core::time::Duration;
use minifb::{
    CursorStyle, InputCallback, Key, KeyRepeat, Menu, MenuHandle, MouseButton, MouseMode, UnixMenu,
    Window, WindowOptions,
};

pub struct ImageWindow {
    window: Window,
    buffer: Vec<u32>,
}

impl ImageWindow {
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
    pub fn update(&mut self) {
        self.window.update();
    }
    pub fn update_with_buffer(
        &mut self,
        buffer: &[u32],
        width: usize,
        height: usize,
    ) -> Result<()> {
        self.update_with_buffer(buffer, width, height)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
