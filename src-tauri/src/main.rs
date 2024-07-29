// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod utils;

use commands::{close_window, get_route_arg, get_str_arg};
use std::env;
use std::thread;
use std::time::Duration;
use tauri::{Manager, PhysicalPosition};
use utils::{adjust_window_size, get_screen_size, parse_close_delay, parse_position};

fn main() {
    // 获取命令行参数
    let args: Vec<String> = env::args().collect();
    let (x_pos, y_pos) = parse_position(&args);
    let close_delay = parse_close_delay(&args);

    tauri::Builder::default()
        .setup(move |app| {
            let window = app.get_window("main").unwrap(); // 替换成你的窗口标签

            // 获取屏幕尺寸
            let (screen_width, screen_height) = get_screen_size(&window);

            // 获取字符串参数
            let str_arg = get_str_arg();

            // 动态调整窗口大小
            adjust_window_size(&window, &str_arg);

            // 获取窗口尺寸
            let window_size = window.outer_size().unwrap();
            let window_width = window_size.width as f64;
            let window_height = window_size.height as f64;

            // 计算默认位置：屏幕中间偏上四分之一处
            let default_x = (screen_width - window_width) / 2.0;
            let default_y = (screen_height - window_height) / 4.0;

            // 使用命令行参数或默认值设置窗口位置
            let final_x = x_pos.unwrap_or(default_x);
            let final_y = y_pos.unwrap_or(default_y);

            window
                .set_position(PhysicalPosition::new(final_x, final_y))
                .unwrap(); // 设置窗口位置

            // 处理关闭延迟
            if let Some(delay) = close_delay {
                let window_clone = window.clone();
                thread::spawn(move || {
                    thread::sleep(Duration::from_secs(delay));
                    window_clone.close().unwrap();
                });
            }

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            close_window,
            get_str_arg,
            get_route_arg
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
