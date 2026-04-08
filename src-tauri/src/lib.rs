use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio};
use std::sync::{Arc, Mutex};
use base64::{Engine as _, engine::general_purpose::STANDARD as B64};
use tauri::{AppHandle, Emitter, State};

#[cfg(windows)]
use std::os::windows::process::CommandExt;

#[cfg(windows)]
const CREATE_NO_WINDOW: u32 = 0x08000000;

#[derive(serde::Serialize, Clone)]
struct ProgressEvent {
    msg: String,
    done: bool,
    error: Option<String>,
}

struct DownloadState {
    pid: Arc<Mutex<Option<u32>>>,
    cancelled: Arc<Mutex<bool>>,
}

fn emit(app: &AppHandle, msg: &str) {
    let _ = app.emit("download-progress", ProgressEvent {
        msg: msg.to_string(),
        done: false,
        error: None,
    });
}

fn is_relevant(line: &str) -> bool {
    !line.is_empty()
}

fn make_cmd(name: &str) -> Command {
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

fn get_token_from_sp_dc(sp_dc: &str) -> Result<String, String> {
    let cookie = format!("sp_dc={}", sp_dc);

    let resp = ureq::get("https://open.spotify.com/get_access_token?reason=transport&productType=web_player")
        .set("Cookie", &cookie)
        .set("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/124.0.0.0 Safari/537.36")
        .set("Referer", "https://open.spotify.com/")
        .call()
        .map_err(|e| match e {
            ureq::Error::Status(code, resp) => {
                let body = resp.into_string().unwrap_or_default();
                format!("Erro ao obter token via sp_dc ({}): {}", code, body)
            }
            other => format!("Erro de rede ao obter token sp_dc: {}", other),
        })?;

    let json: serde_json::Value = resp
        .into_json()
        .map_err(|e| format!("Erro ao ler resposta do token sp_dc: {}", e))?;

    if json["isAnonymous"].as_bool().unwrap_or(true) {
        return Err(
            "Cookie sp_dc inválido ou expirado. \
            Abra open.spotify.com, faça login, pressione F12 → Application → Cookies → open.spotify.com, \
            copie o valor do cookie 'sp_dc' e cole aqui."
            .into(),
        );
    }

    json["accessToken"]
        .as_str()
        .map(|s| s.to_string())
        .ok_or_else(|| format!("Campo 'accessToken' não encontrado: {}", json))
}

fn get_spotify_token(client_id: &str, client_secret: &str) -> Result<String, String> {
    let creds = B64.encode(format!("{}:{}", client_id, client_secret));

    let resp = ureq::post("https://accounts.spotify.com/api/token")
        .set("Authorization", &format!("Basic {}", creds))
        .send_form(&[("grant_type", "client_credentials")])
        .map_err(|e| format!("Erro ao autenticar no Spotify: {}", e))?;

    let json: serde_json::Value = resp
        .into_json()
        .map_err(|e| format!("Erro ao ler token: {}", e))?;

    json["access_token"]
        .as_str()
        .map(|s| s.to_string())
        .ok_or_else(|| format!("Token não encontrado: {}", json))
}

fn spotify_get(url: &str, token: &str) -> Result<serde_json::Value, String> {
    ureq::get(url)
        .set("Authorization", &format!("Bearer {}", token))
        .call()
        .map_err(|e| match e {
            ureq::Error::Status(code, resp) => {
                let body = resp.into_string().unwrap_or_default();
                format!("Spotify API erro {}: {}", code, body)
            }
            other => format!("Erro de rede: {}", other),
        })?
        .into_json::<serde_json::Value>()
        .map_err(|e| format!("Erro ao ler resposta: {}", e))
}

fn parse_tracks_from_items(items: &[serde_json::Value], tracks: &mut Vec<String>) {
    for item in items {
        let track = &item["track"];
        if track.is_null() { continue; }
        if let (Some(name), Some(artists)) = (
            track["name"].as_str(),
            track["artists"].as_array(),
        ) {
            let artist = artists.first()
                .and_then(|a| a["name"].as_str())
                .unwrap_or("Unknown");
            tracks.push(format!("{} - {}", artist, name));
        }
    }
}

fn get_playlist_tracks(token: &str, playlist_id: &str) -> Result<Vec<String>, String> {
    let mut tracks: Vec<String> = Vec::new();

    // Endpoint base da playlist (retorna as primeiras faixas embutidas)
    let url = format!("https://api.spotify.com/v1/playlists/{}", playlist_id);
    let json = spotify_get(&url, token)?;

    if let Some(items) = json["tracks"]["items"].as_array() {
        parse_tracks_from_items(items, &mut tracks);
    }

    // Paginar se houver mais faixas
    let mut next_url = json["tracks"]["next"].as_str().map(|s| s.to_string());
    while let Some(url) = next_url {
        let page = spotify_get(&url, token)?;
        if let Some(items) = page["items"].as_array() {
            parse_tracks_from_items(items, &mut tracks);
        }
        next_url = page["next"].as_str().map(|s| s.to_string());
    }

    Ok(tracks)
}

fn get_playlist_tracks_via_embed(playlist_id: &str) -> Result<Vec<String>, String> {
    let url = format!(
        "https://open.spotify.com/embed/playlist/{}?utm_source=generator",
        playlist_id
    );

    let html = ureq::get(&url)
        .set("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/124.0.0.0 Safari/537.36")
        .set("Accept", "text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,*/*;q=0.8")
        .set("Accept-Language", "en-US,en;q=0.9")
        .call()
        .map_err(|e| match e {
            ureq::Error::Status(code, resp) => {
                let body = resp.into_string().unwrap_or_default();
                format!("Embed HTTP {}: {:.120}", code, body)
            }
            other => format!("Embed rede: {}", other),
        })?
        .into_string()
        .map_err(|e| format!("Embed leitura: {}", e))?;

    let marker = r#"<script id="__NEXT_DATA__" type="application/json">"#;
    let start = html
        .find(marker)
        .ok_or("__NEXT_DATA__ não encontrado no embed. A playlist pode ser privada.")?;
    let json_start = start + marker.len();
    let remaining = &html[json_start..];
    let json_end = remaining
        .find("</script>")
        .ok_or("Fim do __NEXT_DATA__ não encontrado.")?;
    let json_str = &remaining[..json_end];

    let data: serde_json::Value = serde_json::from_str(json_str)
        .map_err(|e| format!("Erro ao analisar __NEXT_DATA__: {}", e))?;

    let entity = &data["props"]["pageProps"]["state"]["data"]["entity"];

    if let Some(track_list) = entity["trackList"].as_array() {
        if !track_list.is_empty() {
            let tracks: Vec<String> = track_list
                .iter()
                .filter_map(|t| {
                    let title = t["title"].as_str()?;
                    let subtitle = t["subtitle"].as_str().unwrap_or("");
                    let artist = subtitle
                        .split('\u{00B7}')
                        .next()
                        .unwrap_or(subtitle)
                        .trim();
                    Some(if artist.is_empty() {
                        title.to_string()
                    } else {
                        format!("{} - {}", artist, title)
                    })
                })
                .collect();
            if !tracks.is_empty() {
                return Ok(tracks);
            }
        }
    }

    if let Some(items) = entity["tracks"]["items"].as_array() {
        let mut tracks = Vec::new();
        parse_tracks_from_items(items, &mut tracks);
        if !tracks.is_empty() {
            return Ok(tracks);
        }
    }

    Err("Lista de músicas não encontrada no embed. Tente fornecer o cookie sp_dc ou Client ID/Secret.".into())
}

fn spawn_download(
    cmd_name: &str,
    args: Vec<String>,
    envs: Vec<(&'static str, &'static str)>,
    app: AppHandle,
    pid_state: Arc<Mutex<Option<u32>>>,
    cancelled: Arc<Mutex<bool>>,
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

    let stdout = child.stdout.take().unwrap();
    let stderr = child.stderr.take().unwrap();

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
                let _ = app.emit("download-progress", ProgressEvent {
                    msg: String::new(),
                    done: true,
                    error: None,
                });
            }
            Ok(status) => {
                *pid_state_wait.lock().unwrap() = None;
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
    // Extrai o ID da playlist da URL
    let playlist_id = url
        .split('/')
        .last()
        .and_then(|s| s.split('?').next())
        .filter(|s| !s.is_empty())
        .ok_or("URL da playlist inválida")?
        .to_string();

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
        tracks.truncate(end as usize);
    }

    emit(&app, &format!("📋 {} músicas encontradas. Baixando do YouTube Music...", tracks.len()));

    let queries: Vec<String> = tracks
        .iter()
        .map(|t| format!("ytsearch1:{}", t))
        .collect();

    let batch_path = std::env::temp_dir().join("yt_spotify_batch.txt");
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

    spawn_download("yt-dlp", args, vec![], app, pid_state, cancelled)
}

