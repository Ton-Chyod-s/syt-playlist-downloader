use std::sync::{Arc, Mutex};
use tauri::{AppHandle, Emitter, State};

use crate::types::{DownloadState, ProgressEvent};
use crate::validation::{validate_cookies_path, validate_output_dir, validate_spotify_id};
use crate::process::{emit, spawn_download};
use crate::spotify::{
    get_playlist_tracks, get_playlist_tracks_via_embed,
    get_spotify_token, get_token_from_sp_dc,
};

fn download_spotify(
    app: AppHandle,
    url: String,
    output_dir: String,
    playlist_end: Option<u32>,
    client_id: String,
    client_secret: String,
    sp_dc: Option<String>,
    pid_state: Arc<Mutex<Option<u32>>>,
    cancelled: Arc<Mutex<bool>>,
) -> Result<(), String> {
    let playlist_id = url
        .split('/')
        .last()
        .and_then(|s| s.split('?').next())
        .filter(|s| !s.is_empty())
        .ok_or("URL da playlist inválida")?
        .to_string();
    let playlist_id = validate_spotify_id(&playlist_id)?;

    emit(&app, "🎵 Buscando músicas da playlist do Spotify...");

    let mut tracks = match get_playlist_tracks_via_embed(&playlist_id) {
        Ok(t) if !t.is_empty() => t,
        embed_result => {
            match &embed_result {
                Err(e) => emit(&app, &format!("⚠️ Embed: {}. Tentando API...", e)),
                Ok(_)  => emit(&app, "⚠️ Embed retornou lista vazia. Tentando API..."),
            }

            let token = if let Some(ref dc) = sp_dc.filter(|s| !s.is_empty()) {
                emit(&app, "🍪 Autenticando via cookie sp_dc...");
                get_token_from_sp_dc(dc)?
            } else if !client_id.is_empty() && !client_secret.is_empty() {
                get_spotify_token(&client_id, &client_secret)?
            } else {
                return Err(
                    "Não foi possível ler a playlist automaticamente. \
                    Forneça o cookie sp_dc OU o Client ID/Secret do Spotify para autenticar."
                    .into(),
                );
            };
            get_playlist_tracks(&token, &playlist_id)?
        }
    };

    if tracks.is_empty() {
        return Err("Nenhuma música encontrada na playlist.".into());
    }

    if let Some(end) = playlist_end {
        let limit = (end as usize).min(10_000);
        if limit > 0 {
            tracks.truncate(limit);
        }
    }

    emit(&app, &format!("📋 {} músicas encontradas. Baixando do YouTube Music...", tracks.len()));

    let queries: Vec<String> = tracks
        .iter()
        .map(|t| format!("ytsearch1:{}", t))
        .collect();

    let batch_name = format!(
        "syt_batch_{}_{}.txt",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis()
    );
    let batch_path = std::env::temp_dir().join(batch_name);
    std::fs::write(&batch_path, queries.join("\n"))
        .map_err(|e| format!("Erro ao criar arquivo temporário: {}", e))?;

    let output_template = format!("{}\\%(title)s.%(ext)s", output_dir);
    let args = vec![
        "--js-runtimes".into(), "node".into(),
        "--default-search".into(), "ytsearch".into(),
        "-f".into(), "bestaudio[ext=webm]/bestaudio/bestaudio[ext=m4a]/bestaudio".into(),
        "--extract-audio".into(),
        "--audio-format".into(), "mp3".into(),
        "--audio-quality".into(), "0".into(),
        "--ignore-errors".into(),
        "-N".into(), "4".into(),
        "--newline".into(),
        "-o".into(), output_template,
        "--batch-file".into(), batch_path.to_string_lossy().into_owned(),
    ];

    spawn_download("yt-dlp", args, vec![], app, pid_state, cancelled, vec![batch_path])
}

#[tauri::command]
pub fn download_playlist(
    app: AppHandle,
    url: String,
    output_dir: String,
    cookies_path: Option<String>,
    playlist_end: Option<u32>,
    client_id: Option<String>,
    client_secret: Option<String>,
    sp_dc: Option<String>,
    mode: Option<String>,
    state: State<DownloadState>,
) -> Result<(), String> {
    let pid_arc = state.pid.clone();
    let cancelled_arc = state.cancelled.clone();
    *cancelled_arc.lock().unwrap() = false;

    let output_dir = validate_output_dir(&output_dir)?;

    if let Some(ref c) = cookies_path {
        if !c.is_empty() {
            validate_cookies_path(c)?;
        }
    }

    if url.contains("spotify.com") {
        let sp_dc_val = sp_dc.filter(|s| !s.is_empty());

        let (id, secret) = if sp_dc_val.is_none() {
            let id = client_id.filter(|s| !s.is_empty())
                .ok_or_else(|| "Forneça o cookie sp_dc OU o Client ID do Spotify.".to_string())?;
            let secret = client_secret.filter(|s| !s.is_empty())
                .ok_or_else(|| "Forneça o cookie sp_dc OU o Client Secret do Spotify.".to_string())?;
            (id, secret)
        } else {
            (String::new(), String::new())
        };

        let app_bg = app.clone();
        std::thread::spawn(move || {
            if let Err(e) = download_spotify(
                app_bg.clone(), url, output_dir, playlist_end,
                id, secret, sp_dc_val, pid_arc, cancelled_arc,
            ) {
                let _ = app_bg.emit("download-progress", ProgressEvent {
                    msg: String::new(),
                    done: false,
                    error: Some(e),
                });
            }
        });
        Ok(())
    } else if mode.as_deref() == Some("original") {
        let output_template = format!(
            "{}\\%(playlist_index)s%(playlist_index&. - |)s%(title)s.%(ext)s",
            output_dir
        );
        let mut args: Vec<String> = vec![
            "--js-runtimes".into(), "node".into(),
            "-f".into(), "bestvideo+bestaudio/best".into(),
            "--merge-output-format".into(), "mp4".into(),
            "-N".into(), "4".into(),
            "--concurrent-fragments".into(), "4".into(),
            "--newline".into(),
            "-o".into(), output_template,
        ];

        if let Some(ref c) = cookies_path {
            if !c.is_empty() {
                args.push("--cookies".into());
                args.push(c.clone());
            }
        }

        if let Some(end) = playlist_end {
            args.push("--playlist-end".into());
            args.push(end.to_string());
        }

        args.push(url);
        spawn_download("yt-dlp", args, vec![], app, pid_arc, cancelled_arc, vec![])
    } else {
        let output_template = format!("{}\\%(playlist_index)s - %(title)s.%(ext)s", output_dir);
        let mut args: Vec<String> = vec![
            "--js-runtimes".into(), "node".into(),
            "-f".into(),
            "bestvideo[vcodec^=avc][height<=1080][ext=mp4]+bestaudio/bestvideo[vcodec^=avc][height<=1080]+bestaudio/best[height<=1080]".into(),
            "--merge-output-format".into(), "mp4".into(),
            "--postprocessor-args".into(), "ffmpeg:-c:v copy -c:a libmp3lame -q:a 2".into(),
            "-N".into(), "4".into(),
            "--concurrent-fragments".into(), "4".into(),
            "--newline".into(),
            "-o".into(), output_template,
        ];

        if let Some(ref c) = cookies_path {
            if !c.is_empty() {
                args.push("--cookies".into());
                args.push(c.clone());
            }
        }

        if let Some(end) = playlist_end {
            args.push("--playlist-end".into());
            args.push(end.to_string());
        }

        args.push(url);
        spawn_download("yt-dlp", args, vec![], app, pid_arc, cancelled_arc, vec![])
    }
}
