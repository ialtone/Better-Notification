use tauri::{LogicalSize, Window};

pub fn adjust_window_size(window: &Window, str_arg: &str) {
    let str_len = str_arg.chars().count();
    let base_width = 300.0;
    let base_height = 100.0;
    let char_width = 10.0;
    let max_width = 500.0;
    let chars_per_line = (max_width / char_width) as usize;
    let lines = (str_len as f64 / chars_per_line as f64).ceil() as f64;
    let new_width = (base_width + (str_len as f64 * char_width)).min(max_width);
    let new_height = base_height + (lines * 20.0);
    window
        .set_size(LogicalSize::new(new_width, new_height))
        .unwrap();
}

pub fn get_screen_size(window: &Window) -> (f64, f64) {
    let monitor = window.current_monitor().unwrap().unwrap();
    let screen_size = monitor.size();
    (screen_size.width as f64, screen_size.height as f64)
}
