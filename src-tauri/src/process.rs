use serde::Serialize;
use sysinfo::{ProcessRefreshKind, ProcessesToUpdate, System};

#[derive(Serialize)]
pub struct ProcessInfo {
    pid: u32,
    name: String,
    cpu_usage: f32,
    memory_bytes: u64,
    disk_usage: u64,
}

#[tauri::command]
pub fn get_processes() -> Vec<ProcessInfo> {
    let mut system = System::new_all();

    std::thread::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL);

    system.refresh_processes_specifics(
        ProcessesToUpdate::All,
        true,
        ProcessRefreshKind::everything()
    );
    
    system
        .processes()
        .iter()
        .map(|(&pid, process)| {
            ProcessInfo {
                pid: pid.as_u32(),
                name: process.name().to_str().unwrap().to_string(),
                cpu_usage: process.cpu_usage(),
                memory_bytes: process.memory(),
                disk_usage: process.disk_usage().read_bytes + process.disk_usage().written_bytes,
            }
        })
        .collect()
}

