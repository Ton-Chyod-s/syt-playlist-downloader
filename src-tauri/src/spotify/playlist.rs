pub fn spotify_get(url: &str, token: &str) -> Result<serde_json::Value, String> {
    ureq::get(url)
        .set("Authorization", &format!("Bearer {}", token))
        .call()
        .map_err(|e| match e {
            ureq::Error::Status(code, _) => format!("Erro na API Spotify (HTTP {}).", code),
            _ => "Erro de rede ao acessar Spotify.".into(),
        })?
        .into_json::<serde_json::Value>()
        .map_err(|_| "Erro ao processar resposta da API Spotify.".into())
}

pub fn parse_tracks_from_items(items: &[serde_json::Value], tracks: &mut Vec<String>) {
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

pub fn get_playlist_tracks(token: &str, playlist_id: &str) -> Result<Vec<String>, String> {
    let mut tracks: Vec<String> = Vec::new();

    let url = format!("https://api.spotify.com/v1/playlists/{}", playlist_id);
    let json = spotify_get(&url, token)?;

    if let Some(items) = json["tracks"]["items"].as_array() {
        parse_tracks_from_items(items, &mut tracks);
    }

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

pub fn get_playlist_tracks_via_embed(playlist_id: &str) -> Result<Vec<String>, String> {
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
            ureq::Error::Status(code, _) => format!("Erro ao acessar playlist (HTTP {}).", code),
            _ => "Erro de rede ao acessar playlist.".into(),
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
