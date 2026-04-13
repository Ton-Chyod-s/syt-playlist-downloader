use std::process::Command;
use tauri::State;

use crate::types::DownloadState;

#[cfg(windows)]
use crate::process::CREATE_NO_WINDOW;
#[cfg(windows)]
use std::os::windows::process::CommandExt;

#[tauri::command]
pub fn cancel_download(state: State<DownloadState>) -> Result<(), String> {
    *state.cancelled.lock().unwrap() = true;

    let mut pid_lock = state.pid.lock().unwrap();
    if let Some(pid) = *pid_lock {
        #[cfg(windows)]
        {
            Command::new("taskkill")
                .args(["/F", "/T", "/PID", &pid.to_string()])
                .creation_flags(CREATE_NO_WINDOW)
                .spawn()
                .map_err(|e| format!("Falha ao cancelar: {}", e))?;
        }
        #[cfg(not(windows))]
        {
            Command::new("kill")
                .args(["-9", &pid.to_string()])
                .spawn()
                .map_err(|e| format!("Falha ao cancelar: {}", e))?;
        }
        *pid_lock = None;
    }
    Ok(())
}
