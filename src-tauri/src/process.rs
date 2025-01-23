use serde::Serialize;
use sysinfo::System;

#[derive(Serialize)]
pub struct ProcessInfo {
    pid: u32,
    name: String,
    cpu_usage: f32,
    memory_bytes: u64,
}

#[tauri::command]
pub fn get_processes() -> Vec<ProcessInfo> {
    let mut system = System::new_all();
    system.refresh_all();

    system
        .processes()
        .iter()
        .map(|(&pid, process)| {
            ProcessInfo {
                pid: pid.as_u32(),
                name: process.name().to_str().unwrap().to_string(),
                cpu_usage: process.cpu_usage(),
                memory_bytes: process.memory()
            }
        })
        .collect()
}

