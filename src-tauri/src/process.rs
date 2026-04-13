use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio};
use std::sync::{Arc, Mutex};
use tauri::{AppHandle, Emitter};

#[cfg(windows)]
use std::os::windows::process::CommandExt;

#[cfg(windows)]
pub const CREATE_NO_WINDOW: u32 = 0x08000000;

use crate::types::ProgressEvent;

pub fn emit(app: &AppHandle, msg: &str) {
    let _ = app.emit("download-progress", ProgressEvent {
        msg: msg.to_string(),
        done: false,
        error: None,
    });
}

fn is_relevant(line: &str) -> bool {
    !line.is_empty()
}

pub fn make_cmd(name: &str) -> Command {
    #[cfg(windows)]
    {
        let mut c = Command::new(name);
        c.creation_flags(CREATE_NO_WINDOW);
        c
    }
    #[cfg(not(windows))]
    {
        Command::new(name)
    }
}

pub fn spawn_download(
    cmd_name: &str,
    args: Vec<String>,
    envs: Vec<(&'static str, &'static str)>,
    app: AppHandle,
    pid_state: Arc<Mutex<Option<u32>>>,
    cancelled: Arc<Mutex<bool>>,
    cleanup_files: Vec<std::path::PathBuf>,
) -> Result<(), String> {
    let mut cmd = make_cmd(cmd_name);
    cmd.args(&args);
    for (k, v) in envs {
        cmd.env(k, v);
    }
    let mut child = cmd
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| format!("Falha ao iniciar {}: {}. Verifique se está instalado e no PATH.", cmd_name, e))?;

    *pid_state.lock().unwrap() = Some(child.id());

    let stdout = child.stdout.take()
        .ok_or("Falha ao capturar stdout do processo.".to_string())?;
    let stderr = child.stderr.take()
        .ok_or("Falha ao capturar stderr do processo.".to_string())?;

    let app_stdout = app.clone();
    std::thread::spawn(move || {
        let reader = BufReader::new(stdout);
        for line in reader.lines().flatten() {
            if is_relevant(&line) {
                let _ = app_stdout.emit("download-progress", ProgressEvent {
                    msg: line,
                    done: false,
                    error: None,
                });
            }
        }
    });

    let app_stderr = app.clone();
    std::thread::spawn(move || {
        let reader = BufReader::new(stderr);
        for line in reader.lines().flatten() {
            if is_relevant(&line) {
                let _ = app_stderr.emit("download-progress", ProgressEvent {
                    msg: line,
                    done: false,
                    error: None,
                });
            }
        }
    });

    let pid_state_wait = pid_state.clone();
    std::thread::spawn(move || {
        match child.wait() {
            Ok(status) if status.success() => {
                *pid_state_wait.lock().unwrap() = None;
                for f in &cleanup_files { let _ = std::fs::remove_file(f); }
                if *cancelled.lock().unwrap() {
                    return;
                }
                let _ = app.emit("download-progress", ProgressEvent {
                    msg: String::new(),
                    done: true,
                    error: None,
                });
            }
            Ok(status) => {
                *pid_state_wait.lock().unwrap() = None;
                for f in &cleanup_files { let _ = std::fs::remove_file(f); }
                if *cancelled.lock().unwrap() {
                    return;
                }
                let code = status.code().unwrap_or(-1);
                let msg = match code {
                    1 => "Falha no download. Verifique se a URL é válida e se sua conexão está ok.".into(),
                    2 => "Uso incorreto do comando. Tente atualizar o yt-dlp.".into(),
                    _ => format!("O processo encerrou inesperadamente (código {}). Tente novamente.", code),
                };
                let _ = app.emit("download-progress", ProgressEvent {
                    msg: String::new(),
                    done: false,
                    error: Some(msg),
                });
            }
            Err(e) => {
                *pid_state_wait.lock().unwrap() = None;
                for f in &cleanup_files { let _ = std::fs::remove_file(f); }
                if *cancelled.lock().unwrap() {
                    return;
                }
                let _ = app.emit("download-progress", ProgressEvent {
                    msg: String::new(),
                    done: false,
                    error: Some(format!("Erro ao aguardar processo: {}", e)),
                });
            }
        }
    });

    Ok(())
}