#[tauri::command]
fn download_playlist(
    app: AppHandle,
    url: String,
    output_dir: String,
    cookies_path: Option<String>,
    playlist_end: Option<u32>,
    client_id: Option<String>,
    client_secret: Option<String>,
    sp_dc: Option<String>,
    mode: Option<String>,   // "playlist" | "video"
    state: State<DownloadState>,
) -> Result<(), String> {
    let pid_arc = state.pid.clone();
    let cancelled_arc = state.cancelled.clone();
    *cancelled_arc.lock().unwrap() = false;

    if url.contains("spotify.com") {
        let sp_dc_val = sp_dc.filter(|s| !s.is_empty());

        // Se sp_dc não foi fornecido, exige client_id + client_secret
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
            if let Err(e) = download_spotify(app_bg.clone(), url, output_dir, playlist_end, id, secret, sp_dc_val, pid_arc, cancelled_arc) {
                let _ = app_bg.emit("download-progress", ProgressEvent {
                    msg: String::new(),
                    done: false,
                    error: Some(e),
                });
            }
        });
        Ok(())
    } else if mode.as_deref() == Some("original") {
        let output_template = format!("{}\\%(playlist_index)s%(playlist_index&. - |)s%(title)s.%(ext)s", output_dir);
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
        spawn_download("yt-dlp", args, vec![], app, pid_arc, cancelled_arc)
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
        spawn_download("yt-dlp", args, vec![], app, pid_arc, cancelled_arc)
    }
}

#[tauri::command]
fn cancel_download(state: State<DownloadState>) -> Result<(), String> {
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

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(DownloadState {
            pid: Arc::new(Mutex::new(None)),
            cancelled: Arc::new(Mutex::new(false)),
        })
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![download_playlist, cancel_download])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
