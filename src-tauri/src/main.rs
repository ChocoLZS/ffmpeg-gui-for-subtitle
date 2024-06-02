// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::process::{Command, Output};

#[tauri::command]
fn greet(name: &str) -> String {
   // 调用一个本地命令，例如 "ls" 或 "echo"
    let output = Command::new("ffmpeg")
        .arg("--help")
        .output();
    match output {
      Ok(output) => {
         if output.status.success() {
        // 将输出转换为字符串
        let stdout = String::from_utf8_lossy(&output.stdout);
        return format!("Command succeeded with output:\n{}", stdout);
    } else {
        // 将错误信息转换为字符串
        let stderr = String::from_utf8_lossy(&output.stderr);
        return format!("Command failed with error:\n{}", stderr);
    }
      },
      Err(e) => {
        println!("Error: {}", e);
        return format!("Error: {}", e);
      }
    }
    // 检查命令是否成功执行
   
}

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![greet])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}