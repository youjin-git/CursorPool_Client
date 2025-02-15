use sysinfo::{System, RefreshKind, ProcessRefreshKind};

pub fn kill_cursor_process() {
    let mut sys = System::new_with_specifics(RefreshKind::new().with_processes(ProcessRefreshKind::everything()));
    sys.refresh_processes();

    for (pid, process) in sys.processes() {
        if process.name().to_lowercase().contains("cursor") {
            #[cfg(target_os = "windows")]
            {
                use windows::Win32::System::Threading::{OpenProcess, TerminateProcess, PROCESS_TERMINATE};
                use windows::Win32::Foundation::CloseHandle;
                unsafe {
                    if let Ok(handle) = OpenProcess(PROCESS_TERMINATE, false, pid.as_u32()) {
                        let _ = TerminateProcess(handle, 0);
                        let _ = CloseHandle(handle);
                    }
                }
            }
            #[cfg(not(target_os = "windows"))]
            {
                process.kill();
            }
        }
    }
}
